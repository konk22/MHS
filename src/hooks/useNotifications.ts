import { useCallback } from 'react'
import { invokeTauri } from '@/lib/tauri'
import { HostInfo } from './useHosts'

export function useNotifications() {
  const sendNotification = useCallback(async (title: string, body: string) => {
    try {
      await invokeTauri('send_system_notification_command', { title, body })
    } catch (error) {
      console.error('Failed to send notification:', error)
    }
  }, [])

  const getPrinterStatus = useCallback((host: HostInfo): string => {
    // First check if host is marked as offline
    if (host.status === 'offline') {
      return 'offline'
    }
    
    // Check if we have too many failed attempts (host is effectively offline)
    if (host.failed_attempts && host.failed_attempts >= 5) {
      return 'offline'
    }
    
    // Check if Klippy is completely disconnected (not just in error state)
    if (host.klippy_state === 'disconnected') {
      return 'offline'
    }
    
    // If no printer flags, check if we have any device status
    if (!host.printer_flags) {
      if (host.device_status === 'offline' || host.device_status === 'klippy_disconnected') {
        return 'offline'
      }
      // If Klippy is in error state but host responds, show error status
      if (host.klippy_state === 'error') {
        return 'error'
      }
      return 'standby'
    }
    
    const flags = host.printer_flags
    
    // Priority order: cancelling > error > paused > printing > ready > standby
    if (flags.cancelling) {
      return 'cancelling'
    }
    if (flags.error) {
      return 'error'
    }
    if (flags.paused) {
      return 'paused'
    }
    if (flags.printing) {
      return 'printing'
    }
    if (flags.ready) {
      return 'standby'
    }
    
    return 'standby'
  }, [])

  const checkStatusChangeAndNotify = useCallback((
    oldHost: HostInfo, 
    newHost: HostInfo, 
    notifications: Record<string, boolean>,
    t: any
  ) => {
    const oldStatus = getPrinterStatus(oldHost)
    const newStatus = getPrinterStatus(newHost)
    
    if (oldStatus !== newStatus) {
      // Get current settings to ensure we have the latest notification preferences
      const currentSettings = JSON.parse(localStorage.getItem('networkScanner_settings') || '{}')
      const currentNotifications = currentSettings.notifications || notifications
      
      // Check if notifications are enabled for this status
      const statusKey = newStatus as keyof typeof currentNotifications
      const notificationEnabled = currentNotifications[statusKey];
      
      if (notificationEnabled) {
        const title = `${t.networkScanner} - ${oldHost.hostname}`
        const body = `${t.status}: ${t[statusKey as keyof typeof t] || newStatus}`
        
        sendNotification(title, body)
      }
    }
  }, [getPrinterStatus, sendNotification])

  return {
    sendNotification,
    getPrinterStatus,
    checkStatusChangeAndNotify
  }
}
