//! Scan progress tracking structures
//! 
//! This module contains data structures for tracking network scanning progress
//! and providing real-time feedback to the user interface.

use serde::{Deserialize, Serialize};

/// Scan progress information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanProgress {
    /// Current phase of scanning
    pub phase: ScanPhase,
    /// Progress percentage (0-100)
    pub percentage: u8,
    /// Current IP being scanned
    pub current_ip: Option<String>,
    /// Total IPs to scan
    pub total_ips: usize,
    /// Scanned IPs count
    pub scanned_ips: usize,
    /// Found hosts count
    pub found_hosts: usize,
    /// Phase-specific message
    pub message: String,
}

/// Scanning phases
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScanPhase {
    /// Preparing scan
    Preparing,
    /// Scanning ports
    PortScanning,
    /// Checking APIs
    ApiChecking,
    /// Completed
    Completed,
    /// Error occurred
    Error,
}

impl Default for ScanProgress {
    fn default() -> Self {
        Self {
            phase: ScanPhase::Preparing,
            percentage: 0,
            current_ip: None,
            total_ips: 0,
            scanned_ips: 0,
            found_hosts: 0,
            message: "Preparing scan...".to_string(),
        }
    }
}

impl ScanProgress {
    /// Creates a new scan progress instance
    pub fn new(total_ips: usize) -> Self {
        Self {
            total_ips,
            ..Default::default()
        }
    }

    /// Updates progress for port scanning phase
    pub fn update_port_scanning(&mut self, scanned: usize) {
        self.phase = ScanPhase::PortScanning;
        self.scanned_ips = scanned;
        self.percentage = if self.total_ips > 0 {
            ((scanned as f32 / self.total_ips as f32) * 40.0) as u8
        } else {
            0
        };
        self.message = format!("Scanning ports: {}/{} IPs", scanned, self.total_ips);
    }

    /// Updates progress for API checking phase
    pub fn update_api_checking(&mut self, checked: usize, found: usize) {
        self.phase = ScanPhase::ApiChecking;
        self.scanned_ips = checked;
        self.found_hosts = found;
        self.percentage = 40 + if self.total_ips > 0 {
            ((checked as f32 / self.total_ips as f32) * 50.0) as u8
        } else {
            0
        };
        self.message = format!("Checking APIs: {}/{} IPs, found {} hosts", checked, self.total_ips, found);
    }

    /// Marks scan as completed
    pub fn complete(&mut self, found_hosts: usize) {
        self.phase = ScanPhase::Completed;
        self.percentage = 100;
        self.found_hosts = found_hosts;
        self.message = format!("Scan completed! Found {} hosts", found_hosts);
    }

    /// Marks scan as error
    pub fn error(&mut self, error_message: &str) {
        self.phase = ScanPhase::Error;
        self.message = format!("Scan error: {}", error_message);
    }

    /// Updates current IP being processed
    pub fn set_current_ip(&mut self, ip: Option<String>) {
        self.current_ip = ip;
    }
}
