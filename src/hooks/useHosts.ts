import { useState, useEffect, useCallback, useMemo } from 'react'

export interface HostInfo {
  id: string
  hostname: string
  original_hostname: string
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
  failed_attempts?: number
}

export function useHosts() {
  const [hosts, setHosts] = useState<HostInfo[]>([])
  const [isLoaded, setIsLoaded] = useState(false)

  // Load hosts from localStorage
  useEffect(() => {
    const savedHosts = localStorage.getItem('networkScanner_hosts')
    if (savedHosts) {
      try {
        const parsed = JSON.parse(savedHosts)
        // Ensure backward compatibility
        const hostsWithOriginal = parsed.map((host: any) => ({
          ...host,
          original_hostname: host.original_hostname || host.hostname
        }))
        setHosts(hostsWithOriginal)
      } catch (error) {
        console.error('Failed to load hosts:', error)
      }
    }
    setIsLoaded(true)
  }, [])

  // Save hosts to localStorage
  useEffect(() => {
    if (isLoaded) {
      localStorage.setItem('networkScanner_hosts', JSON.stringify(hosts))
    }
  }, [hosts, isLoaded])

  // Computed values
  const onlineHosts = useMemo(() => 
    hosts.filter(host => host.status === 'online').length, 
    [hosts]
  )

  const offlineHosts = useMemo(() => 
    hosts.filter(host => host.status === 'offline').length, 
    [hosts]
  )

  // Add host
  const addHost = useCallback((host: Omit<HostInfo, 'id'>) => {
    const newHost: HostInfo = {
      ...host,
      id: Date.now().toString(),
      last_seen: new Date().toISOString(),
      failed_attempts: 0
    }
    setHosts(prev => [...prev, newHost])
  }, [])

  // Update host
  const updateHost = useCallback((id: string, updates: Partial<HostInfo>) => {
    setHosts(prev => prev.map(host => 
      host.id === id ? { ...host, ...updates } : host
    ))
  }, [])

  // Remove host
  const removeHost = useCallback((id: string) => {
    setHosts(prev => prev.filter(host => host.id !== id))
  }, [])

  // Update multiple hosts
  const updateHosts = useCallback((updates: HostInfo[]) => {
    setHosts(prev => prev.map(host => {
      const update = updates.find(u => u.id === host.id)
      return update ? { ...host, ...update } : host
    }))
  }, [])

  // Clear all hosts
  const clearHosts = useCallback(() => {
    setHosts([])
  }, [])

  // Get host by ID
  const getHostById = useCallback((id: string) => {
    return hosts.find(host => host.id === id)
  }, [hosts])

  // Get hosts by status
  const getHostsByStatus = useCallback((status: "online" | "offline") => {
    return hosts.filter(host => host.status === status)
  }, [hosts])

  return {
    hosts,
    onlineHosts,
    offlineHosts,
    isLoaded,
    addHost,
    updateHost,
    updateHosts,
    removeHost,
    clearHosts,
    getHostById,
    getHostsByStatus
  }
}
