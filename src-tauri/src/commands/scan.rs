//! Network scanning Tauri commands
//! 
//! This module contains Tauri commands for network scanning and host discovery.

use crate::error::error_to_string;
use crate::models::{SubnetConfig, HostInfo};
use crate::network::scanner::{scan_network, scan_host, check_host_status};

/// Scans the network for Moonraker-enabled printers
/// 
/// # Arguments
/// * `subnets` - Vector of subnet configurations to scan
/// 
/// # Returns
/// * ScanResult with discovered hosts
#[tauri::command]
pub async fn scan_network_command(subnets: Vec<SubnetConfig>) -> Result<crate::models::ScanResult, String> {
    scan_network(subnets)
        .await
        .map_err(error_to_string)
}

/// Gets detailed information about a specific host
/// 
/// # Arguments
/// * `host` - Host IP address
/// 
/// # Returns
/// * HostInfo for the specified host
#[tauri::command]
pub async fn get_host_info_command(host: String) -> Result<HostInfo, String> {
    scan_host(&host)
        .await
        .ok_or_else(|| "Host not found or not responding".to_string())
}

/// Checks the current status of a host
/// 
/// # Arguments
/// * `ip` - Host IP address
/// 
/// # Returns
/// * HostStatusResponse with current status
#[tauri::command]
pub async fn check_host_status_command(ip: String) -> Result<crate::models::HostStatusResponse, String> {
    Ok(check_host_status(&ip).await)
}
