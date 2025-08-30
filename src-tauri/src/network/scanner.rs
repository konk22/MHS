//! Network scanner functionality
//! 
//! This module provides functions for scanning networks to discover
//! Moonraker-enabled 3D printers.

use std::collections::HashMap;
use crate::error::MoonrakerResult;
use crate::models::{
    HostInfo,
    SubnetConfig,
    ScanResult,
    HostStatusResponse,
};
use crate::api::moonraker::{check_moonraker_api, get_printer_flags, get_printer_info};
use crate::network::port_checker::check_moonraker_port;
use crate::network::ip_utils::generate_ip_range;

/// Scans a single host for Moonraker API availability
/// 
/// # Arguments
/// * `ip` - IP address to scan
/// 
/// # Returns
/// * HostInfo if Moonraker is found, None otherwise
pub async fn scan_host(ip: &str) -> Option<HostInfo> {
    // First check if port 7125 is open
    if !check_moonraker_port(ip).await {
        return None;
    }

    // Then check Moonraker API
    match check_moonraker_api(ip).await {
        Ok(server_info) => {
            // Get printer hostname
            let hostname = match get_printer_info(ip).await {
                Ok(printer_info) => printer_info.result.hostname.unwrap_or_else(|| ip.to_string()),
                Err(_) => ip.to_string(),
            };

            // Get printer flags
            let printer_flags = match get_printer_flags(ip).await {
                Ok(flags) => Some(flags),
                Err(_) => None
            };

            // Determine printer status based on flags
            let printer_state = if let Some(flags) = &printer_flags {
                flags.get_status()
            } else {
                "standby"
            };

            Some(HostInfo {
                id: ip.to_string(),
                hostname: hostname.clone(),
                original_hostname: hostname,
                ip_address: ip.to_string(),
                subnet: "".to_string(), // Will be filled later
                status: "online".to_string(),
                device_status: printer_state.to_string(),
                moonraker_version: Some(server_info.result.moonraker_version),
                klippy_state: Some(server_info.result.klippy_state),
                printer_state: Some(printer_state.to_string()),
                printer_flags,
                last_seen: Some(chrono::Utc::now().to_rfc3339()),
                failed_attempts: Some(0),
            })
        }
        Err(_) => None,
    }
}

/// Checks the status of a single host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// 
/// # Returns
/// * HostStatusResponse with current status
pub async fn check_host_status(ip: &str) -> HostStatusResponse {
    // First check if port 7125 is open
    if !check_moonraker_port(ip).await {
        return HostStatusResponse {
            success: false,
            status: "offline".to_string(),
            device_status: None,
            moonraker_version: None,
            klippy_state: None,
            printer_state: None,
            printer_flags: None,
        };
    }

            // Check Moonraker API
        match check_moonraker_api(ip).await {
            Ok(server_info) => {
                // Check if Klippy is completely disconnected (not just in error state)
                let klippy_disconnected = server_info.result.klippy_state == "disconnected";
                
                if klippy_disconnected {
                    return HostStatusResponse {
                success: false,
                status: "offline".to_string(),
                device_status: Some("klippy_disconnected".to_string()),
                moonraker_version: Some(server_info.result.moonraker_version),
                klippy_state: Some(server_info.result.klippy_state),
                printer_state: Some("offline".to_string()),
                printer_flags: None,
            };
        }
            
                            // Get printer flags
                let printer_flags = match get_printer_flags(ip).await {
                    Ok(flags) => Some(flags),
                    Err(e) => {
                        eprintln!("Failed to get printer flags for {}: {}", ip, e);
                        None
                    }
                };

                // Determine printer status based on flags
                let printer_state = if let Some(flags) = &printer_flags {
                    flags.get_status()
                } else {
                    "standby"
                };
            
            HostStatusResponse {
                success: true,
                status: "online".to_string(),
                device_status: Some(printer_state.to_string()),
                moonraker_version: Some(server_info.result.moonraker_version),
                klippy_state: Some(server_info.result.klippy_state),
                printer_state: Some(printer_state.to_string()),
                printer_flags,
            }
                    }
            Err(_) => {
                HostStatusResponse {
                success: false,
                status: "offline".to_string(),
                device_status: None,
                moonraker_version: None,
                klippy_state: None,
                printer_state: None,
                printer_flags: None,
            }
        }
    }
}

/// Scans multiple subnets for Moonraker hosts
/// 
/// # Arguments
/// * `subnets` - Vector of subnet configurations to scan
/// 
/// # Returns
/// * ScanResult with discovered hosts
pub async fn scan_network(subnets: Vec<SubnetConfig>) -> MoonrakerResult<ScanResult> {
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
    
    // Count total IP addresses
    for subnet in &enabled_subnets {
        match generate_ip_range(&subnet.range) {
            Ok(ips) => {
                total_ips += ips.len();
                for ip in ips {
                    ip_subnet_map.insert(ip, subnet.range.clone());
                }
            }
            Err(e) => return Err(e),
        }
    }

    let mut online_hosts = 0;

    // First quickly scan port 7125 on all IP addresses
    let mut port_scan_tasks = Vec::new();
    for (ip, _) in &ip_subnet_map {
        let ip_clone = ip.clone();
        let task = tokio::spawn(async move {
            check_moonraker_port(&ip_clone).await
        });
        port_scan_tasks.push((ip.clone(), task));
    }

    // Collect port scan results
    let mut hosts_with_open_port = Vec::new();
    for (ip, task) in port_scan_tasks {
        if let Ok(is_open) = task.await {
            if is_open {
                hosts_with_open_port.push(ip);
            }
        }
    }

    // Now make API requests only to hosts with open port 7125
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
