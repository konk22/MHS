import React from 'react'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { HostInfo } from '@/hooks/useHosts'
import { useNotifications } from '@/hooks/useNotifications'
import { tauriCommands } from '@/lib/tauri'
import {
  Terminal,
  Camera,
  ExternalLink,
  Trash2,
  Play,
  Pause,
  Square,
  AlertTriangle,
} from "lucide-react"

interface HostTableProps {
  hosts: HostInfo[]
  onDeleteHost: (id: string) => void
  onUpdateHostname: (id: string, hostname: string) => void
  t: any
}

export function HostTable({ hosts, onDeleteHost, onUpdateHostname, t }: HostTableProps) {
  const { getPrinterStatus } = useNotifications()

  const handleControlPrinter = async (ip: string, action: string) => {
    try {
      await tauriCommands.controlPrinter(ip, action)
    } catch (error) {
      console.error(`Failed to ${action} printer:`, error)
    }
  }

  const handleOpenSSH = async (ip: string) => {
    try {
      await tauriCommands.openSSHConnection(ip, 'pi')
    } catch (error) {
      console.error('Failed to open SSH:', error)
    }
  }

  const handleOpenBrowser = async (ip: string) => {
    try {
      await tauriCommands.openHostInBrowser(ip)
    } catch (error) {
      console.error('Failed to open browser:', error)
    }
  }

  const handleOpenWebcam = async (ip: string) => {
    try {
      await tauriCommands.openWebcam(ip)
    } catch (error) {
      console.error('Failed to open webcam:', error)
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'printing': return 'bg-green-500'
      case 'paused': return 'bg-yellow-500'
      case 'error': return 'bg-red-500'
      case 'cancelling': return 'bg-orange-500'
      case 'offline': return 'bg-gray-500'
      default: return 'bg-blue-500'
    }
  }

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{t.hostname}</TableHead>
          <TableHead>{t.ipAddress}</TableHead>
          <TableHead>{t.status}</TableHead>
          <TableHead>{t.actions}</TableHead>
          <TableHead>{t.control}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {hosts.map((host) => {
          const status = getPrinterStatus(host)
          return (
            <TableRow key={host.id}>
              <TableCell>
                <div className="flex items-center space-x-2">
                  <span className="font-medium">{host.hostname}</span>
                  {host.hostname !== host.original_hostname && (
                    <Badge variant="secondary" className="text-xs">
                      {t.custom}
                    </Badge>
                  )}
                </div>
              </TableCell>
              <TableCell>
                <Button
                  variant="link"
                  className="p-0 h-auto font-mono text-sm"
                  onClick={() => handleOpenBrowser(host.ip_address)}
                >
                  {host.ip_address}
                  <ExternalLink className="ml-1 h-3 w-3" />
                </Button>
              </TableCell>
              <TableCell>
                <div className="flex items-center space-x-2">
                  <div className={`w-2 h-2 rounded-full ${getStatusColor(status)}`} />
                  <Badge variant="outline">
                    {t[status as keyof typeof t] || status}
                  </Badge>
                </div>
              </TableCell>
              <TableCell>
                <div className="flex items-center space-x-1">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleOpenSSH(host.ip_address)}
                    title={t.openSSH}
                  >
                    <Terminal className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleOpenWebcam(host.ip_address)}
                    title={t.openWebcam}
                  >
                    <Camera className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => onDeleteHost(host.id)}
                    title={t.delete}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </TableCell>
              <TableCell>
                <div className="flex items-center space-x-1">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleControlPrinter(host.ip_address, 'start')}
                    disabled={status === 'printing'}
                    title={t.start}
                  >
                    <Play className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleControlPrinter(host.ip_address, 'pause')}
                    disabled={status !== 'printing'}
                    title={t.pause}
                  >
                    <Pause className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleControlPrinter(host.ip_address, 'stop')}
                    disabled={status !== 'printing' && status !== 'paused'}
                    title={t.stop}
                  >
                    <Square className="h-4 w-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleControlPrinter(host.ip_address, 'emergency_stop')}
                    title={t.emergencyStop}
                  >
                    <AlertTriangle className="h-4 w-4" />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          )
        })}
      </TableBody>
    </Table>
  )
}
