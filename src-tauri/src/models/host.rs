//! Host-related data structures

use serde::{Deserialize, Serialize};
use crate::models::api::PrinterFlags;

/// Network host information
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub failed_attempts: Option<u32>,
}

/// Host status response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostStatusResponse {
    pub success: bool,
    pub status: String,
    pub device_status: Option<String>,
    pub moonraker_version: Option<String>,
    pub klippy_state: Option<String>,
    pub printer_state: Option<String>,
    pub printer_flags: Option<PrinterFlags>,
}

/// Subnet configuration for scanning
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubnetConfig {
    pub name: String,
    pub range: String,
    pub enabled: bool,
}

/// Network scan result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub hosts: Vec<HostInfo>,
    pub total_scanned: u32,
    pub hosts_found: u32,
    pub scan_duration_ms: u64,
}
