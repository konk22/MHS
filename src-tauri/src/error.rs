//! Error handling module for Moonraker Host Scanner
//! 
//! This module defines custom error types and provides error handling utilities
//! for the application. All errors are designed to be serializable for Tauri
//! command responses.

use thiserror::Error;

/// Custom error types for Moonraker Host Scanner
#[derive(Debug, Error)]
pub enum MoonrakerError {
    /// Network-related errors (timeouts, connection failures, etc.)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    /// Invalid IP address format
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),
    
    /// Invalid subnet format
    #[error("Invalid subnet: {0}")]
    InvalidSubnet(String),
    
    /// Request timeout
    #[error("Request timeout after {0:?}")]
    Timeout(std::time::Duration),
    
    /// API-related errors (HTTP errors, parsing failures, etc.)
    #[error("API error: {0}")]
    Api(String),
    
    /// Host not found or not responding
    #[error("Host not found or not responding: {0}")]
    HostNotFound(String),
    
    /// System command execution error
    #[error("System command failed: {0}")]
    SystemCommand(String),
}

impl serde::Serialize for MoonrakerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for MoonrakerError {
    fn from(err: std::io::Error) -> Self {
        MoonrakerError::SystemCommand(err.to_string())
    }
}

impl From<serde_json::Error> for MoonrakerError {
    fn from(err: serde_json::Error) -> Self {
        MoonrakerError::Api(format!("JSON parsing error: {}", err))
    }
}

/// Result type alias for Moonraker operations
pub type MoonrakerResult<T> = Result<T, MoonrakerError>;

/// Helper function to convert MoonrakerError to String for Tauri commands
pub fn error_to_string(error: MoonrakerError) -> String {
    error.to_string()
}
