import { useState, useEffect, useCallback } from 'react'
import { tauriCommands } from '@/lib/tauri'

export interface TelegramUser {
  user_id: number
  username?: string
  first_name?: string
  last_name?: string
  registered_at: string
  notifications_enabled: boolean
}

export interface TelegramBotStatus {
  isRunning: boolean
  isLoading: boolean
  error: string | null
}

export function useTelegramBot(enabled: boolean) {
  const [status, setStatus] = useState<TelegramBotStatus>({
    isRunning: false,
    isLoading: false,
    error: null,
  })
  const [users, setUsers] = useState<TelegramUser[]>([])
  const [registrationCode, setRegistrationCode] = useState<string | null>(null)
  const [registrationTimeLeft, setRegistrationTimeLeft] = useState<number | null>(null)
  const [hasToken, setHasToken] = useState<boolean>(false)

  const checkStatus = useCallback(async () => {
    try {
      const isRunning = await tauriCommands.getTelegramBotStatus()
      setStatus(prev => ({ ...prev, isRunning, error: null }))
      
      if (isRunning) {
        await loadUsers()
      }
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Unknown error',
        isRunning: false 
      }))
    }
  }, [])

  const loadUsers = useCallback(async () => {
    try {
      const users = await tauriCommands.getTelegramUsers()
      setUsers(users)
    } catch (error) {
      console.error('Failed to load users:', error)
    }
  }, [])

  const saveUsersToBackend = useCallback(async (users: TelegramUser[]) => {
    try {
      await tauriCommands.saveTelegramUsers(users)
    } catch (error) {
      console.error('Failed to save users to backend:', error)
    }
  }, [])

  const startRegistration = useCallback(async () => {
    setStatus(prev => ({ ...prev, isLoading: true, error: null }))
    
    try {
      const code = await tauriCommands.startTelegramRegistration()
      setRegistrationCode(code)
      setRegistrationTimeLeft(60) // 60 seconds
      setStatus(prev => ({ ...prev, isLoading: false, error: null }))
    } catch (error) {
      console.error('Failed to start registration:', error);
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to start registration',
        isLoading: false 
      }))
    }
  }, [])

  const stopRegistration = useCallback(async () => {
    try {
      await tauriCommands.stopTelegramRegistration()
      setRegistrationCode(null)
      setRegistrationTimeLeft(null)
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to stop registration'
      }))
    }
  }, [])

  const removeUser = useCallback(async (userId: number) => {
    try {
      // Update local state immediately for better UX
      setUsers(prevUsers => 
        prevUsers.filter(user => user.user_id !== userId)
      );
      
      await tauriCommands.removeTelegramUser(userId)
      
      // Also reload from backend to ensure consistency
      const updatedUsers = await tauriCommands.getTelegramUsers()
      setUsers(updatedUsers)
      await saveUsersToBackend(updatedUsers)
    } catch (error) {
      console.error('Failed to remove user:', error);
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to remove user'
      }))
    }
  }, [saveUsersToBackend])

  const updateUserNotifications = useCallback(async (userId: number, notificationsEnabled: boolean) => {
    try {
      // Update local state immediately for better UX
      setUsers(prevUsers => 
        prevUsers.map(user => 
          user.user_id === userId 
            ? { ...user, notifications_enabled: notificationsEnabled }
            : user
        )
      );
      
      await tauriCommands.updateTelegramUserNotifications(userId, notificationsEnabled)
      
      // Also reload from backend to ensure consistency
      const updatedUsers = await tauriCommands.getTelegramUsers()
      setUsers(updatedUsers)
      await saveUsersToBackend(updatedUsers)
    } catch (error) {
      console.error('Failed to update user notifications:', error);
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to update user notifications'
      }))
    }
  }, [saveUsersToBackend])

  const saveToken = useCallback(async (token: string) => {
    try {
      await tauriCommands.saveTelegramBotToken(token)
      setHasToken(true)
      return true
    } catch (error) {
      console.error('Failed to save token:', error)
      return false
    }
  }, [])

  const clearToken = useCallback(async () => {
    try {
      await tauriCommands.clearTelegramBotToken()
      setHasToken(false)
      // Stop bot if running
      if (status.isRunning) {
        await stopBot()
      }
    } catch (error) {
      console.error('Failed to clear token:', error)
    }
  }, [status.isRunning])

  const checkToken = useCallback(async () => {
    try {
      const token = await tauriCommands.getTelegramBotToken()
      setHasToken(!!token)
    } catch (error) {
      console.error('Failed to check token:', error)
      setHasToken(false)
    }
  }, [])

  const startBot = useCallback(async () => {
    if (!hasToken) {
      setStatus(prev => ({ ...prev, error: 'Bot token is required' }))
      return
    }

    setStatus(prev => ({ ...prev, isLoading: true, error: null }))
    
    try {
      await tauriCommands.startTelegramBot()
      setStatus(prev => ({ ...prev, isRunning: true, isLoading: false, error: null }))
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to start bot',
        isLoading: false,
        isRunning: false 
      }))
    }
  }, [hasToken])

  const stopBot = useCallback(async () => {
    setStatus(prev => ({ ...prev, isLoading: true, error: null }))
    
    try {
      await tauriCommands.stopTelegramBot()
      setStatus(prev => ({ ...prev, isRunning: false, isLoading: false, error: null }))
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to stop bot',
        isLoading: false 
      }))
    }
  }, [])

  // Load settings and check token on mount
  useEffect(() => {
    const loadSettings = async () => {
      try {
        await tauriCommands.loadTelegramSettings()
        await checkToken()
      } catch (error) {
        console.error('Failed to load Telegram settings:', error)
      }
    }
    loadSettings()
  }, [checkToken])

  // Auto-start/stop bot when enabled state changes
  useEffect(() => {
    if (enabled && hasToken) {
      startBot()
    } else if (!enabled && status.isRunning) {
      stopBot()
    }
  }, [enabled, hasToken, startBot, stopBot, status.isRunning])

  // Auto-refresh users list when bot is running
  useEffect(() => {
    if (status.isRunning) {
      const interval = setInterval(async () => {
        await loadUsers()
        
        // Check if registration is still active
        if (registrationCode) {
          try {
            const isActive = await tauriCommands.isTelegramRegistrationActive()
            if (!isActive) {
              setRegistrationCode(null)
              setRegistrationTimeLeft(null)
              // Force reload users when registration completes
              const updatedUsers = await tauriCommands.getTelegramUsers()
              setUsers(updatedUsers)
              await saveUsersToBackend(updatedUsers)
            }
          } catch (error) {
            console.error('Failed to check registration status:', error)
          }
        }
      }, 2000) // Check every 2 seconds

      return () => clearInterval(interval)
    }
  }, [status.isRunning, loadUsers, registrationCode, saveUsersToBackend])

  // Function to sync hosts with Telegram bot
  const syncHostsWithBot = useCallback(async (hosts: any[]) => {
    try {
      await tauriCommands.updateTelegramHosts(hosts)
    } catch (error) {
      console.error('Failed to sync hosts with Telegram bot:', error)
    }
  }, [])

  // Registration timer
  useEffect(() => {
    if (registrationTimeLeft && registrationTimeLeft > 0) {
      const timer = setTimeout(() => {
        setRegistrationTimeLeft(prev => {
          if (prev && prev <= 1) {
            // Timer expired, stop registration
            stopRegistration()
            return null
          }
          return prev ? prev - 1 : null
        })
      }, 1000)

      return () => clearTimeout(timer)
    }
  }, [registrationTimeLeft, stopRegistration])

  // Check status on mount
  useEffect(() => {
    checkStatus()
    loadUsers()
  }, [checkStatus, loadUsers])

  return {
    status,
    users,
    registrationCode,
    registrationTimeLeft,
    hasToken,
    startBot,
    stopBot,
    checkStatus,
    startRegistration,
    stopRegistration,
    removeUser,
    loadUsers,
    syncHostsWithBot,
    updateUserNotifications,
    saveToken,
    clearToken,
  }
}
