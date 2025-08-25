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
  autoRefresh: string

  // Table Headers
  actions: string
  hostname: string
  ipAddress: string
  status: string
  ssh: string
  webcam: string
  delete: string

  // Expanded Row
  apiControls: string
  start: string
  pause: string
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
  scanType: string
  apiResponseScan: string
  pingScan: string
  portScan: string

  // SSH Tab
  defaultSSHUsername: string

  // Notifications Tab
  enableNotificationsFor: string
  printing: string
  paused: string
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
}

export const translations: Record<string, Translations> = {
  en: {
    // Header
    networkScanner: "Moonraker Host Scanner",
    discoverHosts: "Discover Klipper 3D printers on your local network",
    settings: "Settings",

    // Stats Cards
    totalHosts: "Total Hosts",
    discoveredInNetwork: "Discovered in network",
    onlineHosts: "Online Hosts",
    respondingToAPI: "Responding to API calls",
    activeSubnets: "Active Subnets",
    networksBeingScanned: "Networks being scanned",
    scanRanges: "Scan Ranges",
    activeScanningRanges: "Active scanning ranges",
    none: "None",

    // Scan Controls
    networkScan: "Network Scan",
    scanConfiguredSubnet: "Scan the configured subnet for hosts that respond to API requests",
    scanning: "Scanning...",
    scanningPorts: "Scanning ports...",
    scanningAPI: "Checking API...",
    startScan: "Start Scan",
    autoRefresh: "Auto-refresh",

    // Table Headers
    actions: "Actions",
    hostname: "Hostname",
    ipAddress: "IP Address",
    status: "Status",
    ssh: "SSH",
    webcam: "Webcam",
    delete: "Delete",

    // Expanded Row
    apiControls: "API Controls",
    start: "Start",
    pause: "Pause",
    stop: "Stop",
    emergencyStop: "Emergency Stop",

    // Settings Dialog
    applicationSettings: "Application Settings",
    configureNetworkScanning: "Configure network scanning and application preferences",
    network: "Network",
    notifications: "Notifications",
    about: "About",
    language: "Language",

    // Network Tab
    subnetRanges: "Subnet Ranges",
    addSubnet: "Add Subnet",
    networkName: "Network name",
    scanType: "Scan Type",
    apiResponseScan: "API Response Scan",
    pingScan: "Ping Scan",
    portScan: "Port Scan",

    // SSH Tab
    defaultSSHUsername: "Default SSH Username",

    // Notifications Tab
    enableNotificationsFor: "Enable notifications for:",
    printing: "Printing",
    paused: "Paused",
    error: "Error",
    ready: "Ready",
    standby: "Standby",
    offline: "Offline",

    // About Tab
    version: "Version 1.0.0",
    powerfulNetworkTool: "A powerful network discovery tool for finding API-enabled hosts",

    // Language Tab
    selectLanguage: "Select Language",

    // Table Content
    discoveredHosts: "Discovered Hosts",
    hostsFoundInNetwork: "Hosts found in the network that respond to API requests",
    newNetwork: "New Network",
    mainNetwork: "Main Network",
    guestNetwork: "Guest Network",
    noSubnetsEnabled: "No subnets enabled for scanning",
  },
  ru: {
    // Header
    networkScanner: "Moonraker Host Scanner",
    discoverHosts: "Поиск 3D принтеров Klipper в локальной сети",
    settings: "Настройки",

    // Stats Cards
    totalHosts: "Всего Хостов",
    discoveredInNetwork: "Обнаружено в сети",
    onlineHosts: "Онлайн Хосты",
    respondingToAPI: "Отвечают на API вызовы",
    activeSubnets: "Активные Подсети",
    networksBeingScanned: "Сканируемые сети",
    scanRanges: "Диапазоны Сканирования",
    activeScanningRanges: "Активные диапазоны сканирования",
    none: "Нет",

    // Scan Controls
    networkScan: "Сканирование Сети",
    scanConfiguredSubnet: "Сканировать настроенную подсеть на хосты, отвечающие на API запросы",
    scanning: "Сканирование...",
    scanningPorts: "Сканирование портов...",
    scanningAPI: "Проверка API...",
    startScan: "Начать Сканирование",
    autoRefresh: "Автообновление",

    // Table Headers
    actions: "Действия",
    hostname: "Имя Хоста",
    ipAddress: "IP Адрес",
    status: "Статус",
    ssh: "SSH",
    webcam: "Веб-камера",
    delete: "Удалить",

    // Expanded Row
    apiControls: "API Управление",
    start: "Старт",
    pause: "Пауза",
    stop: "Стоп",
    emergencyStop: "Экстренный Стоп",

    // Settings Dialog
    applicationSettings: "Настройки Приложения",
    configureNetworkScanning: "Настройка сканирования сети и параметров приложения",
    network: "Сеть",
    notifications: "Уведомления",
    about: "О программе",
    language: "Язык",

    // Network Tab
    subnetRanges: "Диапазоны Подсетей",
    addSubnet: "Добавить Подсеть",
    networkName: "Имя сети",
    scanType: "Тип Сканирования",
    apiResponseScan: "Сканирование API Ответов",
    pingScan: "Ping Сканирование",
    portScan: "Сканирование Портов",

    // SSH Tab
    defaultSSHUsername: "Имя Пользователя SSH по Умолчанию",

    // Notifications Tab
    enableNotificationsFor: "Включить уведомления для:",
    printing: "Печать",
    paused: "Пауза",
    error: "Ошибка",
    ready: "Готов",
    standby: "Ожидание",
    offline: "Оффлайн",

    // About Tab
    version: "Версия 1.0.0",
    powerfulNetworkTool: "Мощный инструмент для обнаружения сетевых устройств с поддержкой API",

    // Language Tab
    selectLanguage: "Выбрать Язык",

    // Table Content
    discoveredHosts: "Обнаруженные Хосты",
    hostsFoundInNetwork: "Хосты, найденные в сети, которые отвечают на API запросы",
    newNetwork: "Новая Сеть",
    mainNetwork: "Основная Сеть",
    guestNetwork: "Гостевая Сеть",
    noSubnetsEnabled: "Нет активных подсетей для сканирования",
  },
}

export function useTranslation(language: string): Translations {
  return translations[language] || translations.en
}
