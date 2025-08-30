import { useState, useCallback, useEffect, useRef } from 'react'
import { invokeTauri } from '@/lib/tauri'
import { HostInfo } from './useHosts'

export function useHostStatus() {
  const [isRefreshing, setIsRefreshing] = useState(false)
  const refreshIntervalRef = useRef<NodeJS.Timeout | null>(null)

  const checkHostStatus = useCallback(async (ip: string) => {
    try {
      return await invokeTauri('check_host_status_command', { ip })
    } catch (error) {
      console.error('Failed to check host status:', error)
      return null
    }
  }, [])

  const refreshHostsStatus = useCallback(async (hosts: HostInfo[]) => {
    if (hosts.length === 0 || isRefreshing) return

    setIsRefreshing(true)

    try {
      const updatedHosts = await Promise.all(
        hosts.map(async (host) => {
          const result = await checkHostStatus(host.ip_address)
          
          if (result?.success) {
            return {
              ...host,
              hostname: host.hostname,
              original_hostname: host.original_hostname,
              status: result.status as "online" | "offline",
              device_status: result.device_status || host.device_status,
              moonraker_version: result.moonraker_version || host.moonraker_version,
              klippy_state: result.klippy_state || host.klippy_state,
              printer_state: result.printer_state || host.printer_state,
              printer_flags: result.printer_flags || host.printer_flags,
              last_seen: new Date().toISOString(),
              failed_attempts: 0
            }
          } else {
            const currentFailedAttempts = host.failed_attempts || 0
            const newFailedAttempts = currentFailedAttempts + 1
            const shouldMarkOffline = newFailedAttempts >= 3

            return {
              ...host,
              hostname: host.hostname,
              original_hostname: host.original_hostname,
              status: shouldMarkOffline ? 'offline' : 'online',
              device_status: shouldMarkOffline ? 'offline' : host.device_status,
              last_seen: new Date().toISOString(),
              failed_attempts: newFailedAttempts
            }
          }
        })
      )

      return updatedHosts
    } catch (error) {
      console.error('Failed to refresh hosts status:', error)
      return hosts
    } finally {
      setIsRefreshing(false)
    }
  }, [checkHostStatus, isRefreshing])

  const startAutoRefresh = useCallback((callback: () => void, interval: number) => {
    stopAutoRefresh()
    refreshIntervalRef.current = setInterval(callback, interval * 1000)
  }, [])

  const stopAutoRefresh = useCallback(() => {
    if (refreshIntervalRef.current) {
      clearInterval(refreshIntervalRef.current)
      refreshIntervalRef.current = null
    }
  }, [])

  useEffect(() => {
    return () => {
      stopAutoRefresh()
    }
  }, [stopAutoRefresh])

  return {
    isRefreshing,
    checkHostStatus,
    refreshHostsStatus,
    startAutoRefresh,
    stopAutoRefresh
  }
}
