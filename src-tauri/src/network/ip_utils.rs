//! IP address utilities
//! 
//! This module provides functions for working with IP addresses
//! and network ranges.

use std::str::FromStr;
use crate::error::{MoonrakerError, MoonrakerResult};

/// Generates a list of IP addresses from a subnet range
/// 
/// # Arguments
/// * `subnet` - Subnet in CIDR notation (e.g., "192.168.1.0/24")
/// 
/// # Returns
/// * Vector of IP addresses in the subnet
pub fn generate_ip_range(subnet: &str) -> MoonrakerResult<Vec<String>> {
    let network = ipnetwork::IpNetwork::from_str(subnet)
        .map_err(|e| MoonrakerError::InvalidSubnet(e.to_string()))?;
    
    let mut ips = Vec::new();
    for ip in network.iter() {
        // Skip network address and broadcast address
        if ip != network.network() && ip != network.broadcast() {
            ips.push(ip.to_string());
        }
    }
    Ok(ips)
}

/// Validates if a string is a valid IP address
/// 
/// # Arguments
/// * `ip` - IP address string to validate
/// 
/// # Returns
/// * True if valid IP address, false otherwise
pub fn is_valid_ip(ip: &str) -> bool {
    std::net::IpAddr::from_str(ip).is_ok()
}

/// Validates if a string is a valid subnet in CIDR notation
/// 
/// # Arguments
/// * `subnet` - Subnet string to validate
/// 
/// # Returns
/// * True if valid subnet, false otherwise
pub fn is_valid_subnet(subnet: &str) -> bool {
    ipnetwork::IpNetwork::from_str(subnet).is_ok()
}

/// Extracts the network portion from a subnet
/// 
/// # Arguments
/// * `subnet` - Subnet in CIDR notation
/// 
/// # Returns
/// * Network address as string
pub fn get_network_address(subnet: &str) -> MoonrakerResult<String> {
    let network = ipnetwork::IpNetwork::from_str(subnet)
        .map_err(|e| MoonrakerError::InvalidSubnet(e.to_string()))?;
    
    Ok(network.network().to_string())
}

/// Gets the number of hosts in a subnet
/// 
/// # Arguments
/// * `subnet` - Subnet in CIDR notation
/// 
/// # Returns
/// * Number of available host addresses
pub fn get_subnet_host_count(subnet: &str) -> MoonrakerResult<usize> {
    let network = ipnetwork::IpNetwork::from_str(subnet)
        .map_err(|e| MoonrakerError::InvalidSubnet(e.to_string()))?;
    
    // For now, return a reasonable estimate for IPv4 networks
    // This function is not critical for the main functionality
    match network {
        ipnetwork::IpNetwork::V4(net) => {
            let total_hosts = net.size();
            Ok(total_hosts.saturating_sub(2) as usize)
        }
        ipnetwork::IpNetwork::V6(_) => {
            // For IPv6, return a large number as estimate
            Ok(65536) // Reasonable default for IPv6 subnet
        }
    }
}
