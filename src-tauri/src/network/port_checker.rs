//! Port checking utilities
//! 
//! This module provides functions for checking port availability
//! on network hosts.

use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use tokio::net::TcpStream;
use std::net::SocketAddr;

use crate::models::config::DEFAULT_PORT_SCAN_TIMEOUT_MS;

/// Checks if a port is open on the specified host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// * `port` - Port number to check
/// 
/// # Returns
/// * True if port is open, false otherwise
pub async fn check_port(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port);
    match SocketAddr::from_str(&addr) {
        Ok(socket_addr) => {
            let timeout_duration = Duration::from_millis(DEFAULT_PORT_SCAN_TIMEOUT_MS);
            match timeout(timeout_duration, TcpStream::connect(socket_addr)).await {
                Ok(Ok(_)) => true,
                _ => false,
            }
        }
        Err(_) => false,
    }
}

/// Checks if Moonraker port (7125) is open on the specified host
/// 
/// # Arguments
/// * `ip` - IP address to check
/// 
/// # Returns
/// * True if Moonraker port is open, false otherwise
pub async fn check_moonraker_port(ip: &str) -> bool {
    check_port(ip, 7125).await
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
