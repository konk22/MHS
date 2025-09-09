import { useState, useEffect, useCallback } from 'react'
import { tauriCommands } from '@/lib/tauri'

export interface BackgroundModeSettings {
  enabled: boolean
  intervalSeconds: number
  isRunning: boolean
}

export function useBackgroundMode() {
  const [settings, setSettings] = useState<BackgroundModeSettings>({
    enabled: false,
    intervalSeconds: 30,
    isRunning: false,
  })

  // Load settings from localStorage
  useEffect(() => {
    const savedSettings = localStorage.getItem('backgroundMode_settings')
    if (savedSettings) {
      try {
        const parsed = JSON.parse(savedSettings)
        setSettings(prev => ({ ...prev, ...parsed }))
      } catch (error) {
        console.error('Failed to load background mode settings:', error)
      }
    }
  }, [])

  // Save settings to localStorage
  useEffect(() => {
    localStorage.setItem('backgroundMode_settings', JSON.stringify(settings))
  }, [settings])

  // Check background monitoring status
  const checkStatus = useCallback(async () => {
    try {
      const isRunning = await tauriCommands.getBackgroundMonitoringStatus()
      setSettings(prev => ({ ...prev, isRunning }))
    } catch (error) {
      console.error('Failed to check background monitoring status:', error)
    }
  }, [])

  // Start background monitoring
  const startMonitoring = useCallback(async () => {
    try {
      await tauriCommands.startBackgroundMonitoring(settings.intervalSeconds)
      setSettings(prev => ({ ...prev, isRunning: true }))
    } catch (error) {
      console.error('Failed to start background monitoring:', error)
      throw error
    }
  }, [settings.intervalSeconds])

  // Stop background monitoring
  const stopMonitoring = useCallback(async () => {
    try {
      await tauriCommands.stopBackgroundMonitoring()
      setSettings(prev => ({ ...prev, isRunning: false }))
    } catch (error) {
      console.error('Failed to stop background monitoring:', error)
      throw error
    }
  }, [])

  // Toggle background monitoring
  const toggleMonitoring = useCallback(async () => {
    if (settings.isRunning) {
      await stopMonitoring()
    } else {
      await startMonitoring()
    }
  }, [settings.isRunning, startMonitoring, stopMonitoring])

  // Update settings
  const updateSettings = useCallback((newSettings: Partial<BackgroundModeSettings>) => {
    setSettings(prev => ({ ...prev, ...newSettings }))
  }, [])

  // Check status on mount
  useEffect(() => {
    checkStatus()
  }, [checkStatus])

  return {
    settings,
    startMonitoring,
    stopMonitoring,
    toggleMonitoring,
    updateSettings,
    checkStatus,
  }
}
