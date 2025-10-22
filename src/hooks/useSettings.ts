import { useState, useEffect, useCallback } from 'react'

export interface AppSettings {
  // Network settings
  subnets: Array<{
    id: string
    range: string
    name: string
    enabled: boolean
  }>
  
  // Auto-refresh settings
  autoRefresh: boolean
  refreshInterval: number
  
  // Notification settings
  notifications: {
    printing: boolean
    paused: boolean
    cancelling: boolean
    error: boolean
    standby: boolean
    offline: boolean
  }
  
  // UI settings
  theme: "light" | "dark" | "system"
  language: "en" | "ru"
  
  // SSH settings
  defaultSSHUser: string
}

const DEFAULT_SETTINGS: AppSettings = {
  subnets: [
    { id: "1", range: "192.168.1.0/24", name: "Home Network", enabled: true }
  ],
  autoRefresh: true,
  refreshInterval: 3,
  notifications: {
    printing: true,
    paused: true,
    cancelling: true,
    error: true,
    standby: false,
    offline: true
  },
  theme: "system",
  language: "en",
  defaultSSHUser: "pi"
}

export function useSettings() {
  const [settings, setSettings] = useState<AppSettings>(DEFAULT_SETTINGS)
  const [isLoaded, setIsLoaded] = useState(false)

  // Load settings from localStorage
  useEffect(() => {
    const savedSettings = localStorage.getItem('networkScanner_settings')
    if (savedSettings) {
      try {
        const parsed = JSON.parse(savedSettings)
        setSettings(prev => ({ ...prev, ...parsed }))
      } catch (error) {
        console.error('Failed to load settings:', error)
      }
    }
    setIsLoaded(true)
  }, [])

  // Save settings to localStorage
  useEffect(() => {
    if (isLoaded) {
      localStorage.setItem('networkScanner_settings', JSON.stringify(settings))
    }
  }, [settings, isLoaded])

  // Update specific setting
  const updateSetting = useCallback(<K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ) => {
    setSettings(prev => ({ ...prev, [key]: value }))
  }, [])

  // Update notification setting
  const updateNotificationSetting = useCallback((
    key: keyof AppSettings['notifications'],
    value: boolean
  ) => {
    setSettings(prev => ({
      ...prev,
      notifications: {
        ...prev.notifications,
        [key]: value
      }
    }))
  }, [])

  // Add subnet
  const addSubnet = useCallback((range: string, name: string) => {
    const newSubnet = {
      id: Date.now().toString(),
      range,
      name,
      enabled: true
    }
    setSettings(prev => ({
      ...prev,
      subnets: [...prev.subnets, newSubnet]
    }))
  }, [])

  // Remove subnet
  const removeSubnet = useCallback((id: string) => {
    setSettings(prev => ({
      ...prev,
      subnets: prev.subnets.filter(subnet => subnet.id !== id)
    }))
  }, [])

  // Toggle subnet enabled state
  const toggleSubnet = useCallback((id: string) => {
    setSettings(prev => ({
      ...prev,
      subnets: prev.subnets.map(subnet =>
        subnet.id === id ? { ...subnet, enabled: !subnet.enabled } : subnet
      )
    }))
  }, [])

  return {
    settings,
    isLoaded,
    updateSetting,
    updateNotificationSetting,
    addSubnet,
    removeSubnet,
    toggleSubnet
  }
}
