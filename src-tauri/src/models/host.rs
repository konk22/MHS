//! Host-related data structures
//! 
//! This module contains data structures for network hosts, scan results,
//! and host status information.

use serde::{Deserialize, Serialize};
use crate::models::api::PrinterFlags;

/// Host information for discovered Moonraker printers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostInfo {
    /// Unique identifier for the host
    pub id: String,
    /// Display name for the host (can be customized)
    pub hostname: String,
    /// Original hostname from the printer
    pub original_hostname: String,
    /// IP address of the host
    pub ip_address: String,
    /// Subnet the host belongs to
    pub subnet: String,
    /// Online/offline status
    pub status: String,
    /// Printer device status (printing, paused, etc.)
    pub device_status: String,
    /// Moonraker version
    pub moonraker_version: Option<String>,
    /// Klippy connection state
    pub klippy_state: Option<String>,
    /// Printer state
    pub printer_state: Option<String>,
    /// Printer status flags
    pub printer_flags: Option<PrinterFlags>,
    /// Last time the host was seen
    pub last_seen: Option<String>,
    /// Number of consecutive failed connection attempts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_attempts: Option<u32>,
}

impl HostInfo {
    /// Creates a new HostInfo instance
    pub fn new(
        ip_address: String,
        hostname: String,
        subnet: String,
    ) -> Self {
        Self {
            id: ip_address.clone(),
            hostname: hostname.clone(),
            original_hostname: hostname,
            ip_address,
            subnet,
            status: "offline".to_string(),
            device_status: "standby".to_string(),
            moonraker_version: None,
            klippy_state: None,
            printer_state: None,
            printer_flags: None,
            last_seen: None,
            failed_attempts: Some(0),
        }
    }

    /// Updates the host with online status and printer information
    pub fn update_online(
        &mut self,
        moonraker_version: String,
        klippy_state: String,
        printer_flags: Option<PrinterFlags>,
    ) {
        self.status = "online".to_string();
        self.moonraker_version = Some(moonraker_version);
        self.klippy_state = Some(klippy_state);
        self.printer_flags = printer_flags.clone();
        
        if let Some(flags) = printer_flags {
            self.device_status = flags.get_status().to_string();
            self.printer_state = Some(flags.get_status().to_string());
        }
        
        self.last_seen = Some(chrono::Utc::now().to_rfc3339());
        self.failed_attempts = Some(0);
    }

    /// Marks the host as offline and increments failed attempts
    pub fn update_offline(&mut self) {
        let failed_attempts = self.failed_attempts.unwrap_or(0) + 1;
        self.failed_attempts = Some(failed_attempts);
        
        // Only mark as offline after 3 consecutive failures
        if failed_attempts >= 3 {
            self.status = "offline".to_string();
        }
    }

    /// Checks if the host should be considered offline
    pub fn is_offline(&self) -> bool {
        self.failed_attempts.unwrap_or(0) >= 3
    }
}

/// Subnet configuration for network scanning
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubnetConfig {
    /// Unique identifier for the subnet
    pub id: String,
    /// IP range (e.g., "192.168.1.0/24")
    pub range: String,
    /// Display name for the subnet
    pub name: String,
    /// Whether this subnet is enabled for scanning
    pub enabled: bool,
}

/// Result of a network scan operation
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    /// List of discovered hosts
    pub hosts: Vec<HostInfo>,
    /// Total number of IP addresses scanned
    pub total_hosts: usize,
    /// Number of hosts that responded
    pub online_hosts: usize,
    /// Scan progress percentage (0-100)
    pub scan_progress: u32,
}

/// Response for host status check
#[derive(Debug, Serialize, Deserialize)]
pub struct HostStatusResponse {
    /// Whether the host responded successfully
    pub success: bool,
    /// Online/offline status
    pub status: String,
    /// Printer device status
    pub device_status: Option<String>,
    /// Moonraker version
    pub moonraker_version: Option<String>,
    /// Klippy state
    pub klippy_state: Option<String>,
    /// Printer state
    pub printer_state: Option<String>,
    /// Printer status flags
    pub printer_flags: Option<PrinterFlags>,
}

/// Request for printer control operations
#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterControlRequest {
    /// Host IP address
    pub host: String,
    /// Action to perform (start, pause, cancel, emergency_stop)
    pub action: String,
}
