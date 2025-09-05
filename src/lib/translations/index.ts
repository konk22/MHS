import { en } from './en'
import { ru } from './ru'
import { de } from './de'

export interface Translations {
  // Header
  networkScanner: string
  discoverHosts: string
  settings: string

  // Stats Cards
  totalHosts: string
  discoveredInNetwork: string
  onlineHosts: string
  respondingToAPI: string
  activeSubnets: string
  networksBeingScanned: string
  scanRanges: string
  activeScanningRanges: string
  none: string

  // Scan Controls
  networkScan: string
  scanConfiguredSubnet: string
  scanning: string
  scanningPorts: string
  scanningAPI: string
  startScan: string

  // Table Headers
  actions: string
  hostname: string
  ipAddress: string
  status: string
  ssh: string
  webcam: string
  order: string
  delete: string

  // Expanded Row
  apiControls: string
  pause: string
  resume: string
  stop: string
  emergencyStop: string

  // Settings Dialog
  applicationSettings: string
  configureNetworkScanning: string
  network: string
  notifications: string
  about: string
  language: string

  // Network Tab
  subnetRanges: string
  addSubnet: string
  networkName: string

  // SSH Tab
  defaultSSHUsername: string

  // Notifications Tab
  enableNotificationsFor: string
  printing: string
  paused: string
  cancelling: string
  error: string
  ready: string
  standby: string
  offline: string

  // About Tab
  version: string
  powerfulNetworkTool: string

  // Language Tab
  selectLanguage: string

  // Table Content
  discoveredHosts: string
  hostsFoundInNetwork: string
  newNetwork: string
  mainNetwork: string
  guestNetwork: string
  noSubnetsEnabled: string
  
      // Hostname Management
  
  
  // Context Menu
  copy: string
  cut: string
  paste: string
  
  // Update System
  checkForUpdates: string
  updateAvailable: string
  noUpdateAvailable: string
  currentVersion: string
  latestVersion: string
  lastCheck: string
  openRepository: string
  openReleases: string
  checkingForUpdates: string
  updateCheckFailed: string
  repository: string
  releases: string
  
  // Print Information
  printProgress: string
  filename: string
  printDuration: string
  totalDuration: string
  estimatedTime: string
  currentLayer: string
  totalLayers: string
  currentHeight: string
  totalHeight: string
  printStats: string
}

export const translations: Record<string, Translations> = {
  en,
  ru,
  de,
}

export function useTranslation(language: string): Translations {
  return translations[language] || translations.en
}

// Re-export individual language files
export { en } from './en'
export { ru } from './ru'
export { de } from './de'
