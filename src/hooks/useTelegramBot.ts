import { useState, useEffect, useCallback } from 'react'
import { tauriCommands } from '@/lib/tauri'

export interface TelegramUser {
  user_id: number
  username?: string
  first_name?: string
  last_name?: string
  registered_at: string
}

export interface TelegramBotStatus {
  isRunning: boolean
  isLoading: boolean
  error: string | null
}

export function useTelegramBot(botToken: string, enabled: boolean) {
  const [status, setStatus] = useState<TelegramBotStatus>({
    isRunning: false,
    isLoading: false,
    error: null,
  })
  const [users, setUsers] = useState<TelegramUser[]>([])
  const [registrationCode, setRegistrationCode] = useState<string | null>(null)
  const [registrationTimeLeft, setRegistrationTimeLeft] = useState<number | null>(null)

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
      // Save to localStorage
      localStorage.setItem('telegram_users', JSON.stringify(users))
    } catch (error) {
      console.error('Failed to load users:', error)
      // Try to load from localStorage as fallback
      try {
        const stored = localStorage.getItem('telegram_users')
        if (stored) {
          const users = JSON.parse(stored)
          setUsers(users)
        }
      } catch (e) {
        console.error('Failed to load users from localStorage:', e)
      }
    }
  }, [])

  const saveUsersToLocalStorage = useCallback((users: TelegramUser[]) => {
    try {
      localStorage.setItem('telegram_users', JSON.stringify(users))
    } catch (error) {
      console.error('Failed to save users to localStorage:', error)
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
      await tauriCommands.removeTelegramUser(userId)
      const updatedUsers = await tauriCommands.getTelegramUsers()
      setUsers(updatedUsers)
      saveUsersToLocalStorage(updatedUsers)
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to remove user'
      }))
    }
  }, [saveUsersToLocalStorage])

  const startBot = useCallback(async () => {
    if (!botToken.trim()) {
      setStatus(prev => ({ ...prev, error: 'Bot token is required' }))
      return
    }

    setStatus(prev => ({ ...prev, isLoading: true, error: null }))
    
    try {
      await tauriCommands.startTelegramBot(botToken)
      setStatus(prev => ({ ...prev, isRunning: true, isLoading: false, error: null }))
    } catch (error) {
      setStatus(prev => ({ 
        ...prev, 
        error: error instanceof Error ? error.message : 'Failed to start bot',
        isLoading: false,
        isRunning: false 
      }))
    }
  }, [botToken])

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

  // Auto-start/stop bot when enabled state changes
  useEffect(() => {
    if (enabled && botToken.trim()) {
      startBot()
    } else if (!enabled && status.isRunning) {
      stopBot()
    }
  }, [enabled, botToken, startBot, stopBot, status.isRunning])

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
              saveUsersToLocalStorage(updatedUsers)
            }
          } catch (error) {
            console.error('Failed to check registration status:', error)
          }
        }
      }, 2000) // Check every 2 seconds

      return () => clearInterval(interval)
    }
  }, [status.isRunning, loadUsers, registrationCode, saveUsersToLocalStorage])

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
    
    // Load users from localStorage on mount
    try {
      const stored = localStorage.getItem('telegram_users')
      if (stored) {
        const users = JSON.parse(stored)
        setUsers(users)
      }
    } catch (e) {
      console.error('Failed to load users from localStorage on mount:', e)
    }
  }, [checkStatus])

  return {
    status,
    users,
    registrationCode,
    registrationTimeLeft,
    startBot,
    stopBot,
    checkStatus,
    startRegistration,
    stopRegistration,
    removeUser,
    loadUsers,
  }
}
