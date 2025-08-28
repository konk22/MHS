use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use tokio::net::TcpStream;
use std::net::SocketAddr;

// Структуры данных для Moonraker API
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerServerInfo {
    pub result: ServerInfoResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfoResult {
    pub klippy_connected: bool,
    pub klippy_state: String,
    pub components: Vec<String>,
    pub failed_components: Vec<String>,
    pub registered_directories: Vec<String>,
    pub warnings: Vec<String>,
    pub websocket_count: i32,
    pub moonraker_version: String,
    pub api_version: Vec<i32>,
    #[serde(rename = "api_version_string")]
    pub api_version_string: Option<String>,
    #[serde(rename = "missing_klippy_requirements")]
    pub missing_klippy_requirements: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerPrinterInfo {
    pub result: PrinterInfoResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterInfoResult {
    pub state: String,
    pub state_message: String,
    pub hostname: Option<String>,
    pub software_version: Option<String>,
    pub cpu_info: Option<String>,
    pub klipper_path: Option<String>,
    pub python_path: Option<String>,
    pub log_file: Option<String>,
    pub config_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonrakerPrinterObjects {
    pub result: PrinterObjectsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterObjectsResult {
    pub objects: HashMap<String, PrinterObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterObject {
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterFlags {
    pub operational: bool,
    pub paused: bool,
    pub printing: bool,
    pub cancelling: bool,
    pub pausing: bool,
    #[serde(default)]
    pub resuming: bool,
    #[serde(rename = "sdReady")]
    #[serde(default)]
    pub sd_ready: bool,
    pub error: bool,
    pub ready: bool,
    #[serde(rename = "closedOrError")]
    pub closed_or_error: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostInfo {
    pub id: String,
    pub hostname: String,
    pub original_hostname: String,
    pub ip_address: String,
    pub subnet: String,
    pub status: String,
    pub device_status: String,
    pub moonraker_version: Option<String>,
    pub klippy_state: Option<String>,
    pub printer_state: Option<String>,
    pub printer_flags: Option<PrinterFlags>,
    pub last_seen: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetConfig {
    pub id: String,
    pub range: String,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub hosts: Vec<HostInfo>,
    pub total_hosts: usize,
    pub online_hosts: usize,
    pub scan_progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterControlRequest {
    pub host: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostStatusResponse {
    pub success: bool,
    pub status: String,
    pub device_status: Option<String>,
    pub moonraker_version: Option<String>,
    pub klippy_state: Option<String>,
    pub printer_state: Option<String>,
    pub printer_flags: Option<PrinterFlags>,
}

// Ошибки
#[derive(Debug, thiserror::Error)]
pub enum MoonrakerError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),
    #[error("Invalid subnet: {0}")]
    InvalidSubnet(String),
    #[error("Timeout")]
    Timeout,
    #[error("API error: {0}")]
    Api(String),
}

impl serde::Serialize for MoonrakerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// HTTP клиент для работы с Moonraker API
async fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

// Проверка доступности Moonraker API
async fn check_moonraker_api(host: &str) -> Result<MoonrakerServerInfo, MoonrakerError> {
    let client = create_client().await;
    let url = format!("http://{}:7125/server/info", host);
    
    let response = timeout(
        Duration::from_secs(3),
        client.get(&url).send()
    ).await;
    
    match response {
        Ok(Ok(response)) => {
            if response.status().is_success() {
                let server_info: MoonrakerServerInfo = response.json().await.map_err(MoonrakerError::Network)?;
                Ok(server_info)
            } else {
                Err(MoonrakerError::Api(format!("HTTP {}", response.status())))
            }
        }
        Ok(Err(e)) => {
            Err(MoonrakerError::Network(e))
        }
        Err(_) => {
            Err(MoonrakerError::Timeout)
        }
    }
}

// Получение информации о принтере
async fn get_printer_info(host: &str) -> Result<MoonrakerPrinterInfo, MoonrakerError> {
    let client = create_client().await;
    let url = format!("http://{}:7125/printer/info", host);
    
    let response = timeout(
        Duration::from_secs(3),
        client.get(&url).send()
    ).await
    .map_err(|_| MoonrakerError::Timeout)?
    .map_err(MoonrakerError::Network)?;

    if response.status().is_success() {
        let printer_info: MoonrakerPrinterInfo = response.json().await.map_err(MoonrakerError::Network)?;
        Ok(printer_info)
    } else {
        Err(MoonrakerError::Api(format!("HTTP {}", response.status())))
    }
}

// Получение объектов принтера
async fn get_printer_objects(host: &str) -> Result<MoonrakerPrinterObjects, MoonrakerError> {
    let client = create_client().await;
    let url = format!("http://{}:7125/printer/objects/query?print_stats", host);
    
    let response = timeout(
        Duration::from_secs(3),
        client.get(&url).send()
    ).await
    .map_err(|_| MoonrakerError::Timeout)?
    .map_err(MoonrakerError::Network)?;

    if response.status().is_success() {
        let printer_objects: MoonrakerPrinterObjects = response.json().await.map_err(MoonrakerError::Network)?;
        Ok(printer_objects)
    } else {
        Err(MoonrakerError::Api(format!("HTTP {}", response.status())))
    }
}

// Получение флагов принтера
async fn get_printer_flags(host: &str) -> Result<PrinterFlags, MoonrakerError> {
    let client = create_client().await;
    let url = format!("http://{}:7125/api/printer", host);
    
    let response = timeout(
        Duration::from_secs(3),
        client.get(&url).send()
    ).await
    .map_err(|_| MoonrakerError::Timeout)?
    .map_err(MoonrakerError::Network)?;

    if response.status().is_success() {
        let data: serde_json::Value = response.json().await.map_err(MoonrakerError::Network)?;
        
        if let Some(state) = data.get("state") {
            if let Some(flags) = state.get("flags") {
                let printer_flags: PrinterFlags = serde_json::from_value(flags.clone())
                    .map_err(|_| MoonrakerError::Api("Failed to parse printer flags".to_string()))?;
                Ok(printer_flags)
            } else {
                Err(MoonrakerError::Api("No flags found in response".to_string()))
            }
        } else {
            Err(MoonrakerError::Api("No state found in response".to_string()))
        }
    } else {
        Err(MoonrakerError::Api(format!("HTTP {}", response.status())))
    }
}

// Управление принтером
async fn control_printer(host: &str, action: &str) -> Result<serde_json::Value, MoonrakerError> {
    let client = create_client().await;
    
    // Правильные URL для Moonraker API
    let url = match action {
        "start" => format!("http://{}:7125/printer/print/start", host),
        "pause" => format!("http://{}:7125/printer/print/pause", host),
        "cancel" => format!("http://{}:7125/printer/print/cancel", host),
        "emergency_stop" => format!("http://{}:7125/printer/emergency_stop", host),
        _ => return Err(MoonrakerError::Api(format!("Unknown action: {}", action)))
    };
    
    let response = timeout(
        Duration::from_secs(5),
        client.post(&url).send()
    ).await
    .map_err(|_| MoonrakerError::Timeout)?
    .map_err(MoonrakerError::Network)?;

    let status = response.status();

    if status.is_success() {
        let result: serde_json::Value = response.json().await.map_err(MoonrakerError::Network)?;
        Ok(result)
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(MoonrakerError::Api(format!("HTTP {}: {}", status, error_text)))
    }
}

// Быстрая проверка доступности порта
async fn check_port(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port);
    match SocketAddr::from_str(&addr) {
        Ok(socket_addr) => {
            match timeout(Duration::from_millis(500), TcpStream::connect(socket_addr)).await {
                Ok(Ok(_)) => true,
                _ => false,
            }
        }
        Err(_) => false,
    }
}

// Генерация IP адресов из подсети
fn generate_ip_range(subnet: &str) -> Result<Vec<String>, MoonrakerError> {
    let network = ipnetwork::IpNetwork::from_str(subnet)
        .map_err(|e| MoonrakerError::InvalidSubnet(e.to_string()))?;
    
    let mut ips = Vec::new();
    for ip in network.iter() {
        // Пропускаем сетевой адрес и broadcast адрес
        if ip != network.network() && ip != network.broadcast() {
            ips.push(ip.to_string());
        }
    }
    Ok(ips)
}

// Сканирование одного хоста
async fn scan_host(ip: &str) -> Option<HostInfo> {
    match check_moonraker_api(ip).await {
        Ok(server_info) => {
            let hostname = match get_printer_info(ip).await {
                Ok(printer_info) => printer_info.result.hostname.unwrap_or_else(|| ip.to_string()),
                Err(_) => ip.to_string(),
            };

            // Получаем флаги принтера из /api/printer
            let printer_flags = match get_printer_flags(ip).await {
                Ok(flags) => Some(flags),
                Err(_) => None
            };

            // Определяем статус принтера на основе флагов
            let printer_state = if let Some(flags) = &printer_flags {
                if flags.error {
                    "error"
                } else if flags.cancelling {
                    "cancelling"
                } else if flags.paused {
                    "paused"
                } else if flags.printing {
                    "printing"
                } else if flags.ready {
                    "standby"
                } else {
                    "standby"
                }
            } else {
                "standby"
            };

            Some(HostInfo {
                id: ip.to_string(),
                hostname: hostname.clone(),
                original_hostname: hostname,
                ip_address: ip.to_string(),
                subnet: "".to_string(), // Будет заполнено позже
                status: "online".to_string(),
                device_status: printer_state.to_string(),
                moonraker_version: Some(server_info.result.moonraker_version),
                klippy_state: Some(server_info.result.klippy_state),
                printer_state: Some(printer_state.to_string()),
                printer_flags,
                last_seen: Some(chrono::Utc::now().to_rfc3339()),
            })
        }
        Err(_) => None,
    }
}

// Tauri команды

#[tauri::command]
async fn scan_network(subnets: Vec<SubnetConfig>) -> Result<ScanResult, String> {
    let mut all_hosts = Vec::new();
    let enabled_subnets: Vec<_> = subnets.into_iter().filter(|s| s.enabled).collect();
    
    if enabled_subnets.is_empty() {
        return Ok(ScanResult {
            hosts: vec![],
            total_hosts: 0,
            online_hosts: 0,
            scan_progress: 100,
        });
    }

    let mut total_ips = 0;
    let mut ip_subnet_map = HashMap::new();
    
    // Подсчитываем общее количество IP адресов
    for subnet in &enabled_subnets {
        match generate_ip_range(&subnet.range) {
            Ok(ips) => {
                total_ips += ips.len();
                for ip in ips {
                    ip_subnet_map.insert(ip, subnet.range.clone());
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    let mut online_hosts = 0;

    // Сначала быстро сканируем порт 7125 на всех IP адресах
    let mut port_scan_tasks = Vec::new();
    for (ip, _) in &ip_subnet_map {
        let ip_clone = ip.clone();
        let task = tokio::spawn(async move {
            check_port(&ip_clone, 7125).await
        });
        port_scan_tasks.push((ip.clone(), task));
    }

    // Собираем результаты сканирования портов
    let mut hosts_with_open_port = Vec::new();
    for (ip, task) in port_scan_tasks {
        if let Ok(is_open) = task.await {
            if is_open {
                hosts_with_open_port.push(ip);
            }
        }
    }

    // Теперь делаем API запросы только к хостам с открытым портом 7125
    for ip in hosts_with_open_port {
        if let Some(mut host_info) = scan_host(&ip).await {
            host_info.subnet = ip_subnet_map.get(&ip).unwrap_or(&"".to_string()).clone();
            all_hosts.push(host_info);
            online_hosts += 1;
        }
    }

    Ok(ScanResult {
        hosts: all_hosts,
        total_hosts: total_ips,
        online_hosts,
        scan_progress: 100,
    })
}

#[tauri::command]
async fn get_host_info(host: String) -> Result<HostInfo, String> {
    scan_host(&host)
        .await
        .ok_or_else(|| "Host not found or not responding".to_string())
}

#[tauri::command]
async fn control_printer_command(host: String, action: String) -> Result<serde_json::Value, String> {
    let action_url = match action.as_str() {
        "start" => "start",
        "pause" => "pause", 
        "cancel" => "cancel",
        "emergency_stop" => "emergency_stop",
        _ => return Err(format!("Unknown action: {}", action))
    };
    
    let result = control_printer(&host, action_url)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
async fn get_printer_status(host: String) -> Result<serde_json::Value, String> {
    let printer_info = get_printer_info(&host).await.map_err(|e| e.to_string())?;
    let printer_objects = get_printer_objects(&host).await.map_err(|e| e.to_string())?;
    
    let status = serde_json::json!({
        "printer_info": printer_info.result,
        "printer_objects": printer_objects.result,
    });
    
    Ok(status)
}

#[tauri::command]
fn open_webcam(host: String) -> Result<(), String> {
    let webcam_url = format!("http://{}/webcam/?action=stream", host);
    println!("Opening webcam URL: {}", webcam_url);
    
    // Используем системный браузер для открытия URL
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open").arg(&webcam_url).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd").args(&["/C", "start", &webcam_url]).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open").arg(&webcam_url).spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn open_host_in_browser(host: String) -> Result<(), String> {
    let url = format!("http://{}", host);
    println!("Opening host URL: {}", url);
    
    // Используем системный браузер для открытия URL
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open").arg(&url).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd").args(&["/C", "start", &url]).spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open").arg(&url).spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn check_host_status(ip: String) -> Result<HostStatusResponse, String> {
    println!("Checking status for host: {}", ip);
    
    // Сначала проверяем порт
    if !check_port(&ip, 7125).await {
        return Ok(HostStatusResponse {
            success: false,
            status: "offline".to_string(),
            device_status: None,
            moonraker_version: None,
            klippy_state: None,
            printer_state: None,
            printer_flags: None,
        });
    }
    
    // Проверяем Moonraker API
    match check_moonraker_api(&ip).await {
        Ok(server_info) => {
            // Получаем флаги принтера
            let printer_flags = match get_printer_flags(&ip).await {
                Ok(flags) => Some(flags),
                Err(_) => None
            };

            // Определяем статус принтера на основе флагов
            let printer_state = if let Some(flags) = &printer_flags {
                if flags.error {
                    "error"
                } else if flags.cancelling {
                    "cancelling"
                } else if flags.paused {
                    "paused"
                } else if flags.printing {
                    "printing"
                } else if flags.ready {
                    "standby"
                } else {
                    "standby"
                }
            } else {
                "standby"
            };
            
            Ok(HostStatusResponse {
                success: true,
                status: "online".to_string(),
                device_status: Some(printer_state.to_string()),
                moonraker_version: Some(server_info.result.moonraker_version),
                klippy_state: Some(server_info.result.klippy_state),
                printer_state: Some(printer_state.to_string()),
                printer_flags,
            })
        }
        Err(_) => {
            Ok(HostStatusResponse {
                success: false,
                status: "offline".to_string(),
                device_status: None,
                moonraker_version: None,
                klippy_state: None,
                printer_state: None,
                printer_flags: None,
            })
        }
    }
}

#[tauri::command]
fn open_ssh_connection(host: String, user: String) -> Result<(), String> {
    println!("Opening SSH connection to {}@{}", user, host);
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // Создаем AppleScript для открытия терминала с SSH
        let script = format!(
            "tell application \"Terminal\" to do script \"ssh {}@{}\"",
            user, host
        );
        Command::new("osascript")
            .args(&["-e", &script])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "start", "ssh", &format!("{}@{}", user, host)])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("gnome-terminal")
            .args(&["--", "bash", "-c", &format!("ssh {}@{}", user, host)])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_network,
            get_host_info,
            control_printer_command,
            get_printer_status,
            open_webcam,
            open_host_in_browser,
            open_ssh_connection,
            check_host_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
