//! Network scanner functionality
//! 
//! This module provides functions for scanning networks to discover
//! Moonraker-enabled 3D printers with optimized scanning algorithms.

use std::collections::HashMap;
use crate::error::MoonrakerResult;
use crate::models::{
    HostInfo,
    SubnetConfig,
    ScanResult,
    HostStatusResponse,
};

use crate::api::moonraker::{check_moonraker_api, get_printer_flags, get_printer_info};
use crate::network::port_checker::{check_moonraker_port_adaptive, scan_multiple_ips_for_moonraker};
use crate::network::ip_utils::generate_ip_range;
use crate::models::config::{API_SCAN_CONCURRENCY, API_SCAN_RETRY_COUNT};

/// Scans a single host for Moonraker API availability with retry logic
/// 
/// # Arguments
/// * `ip` - IP address to scan
/// 
/// # Returns
/// * HostInfo if Moonraker is found, None otherwise
pub async fn scan_host(ip: &str) -> Option<HostInfo> {
    // First check if port 7125 is open with adaptive timeout
    if !check_moonraker_port_adaptive(ip).await {
        return None;
    }

    // Then check Moonraker API with retry logic
    for attempt in 0..API_SCAN_RETRY_COUNT {
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

                return Some(HostInfo {
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
                });
            }
            Err(_) => {
                // If this is not the last attempt, wait a bit and try again
                if attempt < API_SCAN_RETRY_COUNT - 1 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    continue;
                }
            }
        }
    }
    
    None
}

/// Checks the status of a single host with improved error handling
/// 
/// # Arguments
/// * `ip` - IP address to check
/// 
/// # Returns
/// * HostStatusResponse with current status
pub async fn check_host_status(ip: &str) -> HostStatusResponse {
    // First check if port 7125 is open with adaptive timeout
    if !check_moonraker_port_adaptive(ip).await {
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

    // Check Moonraker API with retry logic
    for attempt in 0..API_SCAN_RETRY_COUNT {
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
            
                return HostStatusResponse {
                    success: true,
                    status: "online".to_string(),
                    device_status: Some(printer_state.to_string()),
                    moonraker_version: Some(server_info.result.moonraker_version),
                    klippy_state: Some(server_info.result.klippy_state),
                    printer_state: Some(printer_state.to_string()),
                    printer_flags,
                };
            }
            Err(_) => {
                // If this is not the last attempt, wait a bit and try again
                if attempt < API_SCAN_RETRY_COUNT - 1 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    continue;
                }
            }
        }
    }
    
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

/// Scans multiple subnets for Moonraker hosts with optimized parallel scanning
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
    
    // Count total IP addresses and build IP list
    let mut all_ips = Vec::new();
    for subnet in &enabled_subnets {
        match generate_ip_range(&subnet.range) {
            Ok(ips) => {
                total_ips += ips.len();
                for ip in ips {
                    ip_subnet_map.insert(ip.clone(), subnet.range.clone());
                    all_ips.push(ip);
                }
            }
            Err(e) => return Err(e),
        }
    }

    // Phase 1: Parallel port scanning with controlled concurrency
    let port_scan_results = scan_multiple_ips_for_moonraker(all_ips).await;
    
    let hosts_with_open_port: Vec<String> = port_scan_results
        .into_iter()
        .filter(|(_, is_open)| *is_open)
        .map(|(ip, _)| ip)
        .collect();

    // Phase 2: API scanning with controlled concurrency
    let mut online_hosts = 0;
    
    // Process API checks in chunks to control concurrency
    for chunk in hosts_with_open_port.chunks(API_SCAN_CONCURRENCY) {
        let futures: Vec<_> = chunk.iter().map(|ip| {
            let ip_clone = ip.clone();
            async move {
                let host_info = scan_host(&ip_clone).await;
                (ip_clone, host_info)
            }
        }).collect();
        
        // Execute chunk concurrently
        let chunk_results = futures::future::join_all(futures).await;
        for (ip, host_info) in chunk_results {
            if let Some(mut host_info) = host_info {
                host_info.subnet = ip_subnet_map.get(&ip).unwrap_or(&"".to_string()).clone();
                all_hosts.push(host_info);
                online_hosts += 1;
            }
        }
        
        // Small delay between chunks to be network-friendly
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    Ok(ScanResult {
        hosts: all_hosts,
        total_hosts: total_ips,
        online_hosts,
        scan_progress: 100,
    })
}
