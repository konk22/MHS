import { useState, useCallback } from 'react'
import { invokeTauri } from '@/lib/tauri'

export function useNetworkScanner() {
  const [isScanning, setIsScanning] = useState(false)
  const [scanProgress, setScanProgress] = useState(0)
  const [scanResults, setScanResults] = useState<any[]>([])

  const scanNetwork = useCallback(async (subnets: string[]) => {
    if (isScanning) return

    setIsScanning(true)
    setScanProgress(0)
    setScanResults([])

    try {
      const results = await invokeTauri('scan_network_command', { subnets })
      setScanResults(results || [])
    } catch (error) {
      console.error('Network scan failed:', error)
    } finally {
      setIsScanning(false)
      setScanProgress(100)
    }
  }, [isScanning])

  const getHostInfo = useCallback(async (ip: string) => {
    try {
      return await invokeTauri('get_host_info_command', { ip })
    } catch (error) {
      console.error('Failed to get host info:', error)
      return null
    }
  }, [])

  return {
    isScanning,
    scanProgress,
    scanResults,
    scanNetwork,
    getHostInfo
  }
}
