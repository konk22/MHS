"use client"

import React from "react"
import { Button } from "@/components/ui/button"
import { useEffect, useState } from "react"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Checkbox } from "@/components/ui/checkbox"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Progress } from "@/components/ui/progress"
import {
  Wifi,
  WifiOff,
  RefreshCw,
  Settings,
  Activity,
  Clock,
  Terminal,
  Camera,
  Trash2,
  ChevronDown,
  ChevronRight,
  Play,
  Pause,
  Square,
  AlertTriangle,
  Plus,
  Network,
  Sun,
  Moon,
  Monitor,
  ExternalLink,
  RotateCw,
} from "lucide-react"
import { useTranslation } from "@/lib/i18n"

interface Subnet {
  id: string
  range: string
  name: string
  enabled: boolean
}

interface HostInfo {
  id: string
  hostname: string
  ip_address: string
  subnet: string
  status: "online" | "offline"
  device_status: string
  moonraker_version?: string
  klippy_state?: string
  printer_state?: string
  last_seen?: string
}

interface MoonrakerServerInfo {
  result: {
    klippy_connected: boolean
    klippy_state: string
    components: string[]
    failed_components: string[]
    registered_directories: string[]
    warnings: string[]
    websocket_count: number
    moonraker_version: string
    api_version: number[]
    api_versions: Record<string, number[]>
  }
}

interface MoonrakerPrinterInfo {
  result: {
    state: string
    state_message: string
    hostname?: string
    software_version?: string
    cpu_info?: string
    klipper_path?: string
    python_path?: string
    log_file?: string
    config_file?: string
  }
}

interface AppSettings {
  subnets: Subnet[]
  defaultSSHUser: string
  notifications: {
    printing: boolean
    paused: boolean
    error: boolean
    ready: boolean
    standby: boolean
    offline: boolean
  }
  theme: "light" | "dark" | "system"
  autoRefresh: boolean
  refreshInterval: number
  language: string
}

export function NetworkScanner() {
  const [settings, setSettings] = useState<AppSettings>({
    subnets: [
      {
        id: "1",
        range: "192.168.1.0/24",
        name: "Main Network", // Will be translated dynamically
        enabled: true,
      },
      {
        id: "2",
        range: "192.168.0.0/24",
        name: "Guest Network", // Will be translated dynamically
        enabled: false,
      },
    ],
    defaultSSHUser: "admin",
    notifications: {
      printing: true,
      paused: true,
      error: true,
      ready: false,
      standby: false,
      offline: true,
    },
    theme: "system",
    autoRefresh: false,
    refreshInterval: 30,
    language: "en", // Added default language
  })

  const [onlineHosts, setOnlineHosts] = useState(0)
  const [isScanning, setIsScanning] = useState(false)
  const [scanProgress, setScanProgress] = useState(0)
  const [scanStatus, setScanStatus] = useState("")
  const [hosts, setHosts] = useState<HostInfo[]>([])
  const [webcamDialog, setWebcamDialog] = useState<{ open: boolean; host: HostInfo | null }>({
    open: false,
    host: null,
  })
  const [currentWebcamUrlIndex, setCurrentWebcamUrlIndex] = useState(0)
  const [webcamRotation, setWebcamRotation] = useState(0)
  const [webcamFlip, setWebcamFlip] = useState({ horizontal: false, vertical: false })
  const [expandedRows, setExpandedRows] = useState(new Set())

  const t = useTranslation(settings.language)

  // Tauri API functions
  const invokeTauri = async (command: string, args?: any) => {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      return await (window as any).__TAURI__.core.invoke(command, args)
    }
    throw new Error('Tauri API not available')
  }

  // Load settings from localStorage
  useEffect(() => {
    const savedSettings = localStorage.getItem('networkScanner_settings')
    if (savedSettings) {
      try {
        const parsed = JSON.parse(savedSettings)
        setSettings(prev => ({ ...prev, ...parsed }))
      } catch (error) {
        console.error('Failed to parse saved settings:', error)
      }
    }
  }, [])

  // Save settings to localStorage
  useEffect(() => {
    localStorage.setItem('networkScanner_settings', JSON.stringify(settings))
  }, [settings])

  // Load hosts from localStorage
  useEffect(() => {
    const savedHosts = localStorage.getItem('networkScanner_hosts')
    if (savedHosts) {
      try {
        const parsed = JSON.parse(savedHosts)
        setHosts(parsed)
        setOnlineHosts(parsed.filter((h: HostInfo) => h.status === 'online').length)
      } catch (error) {
        console.error('Failed to parse saved hosts:', error)
      }
    }
  }, [])

  // Save hosts to localStorage
  useEffect(() => {
    localStorage.setItem('networkScanner_hosts', JSON.stringify(hosts))
  }, [hosts])

  useEffect(() => {
    const savedTheme = localStorage.getItem("networkScanner_theme")
    if (savedTheme && ["light", "dark", "system"].includes(savedTheme)) {
      setSettings((prev) => ({ ...prev, theme: savedTheme as "light" | "dark" | "system" }))
    }
  }, [])

  useEffect(() => {
    const applyTheme = (theme: "light" | "dark" | "system") => {
      const root = document.documentElement

      if (theme === "system") {
        const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
        root.classList.toggle("dark", systemTheme === "dark")
      } else {
        root.classList.toggle("dark", theme === "dark")
      }
    }

    applyTheme(settings.theme)
    localStorage.setItem("networkScanner_theme", settings.theme)

    // Listen for system theme changes
    if (settings.theme === "system") {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
      const handleChange = () => applyTheme("system")
      mediaQuery.addEventListener("change", handleChange)
      return () => mediaQuery.removeEventListener("change", handleChange)
    }
  }, [settings.theme])

  useEffect(() => {
    const savedLanguage = localStorage.getItem("networkScanner_language")
    if (savedLanguage && ["en", "ru"].includes(savedLanguage)) {
      setSettings((prev) => ({ ...prev, language: savedLanguage }))
    }
  }, [])

  useEffect(() => {
    localStorage.setItem("networkScanner_language", settings.language)
  }, [settings.language])

  const addSubnet = () => {
    const newSubnet: Subnet = {
      id: Date.now().toString(),
      range: "192.168.2.0/24",
      name: t.newNetwork, // Use translation
      enabled: false,
    }
    setSettings((prev) => ({
      ...prev,
      subnets: [...prev.subnets, newSubnet],
    }))
  }

  const handleScan = async () => {
    setIsScanning(true)
    setScanProgress(0)
    setScanStatus(t.scanningPorts || "Scanning ports...")
    
    try {
      const enabledSubnets = settings.subnets.filter(s => s.enabled)
      if (enabledSubnets.length === 0) {
        alert(t.noSubnetsEnabled || 'No subnets enabled for scanning')
        setIsScanning(false)
        return
      }

      // Показываем прогресс сканирования портов
      setScanProgress(10)
      
      // Имитируем прогресс сканирования портов
      const progressInterval = setInterval(() => {
        setScanProgress(prev => {
          if (prev < 40) return prev + 5
          return prev
        })
      }, 100)
      
      const result = await invokeTauri('scan_network', { subnets: enabledSubnets })
      
      clearInterval(progressInterval)
      setScanProgress(80)
      setScanStatus(t.scanningAPI || "Checking API...")
      
      console.log('Scan result:', result)
      
      if (result.hosts) {
        console.log('Found hosts:', result.hosts)
        setHosts(result.hosts)
        setOnlineHosts(result.online_hosts || 0)
      }
      
      setScanProgress(100)
      setScanStatus("")
    } catch (error) {
      console.error('Scan failed:', error)
      alert('Scan failed: ' + (error as Error).message)
    } finally {
      setIsScanning(false)
      setScanStatus("")
    }
  }

  const toggleRowExpansion = (hostId: string) => {
    setExpandedRows((prev) => {
      const newRows = new Set(prev)
      if (newRows.has(hostId)) {
        newRows.delete(hostId)
      } else {
        newRows.add(hostId)
      }
      return newRows
    })
  }

  const getStatusIcon = (status: string) => {
    if (status === "online") {
      return <Wifi className="h-4 w-4 text-green-600" />
    } else {
      return <WifiOff className="h-4 w-4 text-red-600" />
    }
  }

  const handleAPIAction = async (action: string, hostId: string) => {
    const host = hosts.find(h => h.id === hostId)
    if (!host) return

    try {
      let command = ''
      switch (action) {
        case 'Start':
          command = 'control_printer_command'
          break
        case 'Pause':
          command = 'control_printer_command'
          break
        case 'Stop':
          command = 'control_printer_command'
          break
        case 'Emergency Stop':
          command = 'control_printer_command'
          break
        default:
          return
      }

      const result = await invokeTauri(command, { 
        host: host.ip_address, 
        action: action.toLowerCase().replace(' ', '_') 
      })
      
      console.log(`${action} result:`, result)
      
      // Refresh host status
      const updatedHost = await invokeTauri('get_host_info', { host: host.ip_address })
      if (updatedHost) {
        setHosts(prev => prev.map(h => h.id === hostId ? { ...h, ...updatedHost } : h))
      }
    } catch (error) {
      console.error(`${action} failed:`, error)
      alert(`${action} failed: ${(error as Error).message}`)
    }
  }

  const handleSSHConnect = async (host: HostInfo) => {
    try {
      await invokeTauri('open_ssh_connection', { 
        host: host.ip_address, 
        user: settings.defaultSSHUser 
      })
    } catch (error) {
      console.error('SSH connection failed:', error)
      alert('SSH connection failed: ' + (error as Error).message)
    }
  }

  const handleIPClick = async (ipAddress: string) => {
    try {
      await invokeTauri('open_host_in_browser', { host: ipAddress })
    } catch (error) {
      console.error('Failed to open browser:', error)
      // Fallback to window.open
      window.open(`http://${ipAddress}`, "_blank")
    }
  }

  const handleWebcam = (host: HostInfo) => {
    setCurrentWebcamUrlIndex(0);
    setWebcamRotation(0);
    setWebcamFlip({ horizontal: false, vertical: false });
    setWebcamDialog({ open: true, host })
  }

  const getWebcamUrl = (host: HostInfo, index: number = 0) => {
    const baseUrl = `http://${host.ip_address}`;
    const webcamUrls = [
      `${baseUrl}/webcam/?action=stream`,
      `${baseUrl}/webcam/stream`,
      `${baseUrl}/webcam`,
      `${baseUrl}/mjpeg/1`,
      `${baseUrl}/camera/stream`,
      `${baseUrl}:8080/?action=stream`,
      `${baseUrl}:8080/stream`,
    ];
    return webcamUrls[index] || webcamUrls[0];
  }

  const getNextWebcamUrl = (host: HostInfo) => {
    const nextIndex = (currentWebcamUrlIndex + 1) % 7;
    setCurrentWebcamUrlIndex(nextIndex);
    return getWebcamUrl(host, nextIndex);
  }

  const handleDeleteHost = (hostId: string) => {
    setHosts((prev) => prev.filter((h) => h.id !== hostId))
    setOnlineHosts((prev) => {
      const deletedHost = hosts.find((h) => h.id === hostId)
      return deletedHost?.status === "online" ? prev - 1 : prev
    })
  }

  const handleEditHostname = (hostId: string, newHostname: string) => {
    setHosts((prev) => prev.map((h) => (h.id === hostId ? { ...h, hostname: newHostname } : h)))
  }

  const getStatusBadge = (deviceStatus: string) => {
    const statusConfig = {
      printing: { color: "bg-blue-100 text-blue-800", icon: Activity },
      paused: { color: "bg-yellow-100 text-yellow-800", icon: Pause },
      error: { color: "bg-red-100 text-red-800", icon: AlertTriangle },
      ready: { color: "bg-green-100 text-green-800", icon: Play },
      standby: { color: "bg-gray-100 text-gray-800", icon: Clock },
      offline: { color: "bg-red-100 text-red-800", icon: WifiOff },
    }

    const config = statusConfig[deviceStatus as keyof typeof statusConfig] || statusConfig.offline
    const Icon = config.icon

    return (
      <span className={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium ${config.color}`}>
        <Icon className="h-3 w-3" />
        {t[deviceStatus as keyof typeof t] || deviceStatus}
      </span>
    )
  }

  const getThemeIcon = (theme: "light" | "dark" | "system") => {
    switch (theme) {
      case "light":
        return <Sun className="h-4 w-4" />
      case "dark":
        return <Moon className="h-4 w-4" />
      case "system":
        return <Monitor className="h-4 w-4" />
    }
  }

  // Компоненты для иконок переворота
  const FlipHorizontal = ({ className }: { className?: string }) => (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3" />
      <path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3" />
      <path d="M12 20v2" />
      <path d="M12 14v2" />
      <path d="M12 8v2" />
      <path d="M12 2v2" />
    </svg>
  )

  const FlipVertical = ({ className }: { className?: string }) => (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3" />
      <path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3" />
      <path d="M4 12h2" />
      <path d="M10 12h2" />
      <path d="M16 12h2" />
      <path d="M22 12h2" />
    </svg>
  )

  return (
    <div className="min-h-screen bg-background p-6">
      <div className="mx-auto max-w-7xl space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-foreground">{t.networkScanner}</h1>
            <p className="text-muted-foreground">{t.discoverHosts}</p>
          </div>

          <div className="flex items-center gap-2">
            {/* Theme Toggle Buttons */}
            <div className="flex items-center border rounded-lg p-1">
              {(["light", "dark", "system"] as const).map((theme) => (
                <Button
                  key={theme}
                  variant={settings.theme === theme ? "default" : "ghost"}
                  size="sm"
                  onClick={() => setSettings((prev) => ({ ...prev, theme }))}
                  className="h-8 w-8 p-0"
                >
                  {getThemeIcon(theme)}
                </Button>
              ))}
            </div>

            <Dialog>
              <DialogTrigger asChild>
                <Button variant="outline" size="sm">
                  <Settings className="h-4 w-4 mr-2" />
                  {t.settings}
                </Button>
              </DialogTrigger>
              <DialogContent className="max-w-2xl h-[600px] flex flex-col">
                <DialogHeader className="flex-shrink-0">
                  <DialogTitle>{t.applicationSettings}</DialogTitle>
                  <DialogDescription>{t.configureNetworkScanning}</DialogDescription>
                </DialogHeader>

                <Tabs defaultValue="network" className="flex-1 flex flex-col overflow-hidden">
                  <TabsList className="grid w-full grid-cols-5 flex-shrink-0">
                    <TabsTrigger value="network">{t.network}</TabsTrigger>
                    <TabsTrigger value="ssh">SSH</TabsTrigger>
                    <TabsTrigger value="notifications">{t.notifications}</TabsTrigger>
                    <TabsTrigger value="language">{t.language}</TabsTrigger>
                    <TabsTrigger value="about">{t.about}</TabsTrigger>
                  </TabsList>

                  <div className="flex-1 overflow-y-auto">
                    <TabsContent value="network" className="space-y-4 mt-4">
                      <div>
                        <div className="flex items-center justify-between mb-4">
                          <Label>{t.subnetRanges}</Label>
                          <Button size="sm" onClick={addSubnet}>
                            <Plus className="h-4 w-4 mr-1" />
                            {t.addSubnet}
                          </Button>
                        </div>

                        <div className="space-y-3">
                          {settings.subnets.map((subnet) => (
                            <div key={subnet.id} className="flex items-center gap-3 p-3 border rounded-lg">
                              <Checkbox
                                checked={subnet.enabled}
                                onCheckedChange={(checked) =>
                                  setSettings((prev) => ({
                                    ...prev,
                                    subnets: prev.subnets.map((s) =>
                                      s.id === subnet.id ? { ...s, enabled: checked as boolean } : s,
                                    ),
                                  }))
                                }
                              />
                              <div className="flex-1 grid grid-cols-2 gap-2">
                                <Input
                                  value={subnet.name}
                                  onChange={(e) =>
                                    setSettings((prev) => ({
                                      ...prev,
                                      subnets: prev.subnets.map((s) =>
                                        s.id === subnet.id ? { ...s, name: e.target.value } : s,
                                      ),
                                    }))
                                  }
                                  placeholder={t.networkName}
                                />
                                <Input
                                  value={subnet.range}
                                  onChange={(e) =>
                                    setSettings((prev) => ({
                                      ...prev,
                                      subnets: prev.subnets.map((s) =>
                                        s.id === subnet.id ? { ...s, range: e.target.value } : s,
                                      ),
                                    }))
                                  }
                                  placeholder="192.168.1.0/24"
                                />
                              </div>
                              <Button
                                variant="destructive"
                                size="sm"
                                onClick={() =>
                                  setSettings((prev) => ({
                                    ...prev,
                                    subnets: prev.subnets.filter((s) => s.id !== subnet.id),
                                  }))
                                }
                              >
                                <Trash2 className="h-4 w-4" />
                              </Button>
                            </div>
                          ))}
                        </div>
                      </div>

                      <div>
                        <Label htmlFor="scan-type">{t.scanType}</Label>
                        <Select defaultValue="api">
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectItem value="api">{t.apiResponseScan}</SelectItem>
                            <SelectItem value="ping">{t.pingScan}</SelectItem>
                            <SelectItem value="port">{t.portScan}</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                    </TabsContent>

                    <TabsContent value="ssh" className="space-y-4 mt-4">
                      <div>
                        <Label htmlFor="ssh-user">{t.defaultSSHUsername}</Label>
                        <Input
                          id="ssh-user"
                          value={settings.defaultSSHUser}
                          onChange={(e) =>
                            setSettings((prev) => ({
                              ...prev,
                              defaultSSHUser: e.target.value,
                            }))
                          }
                          className="mt-2"
                        />
                      </div>
                    </TabsContent>

                    <TabsContent value="notifications" className="space-y-4 mt-4">
                      <div className="space-y-3">
                        <Label>{t.enableNotificationsFor}</Label>
                        {Object.entries(settings.notifications).map(([key, value]) => (
                          <div key={key} className="flex items-center space-x-2">
                            <Checkbox
                              id={key}
                              checked={value}
                              onCheckedChange={(checked) =>
                                setSettings((prev) => ({
                                  ...prev,
                                  notifications: {
                                    ...prev.notifications,
                                    [key]: checked as boolean,
                                  },
                                }))
                              }
                            />
                            <Label htmlFor={key} className="capitalize">
                              {t[key as keyof typeof t] || key}
                            </Label>
                          </div>
                        ))}
                      </div>
                    </TabsContent>

                    <TabsContent value="language" className="space-y-4 mt-4">
                      <div>
                        <Label htmlFor="language-select">{t.selectLanguage}</Label>
                        <Select
                          value={settings.language}
                          onValueChange={(value) => setSettings((prev) => ({ ...prev, language: value }))}
                        >
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectItem value="en">English</SelectItem>
                            <SelectItem value="ru">Русский</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                    </TabsContent>

                    <TabsContent value="about" className="space-y-4 mt-4">
                      <div className="text-center space-y-2">
                        <h3 className="text-lg font-semibold">{t.networkScanner}</h3>
                        <p className="text-sm text-muted-foreground">{t.version}</p>
                        <p className="text-sm text-muted-foreground">{t.powerfulNetworkTool}</p>
                      </div>
                    </TabsContent>
                  </div>
                </Tabs>
              </DialogContent>
            </Dialog>
          </div>
        </div>

        {/* Updated Stats Cards */}
        <div className="grid gap-4 md:grid-cols-3">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">{t.onlineHosts}</CardTitle>
              <Activity className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-primary">{onlineHosts}</div>
              <p className="text-xs text-muted-foreground">{t.respondingToAPI}</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">{t.activeSubnets}</CardTitle>
              <Network className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-primary">{settings.subnets.filter((s) => s.enabled).length}</div>
              <p className="text-xs text-muted-foreground">{t.networksBeingScanned}</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">{t.scanRanges}</CardTitle>
              <Clock className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-sm font-bold text-foreground">
                {settings.subnets
                  .filter((s) => s.enabled)
                  .map((s) => s.range)
                  .join(", ") || t.none}
              </div>
              <p className="text-xs text-muted-foreground">{t.activeScanningRanges}</p>
            </CardContent>
          </Card>
        </div>

        {/* Scan Controls */}
        <Card>
          <CardHeader>
            <CardTitle>{t.networkScan}</CardTitle>
            <CardDescription>{t.scanConfiguredSubnet}</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex items-center gap-4 flex-wrap">
              <Button onClick={handleScan} disabled={isScanning} className="bg-primary hover:bg-primary/90">
                {isScanning ? (
                  <>
                    <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                    {scanStatus} {scanProgress}%
                  </>
                ) : (
                  <>
                    <Wifi className="h-4 w-4 mr-2" />
                    {t.startScan}
                  </>
                )}
              </Button>

              <div className="flex items-center gap-2">
                <Checkbox
                  id="auto-refresh"
                  checked={settings.autoRefresh}
                  onCheckedChange={(checked) => setSettings((prev) => ({ ...prev, autoRefresh: checked as boolean }))}
                />
                <Label htmlFor="auto-refresh">{t.autoRefresh}</Label>
                <Select
                  value={settings.refreshInterval.toString()}
                  onValueChange={(value) =>
                    setSettings((prev) => ({ ...prev, refreshInterval: Number.parseInt(value) }))
                  }
                >
                  <SelectTrigger className="w-32">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="10">10s</SelectItem>
                    <SelectItem value="30">30s</SelectItem>
                    <SelectItem value="60">1m</SelectItem>
                    <SelectItem value="300">5m</SelectItem>
                    <SelectItem value="600">10m</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            
            {isScanning && (
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span>{scanStatus}</span>
                  <span>{scanProgress}%</span>
                </div>
                <Progress value={scanProgress} className="w-full" />
              </div>
            )}
          </CardContent>
        </Card>

        {/* Hosts Table */}
        <Card>
          <CardHeader>
            <CardTitle>{t.discoveredHosts}</CardTitle>
            <CardDescription>{t.hostsFoundInNetwork}</CardDescription>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t.actions}</TableHead>
                  <TableHead>{t.hostname}</TableHead>
                  <TableHead>{t.ipAddress}</TableHead>
                  <TableHead>{t.status}</TableHead>
                  <TableHead>{t.ssh}</TableHead>
                  <TableHead>{t.webcam}</TableHead>
                  <TableHead>{t.delete}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {hosts.map((host) => (
                  <React.Fragment key={host.id}>
                    <TableRow className="hover:bg-muted/50">
                      <TableCell>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => toggleRowExpansion(host.id)}
                          className="flex items-center gap-1"
                        >
                          {expandedRows.has(host.id) ? (
                            <ChevronDown className="h-4 w-4" />
                          ) : (
                            <ChevronRight className="h-4 w-4" />
                          )}
                          <div className="flex items-center gap-2">
                            {getStatusIcon(host.status)}
                            {t.actions}
                          </div>
                        </Button>
                      </TableCell>
                      <TableCell>
                        <Input
                          value={host.hostname || host.hostname}
                          onChange={(e) => handleEditHostname(host.id, e.target.value)}
                          className="border-none bg-transparent p-0 h-auto focus-visible:ring-0"
                        />
                      </TableCell>
                      <TableCell>
                        <Button
                          variant="link"
                          className="p-0 h-auto text-blue-600 hover:text-blue-800"
                          onClick={() => handleIPClick(host.ip_address)}
                        >
                          {host.ip_address}
                        </Button>
                      </TableCell>
                      <TableCell>{getStatusBadge(host.device_status)}</TableCell>
                      <TableCell>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleSSHConnect(host)}
                          disabled={host.status !== "online"}
                        >
                          <Terminal className="h-4 w-4 mr-2" />
                          {t.ssh}
                        </Button>
                      </TableCell>
                      <TableCell>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleWebcam(host)}
                          disabled={host.status !== "online"}
                        >
                          <Camera className="h-4 w-4 mr-2" />
                          {t.webcam}
                        </Button>
                      </TableCell>
                      <TableCell>
                        <Button variant="destructive" size="sm" onClick={() => handleDeleteHost(host.id)}>
                          <Trash2 className="h-4 w-4 mr-2" />
                          {t.delete}
                        </Button>
                      </TableCell>
                    </TableRow>
                    {expandedRows.has(host.id) && (
                      <TableRow>
                        <TableCell colSpan={7} className="bg-muted/30">
                          <div className="py-4 space-y-2">
                            <h4 className="font-medium text-sm">{t.apiControls}</h4>
                            <div className="flex gap-2">
                              <Button
                                size="sm"
                                onClick={() => handleAPIAction("Start", host.id)}
                                disabled={host.status !== "online"}
                                className="bg-green-600 hover:bg-green-700"
                              >
                                <Play className="h-4 w-4 mr-1" />
                                {t.start}
                              </Button>
                              <Button
                                size="sm"
                                variant="outline"
                                onClick={() => handleAPIAction("Pause", host.id)}
                                disabled={host.status !== "online"}
                              >
                                <Pause className="h-4 w-4 mr-1" />
                                {t.pause}
                              </Button>
                              <Button
                                size="sm"
                                variant="outline"
                                onClick={() => handleAPIAction("Stop", host.id)}
                                disabled={host.status !== "online"}
                              >
                                <Square className="h-4 w-4 mr-1" />
                                {t.stop}
                              </Button>
                              <Button
                                size="sm"
                                variant="destructive"
                                onClick={() => handleAPIAction("Emergency Stop", host.id)}
                                disabled={host.status !== "online"}
                              >
                                <AlertTriangle className="h-4 w-4 mr-1" />
                                {t.emergencyStop}
                              </Button>
                            </div>
                          </div>
                        </TableCell>
                      </TableRow>
                    )}
                  </React.Fragment>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </div>

      {/* Webcam Dialog */}
      <Dialog open={webcamDialog.open} onOpenChange={(open) => setWebcamDialog({ open, host: null })}>
        <DialogContent className="max-w-4xl">
          <DialogHeader>
            <DialogTitle>
              {t.webcam} - {webcamDialog.host?.hostname} ({webcamDialog.host?.ip_address})
            </DialogTitle>
            <DialogDescription>
              Live webcam stream from {webcamDialog.host?.hostname}
            </DialogDescription>
          </DialogHeader>
          <div className="aspect-video bg-black rounded-lg overflow-hidden">
            {webcamDialog.host && (
              <div className="w-full h-full">
                <div className="text-center mb-2 flex items-center justify-between px-4 relative z-10">
                  <p className="text-white text-sm">
                    {webcamDialog.host.hostname} ({webcamDialog.host.ip_address})
                  </p>
                  <div className="flex gap-2">
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => setWebcamRotation((prev) => (prev + 90) % 360)}
                      title="Rotate 90° clockwise"
                    >
                      <RotateCw className="h-4 w-4" />
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => setWebcamFlip(prev => ({ ...prev, horizontal: !prev.horizontal }))}
                      title="Flip horizontally"
                    >
                      <FlipHorizontal className="h-4 w-4" />
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => setWebcamFlip(prev => ({ ...prev, vertical: !prev.vertical }))}
                      title="Flip vertically"
                    >
                      <FlipVertical className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
                <div className="relative w-full h-full overflow-hidden">
                  <div 
                    className="w-full h-full flex items-center justify-center"
                    style={{
                      transform: `
                        rotate(${webcamRotation}deg) 
                        scaleX(${webcamFlip.horizontal ? -1 : 1}) 
                        scaleY(${webcamFlip.vertical ? -1 : 1})
                      `,
                      transition: 'transform 0.3s ease'
                    }}
                  >
                    <img
                      className="webcam-img max-w-full max-h-full object-contain"
                      src={getWebcamUrl(webcamDialog.host, currentWebcamUrlIndex)}
                      alt={`Webcam stream from ${webcamDialog.host.hostname}`}
                      crossOrigin="anonymous"
                      onLoad={() => {
                        console.log('Webcam stream loaded successfully');
                        const fallback = document.querySelector('.fallback') as HTMLElement;
                        if (fallback) fallback.style.display = 'none';
                      }}
                      onError={(e) => {
                        console.error('Failed to load webcam stream:', e);
                        const target = e.target as HTMLImageElement;
                        target.style.display = 'none';
                        const fallback = target.parentElement?.querySelector('.fallback') as HTMLElement;
                        if (fallback) fallback.style.display = 'flex';
                      }}
                    />
                  </div>
                  <div className="fallback hidden absolute inset-0 flex items-center justify-center bg-black bg-opacity-75">
                    <div className="text-center">
                      <p className="text-white mb-4">Webcam stream not available</p>
                      <p className="text-gray-400 mb-4 text-sm">
                        Current URL: {getWebcamUrl(webcamDialog.host, currentWebcamUrlIndex)}
                      </p>
                      <Button
                        onClick={() => {
                          const url = getWebcamUrl(webcamDialog.host!, currentWebcamUrlIndex);
                          console.log('Opening webcam URL:', url);
                          window.open(url, '_blank');
                        }}
                        className="bg-blue-600 hover:bg-blue-700"
                      >
                        <ExternalLink className="h-4 w-4 mr-2" />
                        Open in Browser
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
