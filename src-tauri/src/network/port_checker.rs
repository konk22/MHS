//! Port checking utilities
//! 
//! This module provides functions for checking port availability
//! on network hosts with optimized scanning and retry logic.

use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use std::collections::HashMap;

use crate::models::config::{
    DEFAULT_PORT_SCAN_TIMEOUT_MS, 
    PORT_SCAN_CONCURRENCY, 
    PORT_SCAN_RETRY_COUNT,
    SLOW_NETWORK_TIMEOUT_MS
};

/// Checks if a port is open on the specified host with retry logic
/// 
/// # Arguments
/// * `ip` - IP address to check
/// * `port` - Port number to check
/// * `timeout_ms` - Timeout in milliseconds
/// 
/// # Returns
/// * True if port is open, false otherwise
pub async fn check_port_with_retry(ip: &str, port: u16, timeout_ms: u64) -> bool {
    let addr = format!("{}:{}", ip, port);
    let socket_addr = match SocketAddr::from_str(&addr) {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    let timeout_duration = Duration::from_millis(timeout_ms);
    
    for attempt in 0..PORT_SCAN_RETRY_COUNT {
        match timeout(timeout_duration, TcpStream::connect(socket_addr)).await {
            Ok(Ok(_)) => return true,
            Ok(Err(_)) => {
                // Connection failed, try again if we have attempts left
                if attempt < PORT_SCAN_RETRY_COUNT - 1 {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    continue;
                }
            }
            Err(_) => {
                // Timeout, try again if we have attempts left
                if attempt < PORT_SCAN_RETRY_COUNT - 1 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
            }
        }
    }
    
    false
}

/// Checks if a port is open on the specified host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// * `port` - Port number to check
/// 
/// # Returns
/// * True if port is open, false otherwise
pub async fn check_port(ip: &str, port: u16) -> bool {
    check_port_with_retry(ip, port, DEFAULT_PORT_SCAN_TIMEOUT_MS).await
}

/// Checks if Moonraker port (7125) is open on the specified host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// 
/// # Returns
/// * True if Moonraker port is open, false otherwise
pub async fn check_moonraker_port(ip: &str) -> bool {
    check_port_with_retry(ip, 7125, DEFAULT_PORT_SCAN_TIMEOUT_MS).await
}

/// Checks if Moonraker port is open with adaptive timeout
/// Uses longer timeout for potentially slow networks
/// 
/// # Arguments
/// * `ip` - IP address to check
/// 
/// # Returns
/// * True if Moonraker port is open, false otherwise
pub async fn check_moonraker_port_adaptive(ip: &str) -> bool {
    // First try with normal timeout
    if check_port_with_retry(ip, 7125, DEFAULT_PORT_SCAN_TIMEOUT_MS).await {
        return true;
    }
    
    // If failed, try with longer timeout for slow networks
    check_port_with_retry(ip, 7125, SLOW_NETWORK_TIMEOUT_MS).await
}

/// Checks multiple ports on a host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// * `ports` - Vector of port numbers to check
/// 
/// # Returns
/// * Vector of (port, is_open) tuples
pub async fn check_multiple_ports(ip: &str, ports: Vec<u16>) -> Vec<(u16, bool)> {
    let mut results = Vec::new();
    
    for port in ports {
        let is_open = check_port(ip, port).await;
        results.push((port, is_open));
    }
    
    results
}

/// Efficiently scans multiple IP addresses for open Moonraker ports
/// Uses controlled concurrency to avoid overwhelming the network
/// 
/// # Arguments
/// * `ips` - Vector of IP addresses to scan
/// 
/// # Returns
/// * HashMap mapping IP addresses to port status
pub async fn scan_multiple_ips_for_moonraker(ips: Vec<String>) -> HashMap<String, bool> {
    let mut results = HashMap::new();
    
    // Process IPs in chunks to control concurrency
    for chunk in ips.chunks(PORT_SCAN_CONCURRENCY) {
        let futures: Vec<_> = chunk.iter().map(|ip| {
            let ip_clone = ip.clone();
            async move {
                let is_open = check_moonraker_port_adaptive(&ip_clone).await;
                (ip_clone, is_open)
            }
        }).collect();
        
        // Execute chunk concurrently
        let chunk_results = futures::future::join_all(futures).await;
        for (ip, is_open) in chunk_results {
            results.insert(ip, is_open);
        }
        
        // Small delay between chunks to be network-friendly
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    results
}
