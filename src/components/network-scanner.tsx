/**
 * Moonraker Host Scanner - Main Application Component
 * 
 * This component provides a comprehensive interface for discovering, monitoring,
 * and controlling Moonraker-enabled 3D printers on the network.
 * 
 * Features:
 * - Network scanning and host discovery
 * - Real-time printer status monitoring
 * - Printer control (start, pause, stop, emergency stop)
 * - Webcam streaming with image manipulation
 * - System notifications for status changes
 * - Multi-language support (English, Russian, German)
 * - Custom hostname management
 * - Settings persistence
 */

"use client"

import React, { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
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
  ChevronUp,
  Layers,
  Send,
} from "lucide-react"
import { useTranslation } from "@/lib/i18n"
import { useUpdater } from "@/hooks/use-updater"
import { useTelegramBot } from "@/hooks/useTelegramBot"
import { useSmartNotifications } from "@/hooks/useSmartNotifications"

/**
 * Network subnet configuration for scanning
 */
interface Subnet {
  id: string
  range: string
  name: string
  enabled: boolean
}

/**
 * Host information for discovered Moonraker printers
 */
interface HostInfo {
  id: string
  hostname: string
  original_hostname: string // Original hostname from server
  ip_address: string
  subnet: string
  status: "online" | "offline"
  device_status: string
  moonraker_version?: string
  klippy_state?: string
  printer_state?: string
  printer_flags?: {
    operational: boolean
    paused: boolean
    printing: boolean
    cancelling: boolean
    pausing: boolean
    resuming?: boolean
    sdReady?: boolean
    error: boolean
    ready: boolean
    closedOrError: boolean
  }
  last_seen?: string
  failed_attempts?: number // Counter for consecutive failed attempts
  print_progress?: number // Current print progress percentage (0-100)
  print_info?: {
    filename: string
    print_duration: number
    total_duration: number
  }
  order?: number // Order index for manual sorting
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

/**
 * Host group for batch operations
 */
interface HostGroup {
  id: string
  name: string
  hostIds: string[]
  createdAt: string
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
    cancelling: boolean
    error: boolean
    standby: boolean
    offline: boolean
  }
  telegram: {
    enabled: boolean
    notifications: {
      printing: boolean
      paused: boolean
      cancelling: boolean
      error: boolean
      standby: boolean
      offline: boolean
    }
  }
  theme: "light" | "dark" | "system"
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
      cancelling: true,
      error: true,
      standby: false,
      offline: true,
    },
    telegram: {
      enabled: false,
      notifications: {
        printing: true,
        paused: true,
        cancelling: true,
        error: true,
        standby: false,
        offline: true,
      },
    },
    theme: "system",
    language: "en", // Added default language
  })

  const [onlineHosts, setOnlineHosts] = useState(0)
  const [isScanning, setIsScanning] = useState(false)


  const [hosts, setHosts] = useState<HostInfo[]>([])
  const [webcamDialog, setWebcamDialog] = useState<{ open: boolean; host: HostInfo | null }>({
    open: false,
    host: null,
  })
  const [batchTasksDialog, setBatchTasksDialog] = useState(false)
  const [sendBatchTaskDialog, setSendBatchTaskDialog] = useState(false)
  const [selectedGcodeFile, setSelectedGcodeFile] = useState<File | null>(null)
  const [isUploading, setIsUploading] = useState(false)
  const [uploadProgress, setUploadProgress] = useState<Record<string, boolean>>({})
  const [hostGroups, setHostGroups] = useState<HostGroup[]>([])
  const [newGroupName, setNewGroupName] = useState("")
  const [selectedHostsForGroup, setSelectedHostsForGroup] = useState<string[]>([])
  const [editingGroupId, setEditingGroupId] = useState<string | null>(null)
  const [currentWebcamUrlIndex, setCurrentWebcamUrlIndex] = useState(0)
  const [webcamRotation, setWebcamRotation] = useState(0)
  const [webcamFlip, setWebcamFlip] = useState({ horizontal: false, vertical: false })
  const [webcamRefreshKey, setWebcamRefreshKey] = useState(0)
  const [expandedRows, setExpandedRows] = useState(new Set())
  const [loadingButtons, setLoadingButtons] = useState<Set<string>>(new Set())

  const t = useTranslation(settings.language)
  const { updateInfo, repositoryInfo, isChecking, checkForUpdates, openRepository, openReleases } = useUpdater()
  const { 
    status: telegramStatus, 
    users: telegramUsers, 
    registrationCode, 
    registrationTimeLeft,
    hasToken,
    startBot, 
    stopBot, 
    startRegistration, 
    stopRegistration,
    removeUser, 
    loadUsers,
    syncHostsWithBot,
    updateUserNotifications,
    saveToken,
    clearToken
  } = useTelegramBot(settings.telegram.enabled)
  
  const { 
    getPrinterStatus: smartGetPrinterStatus,
    checkStatusChangeAndNotify: smartCheckStatusChangeAndNotify,
    resetHostTimeout,
    getHostNotificationState
  } = useSmartNotifications()

  // Tauri API functions
  const invokeTauri = async (command: string, args?: any) => {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      return await (window as any).__TAURI__.core.invoke(command, args)
    }
    throw new Error('Tauri API not available')
  }

  // Get print information for a host
  const getPrintInfo = async (host: HostInfo) => {
    try {
      const printInfo = await invokeTauri('get_print_info_command', { 
        host: host.ip_address, 
        port: 7125 
      })
      
      if (printInfo) {
        // Проверяем, что у нас есть все необходимые поля
        if (!printInfo.progress || typeof printInfo.progress.progress !== 'number') {
          return null;
        }
        
        const result = {
          print_progress: printInfo.progress.progress,
          print_info: {
            filename: printInfo.filename || 'Unknown',
            print_duration: printInfo.progress.print_duration || 0,
            total_duration: printInfo.progress.total_duration || 0,
          }
        };
        return result;
      }
    } catch (error) {
      console.error('Failed to get print info:', error)
    }
    return null
  }

  // Format duration in human readable format
  const formatDuration = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    
    if (hours > 0) {
      return `${hours}h ${minutes}m ${secs}s`;
    } else if (minutes > 0) {
      return `${minutes}m ${secs}s`;
    } else {
      return `${secs}s`;
    }
  }

  // Отправка системного уведомления
  const sendNotification = async (title: string, body: string) => {
    try {
      // Send system notification
      await invokeTauri('send_system_notification_command', { title, body })
      
      // Send Telegram notification if bot is enabled and running
      if (settings.telegram.enabled && telegramStatus.isRunning) {
        try {
          await invokeTauri('send_telegram_notification', { title, body, hostIp: undefined })
        } catch (error) {
          console.error('Failed to send Telegram notification:', error);
        }
      }
    } catch (error) {
      console.error('Failed to send notification:', error);
      // Silent fail for notifications
    }
  }

  // Load settings from localStorage
  useEffect(() => {
    const savedSettings = localStorage.getItem('networkScanner_settings')
    if (savedSettings) {
      try {
        const parsed = JSON.parse(savedSettings)
        
        // Ensure telegram.notifications exists with default values
        if (parsed.telegram && !parsed.telegram.notifications) {
          parsed.telegram.notifications = {
            printing: true,
            paused: true,
            cancelling: true,
            error: true,
            standby: false,
            offline: true,
          }
        }
        
        // Remove botToken from old settings if it exists
        if (parsed.telegram && parsed.telegram.botToken) {
          delete parsed.telegram.botToken
        }
        
        setSettings(prev => ({ ...prev, ...parsed }))
      } catch (error) {
        // Silent fail for settings loading
      }
    }
  }, [])

  // Save settings to localStorage
  useEffect(() => {
    localStorage.setItem('networkScanner_settings', JSON.stringify(settings))
  }, [settings])

  // Apply notification settings to existing hosts when settings change
  useEffect(() => {
    if (hosts.length > 0) {
      // Small delay to ensure settings are saved
      setTimeout(() => {
        refreshHostsStatus();
      }, 200);
    }
  }, [settings.notifications])

  // Load hosts from localStorage
  useEffect(() => {
    const savedHosts = localStorage.getItem('networkScanner_hosts')
    if (savedHosts) {
      try {
        const parsed = JSON.parse(savedHosts)
        // Обеспечиваем обратную совместимость с существующими данными
        const hostsWithOriginal = parsed.map((host: any, index: number) => ({
          ...host,
          original_hostname: host.original_hostname || host.hostname,
          order: host.order !== undefined ? host.order : index // Инициализируем порядок если его нет
        }))
        setHosts(hostsWithOriginal)
        setOnlineHosts(hostsWithOriginal.filter((h: HostInfo) => h.status === 'online').length)
      } catch (error) {
        // Silent fail for hosts loading
      }
    }
  }, [])

  // Sync hosts with Telegram bot when hosts change
  useEffect(() => {
    if (hosts.length > 0 && telegramStatus.isRunning) {
      syncHostsWithBot(hosts)
    }
  }, [hosts, telegramStatus.isRunning, syncHostsWithBot])

  // Update network names when language changes
  useEffect(() => {
    setSettings(prev => ({
      ...prev,
      subnets: prev.subnets.map(subnet => {
        if (subnet.id === "1") {
          return { ...subnet, name: t.mainNetwork }
        } else if (subnet.id === "2") {
          return { ...subnet, name: t.guestNetwork }
        }
        return subnet
      })
    }))
  }, [t.mainNetwork, t.guestNetwork])

  // Save hosts to localStorage
  useEffect(() => {
    localStorage.setItem('networkScanner_hosts', JSON.stringify(hosts))
  }, [hosts])

  // Load host groups from localStorage on component mount
  useEffect(() => {
    const savedGroups = localStorage.getItem('networkScanner_hostGroups')
    if (savedGroups) {
      try {
        const parsedGroups = JSON.parse(savedGroups)
        if (Array.isArray(parsedGroups)) {
          setHostGroups(parsedGroups)
        }
      } catch (error) {
        console.error('Failed to load host groups:', error)
      }
    }
  }, [])

  // Save host groups to localStorage (only when groups actually change)
  useEffect(() => {
    // Don't save on initial mount with empty array
    if (hostGroups.length > 0 || localStorage.getItem('networkScanner_hostGroups') !== null) {
      console.log('Saving host groups to localStorage:', hostGroups)
      localStorage.setItem('networkScanner_hostGroups', JSON.stringify(hostGroups))
    }
  }, [hostGroups])

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

  // Auto-refresh webcam when dialog is open
  useEffect(() => {
    let webcamInterval: NodeJS.Timeout | null = null;

    if (webcamDialog.open && webcamDialog.host) {
      // Refresh webcam every 3 seconds
      webcamInterval = setInterval(() => {
        setWebcamRefreshKey(prev => prev + 1);
      }, 3000);
    }

    return () => {
      if (webcamInterval) {
        clearInterval(webcamInterval);
      }
    };
  }, [webcamDialog.open, webcamDialog.host]);

  // Auto-refresh effect - обновление статусов хостов каждую секунду
  useEffect(() => {
    let statusIntervalId: NodeJS.Timeout | null = null

    if (hosts.length > 0) {
      // Запускаем первый refresh статусов сразу
      refreshHostsStatus()
      
      // Устанавливаем интервал обновления статусов каждую секунду
      statusIntervalId = setInterval(() => {
        refreshHostsStatus()
      }, 1000) // 1 секунда
    }

    return () => {
      if (statusIntervalId) {
        clearInterval(statusIntervalId)
      }
    }
  }, [hosts.length])


  // Функция для перемещения хоста вверх
  const moveHostUp = (hostId: string) => {
    setHosts(prevHosts => {
      const sortedHosts = [...prevHosts].sort((a, b) => (a.order || 0) - (b.order || 0))
      const currentIndex = sortedHosts.findIndex(h => h.id === hostId)
      
      if (currentIndex > 0) {
        const reorderedHosts: HostInfo[] = [...sortedHosts]
        // Меняем местами с предыдущим хостом
        const temp = reorderedHosts[currentIndex]
        reorderedHosts[currentIndex] = reorderedHosts[currentIndex - 1]
        reorderedHosts[currentIndex - 1] = temp
        
        // Обновляем порядок
        return reorderedHosts.map((host: HostInfo, index: number) => ({ ...host, order: index }))
      }
      
      return prevHosts
    })
  }

  // Функция для перемещения хоста вниз
  const moveHostDown = (hostId: string) => {
    setHosts(prevHosts => {
      const sortedHosts = [...prevHosts].sort((a, b) => (a.order || 0) - (b.order || 0))
      const currentIndex = sortedHosts.findIndex(h => h.id === hostId)
      
      if (currentIndex < sortedHosts.length - 1) {
        const reorderedHosts: HostInfo[] = [...sortedHosts]
        // Меняем местами со следующим хостом
        const temp = reorderedHosts[currentIndex]
        reorderedHosts[currentIndex] = reorderedHosts[currentIndex + 1]
        reorderedHosts[currentIndex + 1] = temp
        
        // Обновляем порядок
        return reorderedHosts.map((host: HostInfo, index: number) => ({ ...host, order: index }))
      }
      
      return prevHosts
    })
  }

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
    
    try {
      const enabledSubnets = settings.subnets.filter(s => s.enabled)
      if (enabledSubnets.length === 0) {
        alert(t.noSubnetsEnabled || 'No subnets enabled for scanning')
        setIsScanning(false)
        return
      }
        const result = await invokeTauri('scan_network_command', { subnets: enabledSubnets })
        
        if (result.hosts) {
          // Сохраняем пользовательские имена и порядок при повторном сканировании
          setHosts(prevHosts => {
            const sortedPrevHosts = [...prevHosts].sort((a, b) => (a.order || 0) - (b.order || 0))
            const maxOrder = Math.max(...sortedPrevHosts.map(h => h.order || 0), -1)
            
            const updatedHosts: HostInfo[] = []
            const newHosts: HostInfo[] = []
            let newHostIndex = 0
            
            // Сначала обрабатываем все существующие хосты
            sortedPrevHosts.forEach(existingHost => {
              const foundHost = result.hosts.find((newHost: any) => newHost.ip_address === existingHost.ip_address)
              
              if (foundHost) {
                // Хост найден при сканировании - обновляем его данные
                updatedHosts.push({
                  ...foundHost,
                  original_hostname: foundHost.hostname,
                  hostname: (existingHost.hostname !== existingHost.original_hostname)
                    ? existingHost.hostname // Сохраняем пользовательское имя, если оно было изменено
                    : foundHost.hostname, // Используем новое имя с сервера, если пользователь не изменял
                  failed_attempts: 0, // Сбрасываем счетчик неудачных попыток
                  order: existingHost.order || 0
                })
              } else {
                // Хост не найден при сканировании - сохраняем его как есть, но помечаем как offline
                updatedHosts.push({
                  ...existingHost,
                  status: 'offline',
                  device_status: 'offline',
                  last_seen: new Date().toISOString(),
                  failed_attempts: (existingHost.failed_attempts || 0) + 1
                })
              }
            })
            
            // Затем добавляем новые хосты, которых не было в списке
            result.hosts.forEach((newHost: any) => {
              const existingHost = sortedPrevHosts.find(h => h.ip_address === newHost.ip_address)
              
              if (!existingHost) {
                // Новый хост - добавляем в конец
                newHosts.push({
                  ...newHost,
                  original_hostname: newHost.hostname,
                  failed_attempts: 0,
                  order: maxOrder + newHostIndex + 1
                })
                newHostIndex++
              }
            })
            
            // Объединяем обновленные и новые хосты
            return [...updatedHosts, ...newHosts]
          })
          
          setOnlineHosts(result.online_hosts || 0)
        }
    } catch (error) {
      console.error('Scan error:', error)
    } finally {
      setIsScanning(false)
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

    const buttonKey = `${hostId}-${action}`

    try {
      // Преобразуем действие в правильный формат для API
      let apiAction = ''
      switch (action) {
        case t.pause:
          apiAction = 'pause'
          break
        case t.resume:
          apiAction = 'resume'
          break
        case t.stop:
          apiAction = 'cancel' // Moonraker использует 'cancel' для остановки
          break
        case t.emergencyStop:
          apiAction = 'emergency_stop'
          break
        default:
          return
      }

      setLoadingButtons(prev => new Set([...prev, buttonKey]))
      
      const result = await invokeTauri('control_printer_command', { 
        host: host.ip_address, 
        action: apiAction
      })
      
      // Убираем состояние загрузки
      setLoadingButtons(prev => {
        const newSet = new Set(prev)
        newSet.delete(buttonKey)
        return newSet
      })
      
    } catch (error) {
      alert(`${action} failed: ${(error as Error).message}`)
      setLoadingButtons(prev => {
        const newSet = new Set(prev)
        newSet.delete(buttonKey)
        return newSet
      })
    }
  }

  const handleSSHConnect = async (host: HostInfo) => {
    try {
      await invokeTauri('open_ssh_connection_command', { 
        host: host.ip_address, 
        user: settings.defaultSSHUser 
      })
    } catch (error) {
      alert('SSH connection failed: ' + (error as Error).message)
    }
  }

  const handleIPClick = async (ipAddress: string) => {
    try {
      await invokeTauri('open_host_in_browser_command', { host: ipAddress })
    } catch (error) {
      // Fallback to window.open
      window.open(`http://${ipAddress}`, "_blank")
    }
  }

  const handleWebcam = (host: HostInfo) => {
    setCurrentWebcamUrlIndex(0);
    setWebcamRotation(0);
    setWebcamFlip({ horizontal: false, vertical: false });
    setWebcamRefreshKey(0);
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
    const url = webcamUrls[index] || webcamUrls[0];
    const urlWithRefresh = `${url}${url.includes('?') ? '&' : '?'}t=${webcamRefreshKey}`;
    return urlWithRefresh;
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
    setHosts((prev) => prev.map((h) => (h.id === hostId ? { 
      ...h, 
      hostname: newHostname
    } : h)))
  }


  // Функция для обновления состояния хостов
  const refreshHostsStatus = async () => {
    if (hosts.length === 0) return

    try {
      const updatedHosts: HostInfo[] = []
      
      // Обновляем хосты по одному
      for (let i = 0; i < hosts.length; i++) {
        const host = hosts[i]
        
        try {
          // Проверяем состояние хоста через Tauri API
          const result = await invokeTauri('check_host_status_command', { ip: host.ip_address })
          
          if (result.success) {
            // Хост ответил успешно - сбрасываем счетчик неудачных попыток
            
            // Получаем информацию о печати если хост печатает
            let printInfo = null;
            
            if (result.printer_flags?.printing) {
              printInfo = await getPrintInfo(host);
            }

            updatedHosts.push({
              ...host,
              // Сохраняем пользовательские данные
              hostname: host.hostname, // Сохраняем пользовательское имя
              original_hostname: host.original_hostname, // Сохраняем оригинальное имя
              // Обновляем только техническую информацию
              status: result.status as "online" | "offline",
              device_status: result.device_status || host.device_status,
              moonraker_version: result.moonraker_version || host.moonraker_version,
              klippy_state: result.klippy_state || host.klippy_state,
              printer_state: result.printer_state || host.printer_state,
              printer_flags: result.printer_flags || host.printer_flags,
              last_seen: new Date().toISOString(),
              failed_attempts: 0, // Сбрасываем счетчик неудачных попыток
              // Добавляем информацию о печати
              print_progress: printInfo?.print_progress,
              print_info: printInfo?.print_info
            })
          } else {
            // Хост не ответил - помечаем как offline
            updatedHosts.push({
              ...host,
              // Сохраняем пользовательские данные
              hostname: host.hostname, // Сохраняем пользовательское имя
              original_hostname: host.original_hostname, // Сохраняем оригинальное имя
              // Помечаем как offline
              status: 'offline',
              device_status: 'offline',
              last_seen: new Date().toISOString(),
              failed_attempts: (host.failed_attempts || 0) + 1
            })
          }
        } catch (error) {
          // В случае ошибки помечаем как offline
          updatedHosts.push({
            ...host,
            // Сохраняем пользовательские данные
            hostname: host.hostname, // Сохраняем пользовательское имя
            original_hostname: host.original_hostname, // Сохраняем оригинальное имя
            // Помечаем как offline
            status: 'offline',
            device_status: 'offline',
            last_seen: new Date().toISOString(),
            failed_attempts: (host.failed_attempts || 0) + 1
          })
        }
      }

      // Обновляем хосты, сохраняя пользовательские имена
      setHosts(prevHosts => 
        prevHosts.map(prevHost => {
          const updatedHost = updatedHosts.find(h => h.id === prevHost.id)
          if (updatedHost) {
            // Проверяем, изменил ли пользователь имя
            const hasCustomName = prevHost.hostname !== prevHost.original_hostname
            
            const newHost = {
              ...prevHost,
              status: updatedHost.status as "online" | "offline",
              device_status: updatedHost.device_status,
              moonraker_version: updatedHost.moonraker_version,
              klippy_state: updatedHost.klippy_state,
              printer_state: updatedHost.printer_state,
              printer_flags: updatedHost.printer_flags,
              last_seen: updatedHost.last_seen,
              failed_attempts: updatedHost.failed_attempts,
              // Добавляем информацию о печати
              print_progress: updatedHost.print_progress,
              print_info: updatedHost.print_info,
              // Сохраняем пользовательское имя, если оно было изменено
              hostname: hasCustomName ? prevHost.hostname : updatedHost.hostname,
              original_hostname: updatedHost.original_hostname
            }
            
            // Проверяем изменения статуса и отправляем уведомления
            checkStatusChangeAndNotify(prevHost, newHost)
            
            return newHost
          }
          return prevHost
        })
      )
      
      setOnlineHosts(updatedHosts.filter(h => h.status === 'online').length)
    } catch (error) {
      // Silent fail for auto-refresh
    }
  }

  /**
   * Determines printer status based on Moonraker API flags
   * Priority order: offline > cancelling > error > paused > printing > ready > standby
   * @param host - Host information containing printer flags
   * @returns Status string for display
   */
  const getPrinterStatus = (host: HostInfo): string => {
    return smartGetPrinterStatus(host)
  }

  /**
   * Checks for status changes and sends system notifications with smart deduplication
   * @param oldHost - Previous host state
   * @param newHost - Current host state
   */
  const checkStatusChangeAndNotify = (oldHost: HostInfo, newHost: HostInfo) => {
    // Get current settings to ensure we have the latest notification preferences
    const currentSettings = JSON.parse(localStorage.getItem('networkScanner_settings') || '{}')
    
    // Ensure telegram.notifications exists with default values
    if (currentSettings.telegram && !currentSettings.telegram.notifications) {
      currentSettings.telegram.notifications = {
        printing: true,
        paused: true,
        cancelling: true,
        error: true,
        standby: false,
        offline: true,
      }
    }
    
    const notifications = currentSettings.notifications || settings.notifications
    
    // Use smart notification system
    smartCheckStatusChangeAndNotify(oldHost, newHost, hosts, notifications, t)
    
    // Handle Telegram notifications separately (they have their own logic)
    const oldStatus = getPrinterStatus(oldHost)
    const newStatus = getPrinterStatus(newHost)
    
    if (oldStatus !== newStatus) {
      const telegramNotifications = currentSettings.telegram?.notifications || {
        printing: true,
        paused: true,
        cancelling: true,
        error: true,
        standby: false,
        offline: true,
      }
      
      // Send Telegram notification if enabled
      const statusKey = newStatus as keyof typeof telegramNotifications
      const telegramNotificationEnabled = telegramNotifications[statusKey];
      
      if (telegramNotificationEnabled && currentSettings.telegram?.enabled) {
        const title = `${t.networkScanner} - ${oldHost.hostname}`
        const body = `${t.status}: ${t[statusKey as keyof typeof t] || newStatus}`
        
        invokeTauri('send_telegram_notification', { title, body, hostIp: oldHost.ip_address }).catch(error => {
          console.error('Failed to send Telegram notification:', error);
        });
      }
    }
  }

  const getStatusBadge = (status: string, host?: HostInfo) => {
    const statusConfig = {
      printing: { color: "bg-blue-100 text-blue-800", icon: Activity },
      paused: { color: "bg-yellow-100 text-yellow-800", icon: Pause },
      cancelling: { color: "bg-orange-100 text-orange-800", icon: Square },
      error: { color: "bg-red-100 text-red-800", icon: AlertTriangle },
      ready: { color: "bg-green-100 text-green-800", icon: Play },
      standby: { color: "bg-gray-100 text-gray-800", icon: Clock },
      offline: { color: "bg-red-100 text-red-800", icon: WifiOff },
    }

    const config = statusConfig[status as keyof typeof statusConfig] || statusConfig.offline
    const Icon = config.icon

    return (
      <span className={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium ${config.color}`}>
        <Icon className="h-3 w-3" />
        {t[status as keyof typeof t] || status}
        {status === 'printing' && host?.print_progress && (
          <span className="ml-1 font-bold">
            {Math.round(host.print_progress)}%
          </span>
        )}
              {status === 'printing' && !host?.print_progress && (
        <span className="ml-1 text-xs text-muted-foreground">
          (no progress)
        </span>
      )}
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

  // Group management functions
  const handleSaveGroup = () => {
    if (!newGroupName.trim() || selectedHostsForGroup.length === 0) return

    const newGroup: HostGroup = {
      id: Date.now().toString(),
      name: newGroupName.trim(),
      hostIds: selectedHostsForGroup,
      createdAt: new Date().toISOString()
    }

    console.log('Creating new group:', newGroup)
    setHostGroups(prev => {
      const updated = [...prev, newGroup]
      console.log('Updated host groups:', updated)
      return updated
    })
    setNewGroupName("")
    setSelectedHostsForGroup([])
  }

  const handleDeleteGroup = (groupId: string) => {
    setHostGroups(prev => prev.filter(g => g.id !== groupId))
  }

  const handleEditGroupName = (groupId: string, newName: string) => {
    setHostGroups(prev => prev.map(g => 
      g.id === groupId ? { ...g, name: newName } : g
    ))
  }

  const handleHostSelectionChange = (hostId: string, checked: boolean) => {
    setSelectedHostsForGroup(prev => 
      checked 
        ? [...prev, hostId]
        : prev.filter(id => id !== hostId)
    )
  }

  // Batch task functions
  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file && file.name.toLowerCase().endsWith('.gcode')) {
      setSelectedGcodeFile(file)
    } else {
      alert('Please select a .gcode file')
    }
  }

  const checkHostsStatus = async (group: HostGroup): Promise<boolean> => {
    const groupHosts = hosts.filter(host => group.hostIds.includes(host.id))
    
    for (const host of groupHosts) {
      try {
        const result = await invokeTauri('check_host_status_command', { ip: host.ip_address })
        if (!result.success || getPrinterStatus({ ...host, ...result }) !== 'standby') {
          return false
        }
      } catch (error) {
        console.error(`Failed to check status for host ${host.hostname}:`, error)
        return false
      }
    }
    
    return true
  }

  const uploadFileToHost = async (host: HostInfo, file: File): Promise<boolean> => {
    try {
      // Create FormData for file upload
      const formData = new FormData()
      formData.append('file', file)
      formData.append('print', 'true')
      
      // Upload file using fetch
      const response = await fetch(`http://${host.ip_address}:7125/server/files/upload`, {
        method: 'POST',
        body: formData,
      })
      
      return response.ok
    } catch (error) {
      console.error(`Failed to upload file to host ${host.hostname}:`, error)
      return false
    }
  }

  const handleLaunchBatchTask = async (group: HostGroup) => {
    if (!selectedGcodeFile) {
      alert(t.noFileSelected)
      return
    }

    setIsUploading(true)
    setUploadProgress({})

    try {
      // Check if all hosts are in standby status
      const allHostsReady = await checkHostsStatus(group)
      
      if (!allHostsReady) {
        alert(t.hostsNotReadyMessage)
        setIsUploading(false)
        return
      }

      // Upload file to all hosts in the group
      const groupHosts = hosts.filter(host => group.hostIds.includes(host.id))
      const uploadPromises = groupHosts.map(async (host) => {
        setUploadProgress(prev => ({ ...prev, [host.id]: true }))
        
        const success = await uploadFileToHost(host, selectedGcodeFile)
        
        setUploadProgress(prev => ({ ...prev, [host.id]: false }))
        return { host, success }
      })

      const results = await Promise.all(uploadPromises)
      const failedUploads = results.filter(result => !result.success)
      
      if (failedUploads.length === 0) {
        alert(t.uploadSuccess)
      } else {
        alert(`${t.uploadError}: ${failedUploads.map(r => r.host.hostname).join(', ')}`)
      }
      
    } catch (error) {
      console.error('Batch task failed:', error)
      alert(t.uploadError)
    } finally {
      setIsUploading(false)
      setUploadProgress({})
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
            {/* Update notification */}
            {updateInfo?.update_available && (
              <div className="flex items-center gap-2 px-3 py-2 bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800 rounded-lg">
                <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></div>
                <span className="text-sm text-blue-700 dark:text-blue-300">
                  {t.updateAvailable}: {updateInfo.latest_version}
                </span>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={checkForUpdates}
                  className="h-6 px-2 text-xs"
                >
                  {t.checkForUpdates}
                </Button>
              </div>
            )}
            
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
              <DialogContent className="max-w-4xl h-[600px] flex flex-col">
                <DialogHeader className="flex-shrink-0">
                  <DialogTitle>{t.applicationSettings}</DialogTitle>
                  <DialogDescription>{t.configureNetworkScanning}</DialogDescription>
                </DialogHeader>

                <Tabs defaultValue="network" className="flex-1 flex flex-col overflow-hidden">
                  <TabsList className="flex w-full flex-shrink-0 justify-between">
                    <TabsTrigger value="network">{t.network}</TabsTrigger>
                    <TabsTrigger value="ssh">{t.ssh}</TabsTrigger>
                    <TabsTrigger value="notifications">{t.notifications}</TabsTrigger>
                    <TabsTrigger value="telegram">{t.telegram}</TabsTrigger>
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
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {/* System Notifications */}
                        <div className="space-y-3">
                          <Label className="text-base font-semibold">{t.enableNotificationsFor}</Label>
                          {[
                            { key: 'printing', label: t.printing },
                            { key: 'paused', label: t.paused },
                            { key: 'cancelling', label: t.cancelling },
                            { key: 'error', label: t.error },
                            { key: 'standby', label: t.standby },
                            { key: 'offline', label: t.offline }
                          ].map(({ key, label }) => (
                            <div key={key} className="flex items-center space-x-2">
                              <Checkbox
                                id={key}
                                checked={settings.notifications[key as keyof typeof settings.notifications]}
                                onCheckedChange={(checked) => {
                                  setSettings((prev) => {
                                    const newSettings = {
                                      ...prev,
                                      notifications: {
                                        ...prev.notifications,
                                        [key]: checked as boolean,
                                      },
                                    };
                                    
                                    // Force refresh all existing hosts with new notification settings
                                    setTimeout(() => {
                                      refreshHostsStatus();
                                    }, 100);
                                    
                                    return newSettings;
                                  });
                                }}
                              />
                              <Label htmlFor={key} className="capitalize">
                                {label}
                              </Label>
                            </div>
                          ))}
                        </div>

                        {/* Telegram Notifications */}
                        <div className="space-y-3">
                          <Label className="text-base font-semibold">{t.telegramNotifications}</Label>
                          {[
                            { key: 'printing', label: t.printing },
                            { key: 'paused', label: t.paused },
                            { key: 'cancelling', label: t.cancelling },
                            { key: 'error', label: t.error },
                            { key: 'standby', label: t.standby },
                            { key: 'offline', label: t.offline }
                          ].map(({ key, label }) => (
                            <div key={`telegram-${key}`} className="flex items-center space-x-2">
                              <Checkbox
                                id={`telegram-${key}`}
                                checked={settings.telegram.notifications?.[key as keyof typeof settings.telegram.notifications] || false}
                                disabled={!settings.telegram.enabled}
                                onCheckedChange={(checked) => {
                                  setSettings((prev) => ({
                                    ...prev,
                                    telegram: {
                                      ...prev.telegram,
                                      notifications: {
                                        ...(prev.telegram.notifications || {
                                          printing: true,
                                          paused: true,
                                          cancelling: true,
                                          error: true,
                                          standby: false,
                                          offline: true,
                                        }),
                                        [key]: checked as boolean,
                                      },
                                    },
                                  }));
                                }}
                              />
                              <Label htmlFor={`telegram-${key}`} className={`capitalize ${!settings.telegram.enabled ? 'text-gray-400' : ''}`}>
                                {label}
                              </Label>
                            </div>
                          ))}
                        </div>
                      </div>
                          
                      {/* Test notification buttons */}
                      <div className="pt-4 border-t space-y-2">
                        <div className="flex gap-2">
                          <Button 
                            onClick={() => sendNotification(t.testSystemNotification, t.testSystemNotificationBody)}
                            variant="outline"
                            className="flex-1"
                          >
{t.testSystem}
                          </Button>
                          
                          <Button 
                            onClick={async () => {
                              try {
                                await invokeTauri('send_telegram_notification', { 
                                  title: t.testTelegramNotification, 
                                  body: t.testTelegramNotificationBody,
                                  hostIp: undefined
                                })
                              } catch (error) {
                                console.error('Failed to send test Telegram notification:', error);
                              }
                            }}
                            variant="outline"
                            className="flex-1"
                            disabled={!settings.telegram.enabled || !telegramStatus.isRunning}
                          >
{t.testTelegram}
                          </Button>
                        </div>
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
                            <SelectItem value="de">Deutsch</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                    </TabsContent>

                    <TabsContent value="telegram" className="space-y-4 mt-4">
                      <div>
                        <Label>{t.telegramBotIntegration}</Label>
                        <p className="text-sm text-muted-foreground mt-2">
                          {t.telegramBotDescription}
                        </p>
                        <div className="mt-4 space-y-4">
                          <div className="flex items-center space-x-2">
                            <Checkbox 
                              id="telegram-enabled" 
                              checked={settings.telegram.enabled}
                              onCheckedChange={(checked) => 
                                setSettings((prev) => ({
                                  ...prev,
                                  telegram: {
                                    ...prev.telegram,
                                    enabled: checked as boolean
                                  }
                                }))
                              }
                            />
                            <Label htmlFor="telegram-enabled">{t.enableTelegramBot}</Label>
                            {telegramStatus.isLoading && (
                              <span className="text-sm text-muted-foreground">{t.loading}</span>
                            )}
                            {telegramStatus.isRunning && (
                              <span className="text-sm text-green-600">{t.botIsRunning}</span>
                            )}
                            {telegramStatus.error && (
                              <span className="text-sm text-red-600">✗ {telegramStatus.error}</span>
                            )}
                          </div>
                          
                          {settings.telegram.enabled && (
                            <div>
                              <Label htmlFor="telegram-bot-token">{t.botToken}</Label>
                              {hasToken ? (
                                <div>
                                  <Input
                                    id="telegram-bot-token"
                                    value="••••••••••••••••••••••••••••••••"
                                    className="mt-2"
                                    readOnly
                                  />
                                  <div className="flex items-center justify-between mt-1">
                                    <div className="text-xs text-muted-foreground">
                                      {t.tokenSavedAndHidden}
                                    </div>
                                    <Button
                                      variant="outline"
                                      size="sm"
                                      onClick={clearToken}
                                      className="h-6 px-2 text-xs"
                                    >
                                      {t.change}
                                    </Button>
                                  </div>
                                </div>
                              ) : (
                                <div>
                                  <Input
                                    id="telegram-bot-token"
                                    placeholder={t.enterBotToken}
                                    className="mt-2"
                                    onKeyDown={async (e) => {
                                      if (e.key === 'Enter') {
                                        const token = e.currentTarget.value.trim()
                                        if (token) {
                                          const success = await saveToken(token)
                                          if (success) {
                                            e.currentTarget.value = ''
                                          }
                                        }
                                      }
                                    }}
                                  />
                                  <Button
                                    variant="outline"
                                    size="sm"
                                    className="mt-2"
                                    onClick={async () => {
                                      const input = document.getElementById('telegram-bot-token') as HTMLInputElement
                                      const token = input?.value.trim()
                                      if (token) {
                                        const success = await saveToken(token)
                                        if (success) {
                                          input.value = ''
                                        }
                                      }
                                    }}
                                  >
                                    Save Token
                                  </Button>
                                </div>
                              )}
                              <p className="text-xs text-muted-foreground mt-1">
                                {t.getBotTokenFromBotFather}
                              </p>
                            </div>
                          )}
                          
                          {settings.telegram.enabled && hasToken && telegramStatus.isRunning && (
                            <div className="space-y-4">
                              <div className="space-y-2">
                                <div className="flex items-center space-x-2">
                                  <Button 
                                    onClick={startRegistration}
                                    disabled={telegramStatus.isLoading || !!registrationCode}
                                    size="sm"
                                  >
{registrationCode ? t.registrationActive : t.startRegistration}
                                  </Button>
                                  {registrationCode && (
                                    <Button 
                                      onClick={stopRegistration}
                                      variant="destructive"
                                      size="sm"
                                    >
{t.stopRegistration}
                                    </Button>
                                  )}
                                </div>
                                {registrationCode && (
                                  <div className="bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                                    <div className="flex items-center justify-between mb-2">
                                      <h4 className="font-medium text-blue-900 dark:text-blue-100">{t.registrationActive}</h4>
                                      {registrationTimeLeft && (
                                        <span className="text-sm text-blue-700 dark:text-blue-300 bg-blue-100 dark:bg-blue-900 px-2 py-1 rounded">
                                          {registrationTimeLeft}s
                                        </span>
                                      )}
                                    </div>
                                    <div className="space-y-2">
                                      <div className="text-sm text-blue-800 dark:text-blue-200">
                                        Tell users to send this code to the bot:
                                      </div>
                                      <div className="text-2xl font-bold text-blue-600 dark:text-blue-400 font-mono bg-white dark:bg-gray-800 px-3 py-2 rounded border border-gray-300 dark:border-gray-600 text-center">
                                        {registrationCode}
                                      </div>
                                    </div>
                                  </div>
                                )}
                              </div>
                            </div>
                          )}
                          
                          <div className="space-y-2">
                            <div className="flex items-center justify-between">
                              <Label>{t.registeredUsers} ({telegramUsers.length})</Label>
                              <Button 
                                variant="outline" 
                                size="sm" 
                                onClick={loadUsers}
                                disabled={!telegramStatus.isRunning}
                              >
                                {t.refresh}
                              </Button>
                            </div>
                            {telegramUsers.length > 0 && (
                              <div className="space-y-2 max-h-40 overflow-y-auto">
                                {telegramUsers.map((user) => (
                                  <div key={user.user_id} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
                                    <div className="flex-1">
                                      <div className="flex items-center space-x-2">
                                        <span className="font-medium text-gray-900 dark:text-gray-100">
                                          {user.username ? `@${user.username}` : 
                                           user.first_name ? `${user.first_name} ${user.last_name || ''}`.trim() :
                                           `User ${user.user_id}`}
                                        </span>
                                        {user.username && user.first_name && (
                                          <span className="text-sm text-gray-600 dark:text-gray-400">
                                            ({user.first_name} {user.last_name || ''})
                                          </span>
                                        )}
                                      </div>
                                      <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                        ID: {user.user_id} • Registered: {new Date(user.registered_at).toLocaleDateString()}
                                      </div>
                                    </div>
                                    <div className="flex items-center space-x-2">
                                      <div className="flex items-center space-x-1">
                                        <input
                                          type="checkbox"
                                          id={`notifications-${user.user_id}`}
                                          checked={user.notifications_enabled === true}
                                          onChange={(e) => {
                                            updateUserNotifications(user.user_id, e.target.checked);
                                          }}
                                          className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                        />
                                        <label htmlFor={`notifications-${user.user_id}`} className="text-xs text-gray-600 dark:text-gray-400">
                                          Notifications
                                        </label>
                                      </div>
                                      <Button
                                        variant="destructive"
                                        size="sm"
                                        onClick={() => {
                                          console.log('Removing user:', user.user_id);
                                          removeUser(user.user_id);
                                        }}
                                        className="ml-2"
                                      >
                                        {t.remove}
                                      </Button>
                                    </div>
                                  </div>
                                ))}
                              </div>
                            )}
                          </div>
                          
                        </div>
                      </div>
                    </TabsContent>

                    <TabsContent value="about" className="space-y-4 mt-4">
                      <div className="text-center space-y-4">
                        <div className="space-y-2">
                          <h3 className="text-lg font-semibold">{t.networkScanner}</h3>
                          <p className="text-sm text-muted-foreground">{t.version}</p>
                          <p className="text-sm text-muted-foreground">{t.powerfulNetworkTool}</p>
                        </div>

                        {/* Update Information */}
                        <div className="border rounded-lg p-4 space-y-3">
                          <div className="flex items-center justify-between">
                            <span className="text-sm font-medium">{t.currentVersion}:</span>
                            <span className="text-sm text-muted-foreground">
                              {updateInfo?.current_version || '0.0.9'}
                            </span>
                          </div>

                          {updateInfo?.update_available && (
                            <div className="flex items-center justify-between">
                              <span className="text-sm font-medium text-green-600">{t.latestVersion}:</span>
                              <span className="text-sm text-green-600 font-medium">
                                {updateInfo.latest_version}
                              </span>
                            </div>
                          )}

                          <div className="flex items-center justify-between">
                            <span className="text-sm font-medium">{t.lastCheck}:</span>
                            <span className="text-sm text-muted-foreground">
                              {updateInfo?.last_check ? new Date(updateInfo.last_check).toLocaleString() : '-'}
                            </span>
                          </div>

                          {updateInfo?.update_available && (
                            <div className="p-2 bg-green-50 dark:bg-green-900/20 rounded border border-green-200 dark:border-green-800">
                              <p className="text-sm text-green-700 dark:text-green-300 font-medium">
                                {t.updateAvailable}
                              </p>
                            </div>
                          )}

                          {updateInfo?.error && (
                            <div className="p-2 bg-red-50 dark:bg-red-900/20 rounded border border-red-200 dark:border-red-800">
                              <p className="text-sm text-red-700 dark:text-red-300">
                                {t.updateCheckFailed}: {updateInfo.error}
                              </p>
                            </div>
                          )}

                          <div className="flex gap-2 justify-center">
                            <Button
                              onClick={checkForUpdates}
                              disabled={isChecking}
                              variant="outline"
                              size="sm"
                            >
                              {isChecking ? (
                                <>
                                  <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                                  {t.checkingForUpdates}
                                </>
                              ) : (
                                <>
                                  <RefreshCw className="h-4 w-4 mr-2" />
                                  {t.checkForUpdates}
                                </>
                              )}
                            </Button>

                            <Button
                              onClick={openRepository}
                              variant="outline"
                              size="sm"
                            >
                              <ExternalLink className="h-4 w-4 mr-2" />
                              {t.repository}
                            </Button>

                            <Button
                              onClick={openReleases}
                              variant="outline"
                              size="sm"
                            >
                              <ExternalLink className="h-4 w-4 mr-2" />
                              {t.releases}
                            </Button>
                          </div>
                        </div>
                      </div>
                    </TabsContent>
                  </div>
                </Tabs>
              </DialogContent>
            </Dialog>
          </div>
        </div>

        

        {/* Scan Controls and Batch Tasks */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* Scan Controls */}
          <Card>
            <CardHeader>
              <CardTitle>{t.networkScan}</CardTitle>
              <CardDescription>{t.scanConfiguredSubnet}</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center gap-4 flex-wrap">
                <Button onClick={() => handleScan()} disabled={isScanning} className="bg-primary hover:bg-primary/90">
                  {isScanning ? (
                    <>
                      <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                      {t.scanning}
                    </>
                  ) : (
                    <>
                      <Wifi className="h-4 w-4 mr-2" />
                      {t.startScan}
                    </>
                  )}
                </Button>
              </div>
            </CardContent>
          </Card>

          {/* Batch Tasks */}
          <Card>
            <CardHeader>
              <CardTitle>{t.batchTasks}</CardTitle>
              <CardDescription>{t.batchTasksDescription}</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center gap-4 flex-wrap">
                <Button onClick={() => setBatchTasksDialog(true)} className="bg-blue-600 hover:bg-blue-700 text-white">
                  <Layers className="h-4 w-4 mr-2" />
                  {t.openBatchTasks}
                </Button>
                <Button onClick={() => setSendBatchTaskDialog(true)} className="bg-primary hover:bg-primary/90">
                  <Send className="h-4 w-4 mr-2" />
                  {t.sendBatchTask}
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>

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
                  <TableHead>{t.order}</TableHead>
                  <TableHead>{t.delete}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {hosts.sort((a, b) => (a.order || 0) - (b.order || 0)).map((host) => (
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
                          value={host.hostname}
                          onChange={(e) => handleEditHostname(host.id, e.target.value)}
                          className="border-none bg-transparent p-0 h-auto focus-visible:ring-0 flex-1"
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
                      <TableCell>{getStatusBadge(getPrinterStatus(host), host)}</TableCell>
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
                        <div className="flex flex-col gap-1">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => moveHostUp(host.id)}
                            disabled={hosts.sort((a, b) => (a.order || 0) - (b.order || 0)).findIndex(h => h.id === host.id) === 0}
                            className="h-6 w-6 p-0"
                            title="Переместить вверх"
                          >
                            <ChevronUp className="h-3 w-3" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => moveHostDown(host.id)}
                            disabled={hosts.sort((a, b) => (a.order || 0) - (b.order || 0)).findIndex(h => h.id === host.id) === hosts.length - 1}
                            className="h-6 w-6 p-0"
                            title="Переместить вниз"
                          >
                            <ChevronDown className="h-3 w-3" />
                          </Button>
                        </div>
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
                        <TableCell colSpan={8} className="bg-muted/30">
                          <div className="py-4 space-y-2">
                            <div className="flex justify-between items-start">
                              <div>
                                <h4 className="font-medium text-sm">{t.apiControls}</h4>
                                <div className="flex gap-2">
                              {/* Pause/Resume button - shows Resume when printer is paused */}
                              {getPrinterStatus(host) === 'paused' ? (
                                <Button
                                  size="sm"
                                  variant="outline"
                                  onClick={() => handleAPIAction(t.resume, host.id)}
                                  disabled={host.status !== "online" || loadingButtons.has(`${host.id}-${t.resume}`)}
                                  className={`transition-all duration-200 ${
                                    loadingButtons.has(`${host.id}-${t.resume}`) ? 'opacity-75 scale-95' : ''
                                  }`}
                                >
                                  {loadingButtons.has(`${host.id}-${t.resume}`) ? (
                                    <div className="animate-spin h-4 w-4 mr-1 border-2 border-current border-t-transparent rounded-full" />
                                  ) : (
                                    <Play className="h-4 w-4 mr-1" />
                                  )}
                                  {t.resume}
                                </Button>
                              ) : (
                                <Button
                                  size="sm"
                                  variant="outline"
                                  onClick={() => handleAPIAction(t.pause, host.id)}
                                  disabled={host.status !== "online" || loadingButtons.has(`${host.id}-${t.pause}`)}
                                  className={`transition-all duration-200 ${
                                    loadingButtons.has(`${host.id}-${t.pause}`) ? 'opacity-75 scale-95' : ''
                                  }`}
                                >
                                  {loadingButtons.has(`${host.id}-${t.pause}`) ? (
                                    <div className="animate-spin h-4 w-4 mr-1 border-2 border-current border-t-transparent rounded-full" />
                                  ) : (
                                    <Pause className="h-4 w-4 mr-1" />
                                  )}
                                  {t.pause}
                                </Button>
                              )}
                              <Button
                                size="sm"
                                variant="outline"
                                onClick={() => handleAPIAction(t.stop, host.id)}
                                disabled={host.status !== "online" || loadingButtons.has(`${host.id}-${t.stop}`)}
                                className={`transition-all duration-200 ${
                                  loadingButtons.has(`${host.id}-${t.stop}`) ? 'opacity-75 scale-95' : ''
                                }`}
                              >
                                {loadingButtons.has(`${host.id}-${t.stop}`) ? (
                                  <div className="animate-spin h-4 w-4 mr-1 border-2 border-current border-t-transparent rounded-full" />
                                ) : (
                                  <Square className="h-4 w-4 mr-1" />
                                )}
                                {t.stop}
                              </Button>
                              <Button
                                size="sm"
                                variant="destructive"
                                onClick={() => handleAPIAction(t.emergencyStop, host.id)}
                                disabled={host.status !== "online" || loadingButtons.has(`${host.id}-${t.emergencyStop}`)}
                                className={`transition-all duration-200 ${
                                  loadingButtons.has(`${host.id}-${t.emergencyStop}`) ? 'opacity-75 scale-95' : ''
                                }`}
                              >
                                {loadingButtons.has(`${host.id}-${t.emergencyStop}`) ? (
                                  <div className="animate-spin h-4 w-4 mr-1 border-2 border-white border-t-transparent rounded-full" />
                                ) : (
                                  <AlertTriangle className="h-4 w-4 mr-1" />
                                )}
                                {t.emergencyStop}
                              </Button>
                            </div>
                              </div>
                              
                              {/* Print Information */}
                              {host.print_info && (
                                <div className="text-right space-y-1">
                                  <h4 className="font-medium text-sm">{t.printStats}</h4>
                                  <div className="text-xs space-y-1 text-muted-foreground">
                                    <div><span className="font-medium">{t.filename}:</span> {host.print_info.filename}</div>
                                    <div><span className="font-medium">{t.printDuration}:</span> {formatDuration(host.print_info.print_duration)}</div>
                                    <div><span className="font-medium">{t.totalDuration}:</span> {formatDuration(host.print_info.total_duration)}</div>
                                  </div>
                                </div>
                              )}
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
                      title={t.flipHorizontally}
                    >
                      <FlipHorizontal className="h-4 w-4" />
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => setWebcamFlip(prev => ({ ...prev, vertical: !prev.vertical }))}
                      title={t.flipVertically}
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
                      onLoad={() => {
                        const fallback = document.querySelector('.fallback') as HTMLElement;
                        if (fallback) fallback.style.display = 'none';
                      }}
                      onError={(e) => {
                        const target = e.target as HTMLImageElement;
                        // Retry loading after a short delay
                        setTimeout(() => {
                          target.src = getWebcamUrl(webcamDialog.host!, currentWebcamUrlIndex) + '?t=' + Date.now();
                        }, 1000);
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

      {/* Batch Tasks Dialog */}
      <Dialog open={batchTasksDialog} onOpenChange={setBatchTasksDialog}>
        <DialogContent className="max-w-4xl max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{t.batchTasks}</DialogTitle>
            <DialogDescription>
              {t.batchTasksDescription}
            </DialogDescription>
          </DialogHeader>
          <div className="py-4 space-y-6">
            {/* Create New Group Section */}
            <div className="space-y-4">
              <div className="flex items-center gap-4">
                <Input
                  placeholder={t.enterGroupName}
                  value={newGroupName}
                  onChange={(e) => setNewGroupName(e.target.value)}
                  className="flex-1"
                />
                <Button 
                  onClick={handleSaveGroup}
                  disabled={!newGroupName.trim() || selectedHostsForGroup.length === 0}
                  className="bg-primary hover:bg-primary/90"
                >
                  {t.saveGroup}
                </Button>
              </div>
            </div>

            {/* Existing Groups Section */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold">{t.existingGroups}</h3>
              {hostGroups.length === 0 ? (
                <p className="text-gray-500 text-sm">{t.noGroups}</p>
              ) : (
                <div className="space-y-2">
                  {hostGroups.map((group) => (
                    <div key={group.id} className="flex items-center gap-3 p-3 border rounded-lg">
                      <Input
                        value={group.name}
                        onChange={(e) => handleEditGroupName(group.id, e.target.value)}
                        className="flex-1"
                      />
                      <span className="text-sm text-gray-500">
                        {group.hostIds.length} {group.hostIds.length === 1 ? 'host' : 'hosts'}
                      </span>
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => handleDeleteGroup(group.id)}
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Host Selection Section */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold">{t.selectHosts}</h3>
              {hosts.length === 0 ? (
                <p className="text-gray-500 text-sm">No hosts available</p>
              ) : (
                <div className="space-y-2 max-h-60 overflow-y-auto">
                  {hosts.map((host) => (
                    <div key={host.id} className="flex items-center gap-3 p-2 border rounded">
                      <Checkbox
                        checked={selectedHostsForGroup.includes(host.id)}
                        onCheckedChange={(checked) => 
                          handleHostSelectionChange(host.id, checked as boolean)
                        }
                      />
                      <div className="flex-1">
                        <div className="font-medium">{host.hostname}</div>
                        <div className="text-sm text-gray-500">{host.ip_address}</div>
                      </div>
                      <div className={`px-2 py-1 rounded text-xs ${
                        host.status === 'online' 
                          ? 'bg-green-100 text-green-800' 
                          : 'bg-red-100 text-red-800'
                      }`}>
                        {host.status}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </DialogContent>
      </Dialog>

        {/* Send Batch Task Dialog */}
        <Dialog open={sendBatchTaskDialog} onOpenChange={setSendBatchTaskDialog}>
          <DialogContent className="max-w-2xl max-h-[80vh] overflow-hidden">
          <DialogHeader>
            <DialogTitle>{t.sendBatchTask}</DialogTitle>
            <DialogDescription>
              {t.batchTasksDescription}
            </DialogDescription>
          </DialogHeader>
          <div className="py-4 space-y-6 overflow-y-auto max-h-[60vh]">
            {/* File Selection Section */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold">{t.selectGcodeFile}</h3>
              <div className="space-y-2">
                <input
                  type="file"
                  accept=".gcode"
                  onChange={handleFileSelect}
                  className="hidden"
                  id="gcode-file-input"
                />
                <label
                  htmlFor="gcode-file-input"
                  className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg cursor-pointer hover:bg-gray-50 w-fit"
                >
                  <Send className="h-4 w-4" />
                  {t.selectGcodeFile}
                </label>
                {selectedGcodeFile && (
                  <div className="flex items-center gap-2 p-3 bg-green-50 border border-green-200 rounded-lg max-w-full">
                    <span className="text-green-600 text-lg flex-shrink-0">✓</span>
                    <div className="flex-1 min-w-0 overflow-hidden">
                      <p className="text-sm font-medium text-green-800 truncate">
                        {selectedGcodeFile.name}
                      </p>
                      <p className="text-xs text-green-600">
                        {(selectedGcodeFile.size / 1024 / 1024).toFixed(2)} MB
                      </p>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setSelectedGcodeFile(null)}
                      className="text-green-600 hover:text-green-700 hover:bg-green-100 flex-shrink-0"
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                )}
              </div>
            </div>

            {/* Groups Section */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold">{t.existingGroups}</h3>
              {hostGroups.length === 0 ? (
                <p className="text-gray-500 text-sm">{t.noGroups}</p>
              ) : (
                <div className="space-y-3">
                  {hostGroups.map((group) => {
                    const groupHosts = hosts.filter(host => group.hostIds.includes(host.id))
                    const allHostsStandby = groupHosts.every(host => getPrinterStatus(host) === 'standby')
                    
                    return (
                      <div key={group.id} className="border rounded-lg p-4">
                        <div className="flex items-center justify-between mb-3">
                          <div>
                            <h4 className="font-medium">{group.name}</h4>
                            <p className="text-sm text-gray-500">
                              {groupHosts.length} {groupHosts.length === 1 ? 'host' : 'hosts'}
                            </p>
                          </div>
                          <Button
                            onClick={() => handleLaunchBatchTask(group)}
                            disabled={!selectedGcodeFile || isUploading || !allHostsStandby}
                            className="bg-green-600 hover:bg-green-700 text-white"
                          >
                            {isUploading ? (
                              <>
                                <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                                {t.uploadingFiles}
                              </>
                            ) : (
                              <>
                                <Play className="h-4 w-4 mr-2" />
                                {t.launch}
                              </>
                            )}
                          </Button>
                        </div>
                        
                        {/* Host Status */}
                        <div className="space-y-2">
                          {groupHosts.map((host) => {
                            const status = getPrinterStatus(host)
                            const isUploadingToHost = uploadProgress[host.id]
                            
                            return (
                              <div key={host.id} className="flex items-center justify-between text-sm">
                                <div className="flex items-center gap-2">
                                  <span className="font-medium">{host.hostname}</span>
                                  <span className="text-gray-500">({host.ip_address})</span>
                                </div>
                                <div className="flex items-center gap-2">
                                  {isUploadingToHost && (
                                    <RefreshCw className="h-3 w-3 animate-spin text-blue-500" />
                                  )}
                                  <span className={`px-2 py-1 rounded text-xs ${
                                    status === 'standby' 
                                      ? 'bg-green-100 text-green-800' 
                                      : 'bg-red-100 text-red-800'
                                  }`}>
                                    {status}
                                  </span>
                                </div>
                              </div>
                            )
                          })}
                        </div>
                        
                        {!allHostsStandby && (
                          <div className="mt-3 p-2 bg-yellow-50 border border-yellow-200 rounded text-sm text-yellow-800">
                            ⚠️ {t.hostsNotReadyMessage}
                          </div>
                        )}
                      </div>
                    )
                  })}
                </div>
              )}
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
