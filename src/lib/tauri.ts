// Tauri API wrapper for better error handling and type safety

declare global {
  interface Window {
    __TAURI__: {
      invoke: (command: string, args?: any) => Promise<any>
    }
  }
}

export const invokeTauri = async (command: string, args?: any): Promise<any> => {
  if (typeof window !== 'undefined' && window.__TAURI__) {
    try {
      return await window.__TAURI__.invoke(command, args)
    } catch (error) {
      console.error(`Tauri command failed: ${command}`, error)
      throw error
    }
  } else {
    throw new Error('Tauri API not available')
  }
}

// Predefined commands for better type safety
export const tauriCommands = {
  // Network scanning
  scanNetwork: (subnets: string[]) => invokeTauri('scan_network_command', { subnets }),
  getHostInfo: (ip: string) => invokeTauri('get_host_info_command', { ip }),
  checkHostStatus: (ip: string) => invokeTauri('check_host_status_command', { ip }),
  
  // Printer control
  controlPrinter: (ip: string, action: string) => invokeTauri('control_printer_command', { ip, action }),
  getPrinterStatus: (ip: string) => invokeTauri('get_printer_status_command', { ip }),
  
  // System operations
  openWebcam: (ip: string) => invokeTauri('open_webcam_command', { ip }),
  openHostInBrowser: (ip: string) => invokeTauri('open_host_in_browser_command', { ip }),
  openSSHConnection: (ip: string, user: string) => invokeTauri('open_ssh_connection_command', { ip, user }),
  sendNotification: (title: string, body: string) => invokeTauri('send_system_notification_command', { title, body }),
  
  // Background monitoring
  startBackgroundMonitoring: (intervalSeconds: number) => invokeTauri('start_background_monitoring_command', { intervalSeconds }),
  stopBackgroundMonitoring: () => invokeTauri('stop_background_monitoring_command'),
  getBackgroundMonitoringStatus: () => invokeTauri('get_background_monitoring_status_command')
} as const
