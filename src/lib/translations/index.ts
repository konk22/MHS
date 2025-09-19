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

  // Batch Tasks
  batchTasks: string
  batchTasksDescription: string
  openBatchTasks: string
  sendBatchTask: string
  saveGroup: string
  groupName: string
  enterGroupName: string
  existingGroups: string
  selectHosts: string
  noGroups: string
  selectGcodeFile: string
  noFileSelected: string
  launch: string
  checkingHosts: string
  hostsNotReady: string
  hostsNotReadyMessage: string
  uploadingFiles: string
  uploadSuccess: string
  uploadError: string

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
  backgroundMode: string
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
  noPrintInfo: string
  
  // Background Mode
  backgroundModeDescription: string
  enableBackgroundMode: string
  backgroundModeInterval: string
  backgroundModeStatus: string
  backgroundModeRunning: string
  backgroundModeStopped: string
  startBackgroundMode: string
  stopBackgroundMode: string
  backgroundModeHelp: string
  
  // Telegram Bot
  telegram: string
  telegramBotIntegration: string
  telegramBotDescription: string
  enableTelegramBot: string
  botToken: string
  enterBotToken: string
  tokenSavedAndHidden: string
  change: string
  getBotTokenFromBotFather: string
  registrationActive: string
  startRegistration: string
  stopRegistration: string
  registeredUsers: string
  botIsRunning: string
  loading: string
  testSystem: string
  testTelegram: string
  testSystemNotification: string
  testTelegramNotification: string
  testSystemNotificationBody: string
  testTelegramNotificationBody: string
  telegramNotifications: string
  
  // Additional UI elements
  flipHorizontally: string
  flipVertically: string
  unknown: string
  refresh: string
  remove: string
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
