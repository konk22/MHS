import { useCallback, useRef } from 'react'
import { invokeTauri } from '@/lib/tauri'
import { HostInfo } from './useHosts'

interface NotificationState {
  lastNotificationTime: number
  notificationCount: number
  isInTimeout: boolean
  timeoutUntil: number
  statusHistory: Array<{ status: string; timestamp: number }>
}

interface SmartNotificationConfig {
  // Максимальное количество одинаковых уведомлений за период
  maxDuplicateNotifications: number
  // Период для подсчета дублирующихся уведомлений (в миллисекундах)
  duplicateCheckPeriod: number
  // Таймаут для нестабильных хостов (в миллисекундах)
  unstableHostTimeout: number
  // Количество быстрых переключений для активации таймаута
  maxQuickSwitches: number
  // Период для определения "быстрых переключений" (в миллисекундах)
  quickSwitchPeriod: number
}

const DEFAULT_CONFIG: SmartNotificationConfig = {
  maxDuplicateNotifications: 3,
  duplicateCheckPeriod: 30000, // 30 секунд
  unstableHostTimeout: 300000, // 5 минут
  maxQuickSwitches: 3,
  quickSwitchPeriod: 60000, // 1 минута
}

export function useSmartNotifications() {
  const notificationStates = useRef<Map<string, NotificationState>>(new Map())
  const globalOfflineCount = useRef<number>(0)
  const lastGlobalOfflineTime = useRef<number>(0)

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
    if (host.failed_attempts && host.failed_attempts >= 8) {
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

  const isHostUnstable = useCallback((hostId: string, newStatus: string): boolean => {
    const state = notificationStates.current.get(hostId)
    if (!state) return false

    const now = Date.now()
    
    // Добавляем новый статус в историю
    state.statusHistory.push({ status: newStatus, timestamp: now })
    
    // Очищаем старую историю (старше quickSwitchPeriod)
    state.statusHistory = state.statusHistory.filter(
      entry => now - entry.timestamp <= DEFAULT_CONFIG.quickSwitchPeriod
    )
    
    // Подсчитываем переключения между online/offline
    const onlineOfflineSwitches = state.statusHistory.reduce((count, entry, index) => {
      if (index === 0) return count
      const prevEntry = state.statusHistory[index - 1]
      const isSwitch = (entry.status === 'offline' && prevEntry.status !== 'offline') ||
                      (entry.status !== 'offline' && prevEntry.status === 'offline')
      return count + (isSwitch ? 1 : 0)
    }, 0)
    
    return onlineOfflineSwitches >= DEFAULT_CONFIG.maxQuickSwitches
  }, [])

  const shouldSendNotification = useCallback((
    hostId: string, 
    newStatus: string, 
    allHosts: HostInfo[]
  ): boolean => {
    const now = Date.now()
    let state = notificationStates.current.get(hostId)
    
    if (!state) {
      state = {
        lastNotificationTime: 0,
        notificationCount: 0,
        isInTimeout: false,
        timeoutUntil: 0,
        statusHistory: []
      }
      notificationStates.current.set(hostId, state)
    }

    // Проверяем, не истек ли таймаут
    if (state.isInTimeout && now < state.timeoutUntil) {
      console.log(`Host ${hostId} is in notification timeout until ${new Date(state.timeoutUntil)}`)
      return false
    }

    // Сбрасываем таймаут если он истек
    if (state.isInTimeout && now >= state.timeoutUntil) {
      state.isInTimeout = false
      state.timeoutUntil = 0
      console.log(`Host ${hostId} notification timeout expired`)
    }

    // Проверяем, нестабилен ли хост
    if (isHostUnstable(hostId, newStatus)) {
      state.isInTimeout = true
      state.timeoutUntil = now + DEFAULT_CONFIG.unstableHostTimeout
      console.log(`Host ${hostId} marked as unstable, notifications disabled for ${DEFAULT_CONFIG.unstableHostTimeout / 1000} seconds`)
      return false
    }

    // Проверяем глобальное отключение (все хосты offline одновременно)
    if (newStatus === 'offline') {
      const offlineHosts = allHosts.filter(host => getPrinterStatus(host) === 'offline')
      
      // Если большинство хостов offline, это проблема сети
      if (offlineHosts.length >= Math.ceil(allHosts.length * 0.7)) {
        const timeSinceLastGlobalOffline = now - lastGlobalOfflineTime.current
        
        // Если это происходит впервые за последние 30 секунд
        if (timeSinceLastGlobalOffline > DEFAULT_CONFIG.duplicateCheckPeriod) {
          lastGlobalOfflineTime.current = now
          globalOfflineCount.current = 1
          console.log('Global network issue detected, sending single notification')
          return true
        } else {
          globalOfflineCount.current++
          console.log(`Global network issue continues (${globalOfflineCount.current} hosts offline), suppressing individual notifications`)
          return false
        }
      }
    }

    // Проверяем дублирующиеся уведомления для конкретного хоста
    const timeSinceLastNotification = now - state.lastNotificationTime
    
    if (timeSinceLastNotification <= DEFAULT_CONFIG.duplicateCheckPeriod) {
      state.notificationCount++
      
      if (state.notificationCount >= DEFAULT_CONFIG.maxDuplicateNotifications) {
        console.log(`Host ${hostId} has too many notifications (${state.notificationCount}), suppressing`)
        return false
      }
    } else {
      // Сбрасываем счетчик если прошло достаточно времени
      state.notificationCount = 1
    }

    state.lastNotificationTime = now
    return true
  }, [getPrinterStatus, isHostUnstable])

  const checkStatusChangeAndNotify = useCallback((
    oldHost: HostInfo, 
    newHost: HostInfo, 
    allHosts: HostInfo[],
    notifications: Record<string, boolean>,
    t: any
  ) => {
    const oldStatus = getPrinterStatus(oldHost)
    const newStatus = getPrinterStatus(newHost)
    
    if (oldStatus !== newStatus) {
      // Проверяем, нужно ли отправлять уведомление
      if (!shouldSendNotification(newHost.id, newStatus, allHosts)) {
        return
      }

      // Check if notifications are enabled for this status
      const statusKey = newStatus as keyof typeof notifications
      const notificationEnabled = notifications[statusKey];
      
      if (notificationEnabled) {
        const title = `${t.networkScanner} - ${oldHost.hostname}`
        const body = `${t.status}: ${t[statusKey as keyof typeof t] || newStatus}`
        
        sendNotification(title, body)
      }
    }
  }, [getPrinterStatus, shouldSendNotification, sendNotification])

  const resetHostTimeout = useCallback((hostId: string) => {
    const state = notificationStates.current.get(hostId)
    if (state) {
      state.isInTimeout = false
      state.timeoutUntil = 0
      state.statusHistory = []
      console.log(`Host ${hostId} notification timeout reset`)
    }
  }, [])

  const getHostNotificationState = useCallback((hostId: string) => {
    return notificationStates.current.get(hostId)
  }, [])

  return {
    sendNotification,
    getPrinterStatus,
    checkStatusChangeAndNotify,
    resetHostTimeout,
    getHostNotificationState
  }
}
