//! Configuration data structures
//! 
//! This module contains configuration-related data structures and constants
//! used throughout the application.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration constants
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 5;
pub const DEFAULT_PORT_SCAN_TIMEOUT_MS: u64 = 500; // Fast timeout for offline detection
pub const MOONRAKER_PORT: u16 = 7125;
pub const WEBCAM_PORT: u16 = 8080;

// Optimized scanning constants
pub const PORT_SCAN_CONCURRENCY: usize = 200; // Maximum concurrent port checks
pub const API_SCAN_CONCURRENCY: usize = 50;   // Maximum concurrent API requests
pub const PORT_SCAN_RETRY_COUNT: u32 = 1;     // Number of retry attempts for ports
pub const API_SCAN_RETRY_COUNT: u32 = 3;      // Number of retry attempts for API (reduced false positives)
pub const SLOW_NETWORK_TIMEOUT_MS: u64 = 800; // Timeout for slow networks (reduced)

/// Notification settings for different printer states
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationSettings {
    /// Enable notifications for printing status
    pub printing: bool,
    /// Enable notifications for paused status
    pub paused: bool,
    /// Enable notifications for error status
    pub error: bool,
    /// Enable notifications for cancelling status
    pub cancelling: bool,
    /// Enable notifications for standby status
    pub standby: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            printing: true,
            paused: true,
            error: true,
            cancelling: true,
            standby: false,
        }
    }
}

/// Telegram bot settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramSettings {
    /// Whether Telegram bot is enabled
    pub enabled: bool,
    /// Bot token (encrypted in config file)
    pub bot_token: Option<String>,
    /// Notification settings for Telegram
    pub notifications: NotificationSettings,
    /// Registered users
    pub registered_users: Vec<crate::models::TelegramUser>,
}

impl Default for TelegramSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            bot_token: None,
            notifications: NotificationSettings::default(),
            registered_users: Vec::new(),
        }
    }
}

/// Application settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// Auto-refresh interval in seconds
    pub auto_refresh_interval: u64,
    /// Whether auto-refresh is enabled
    pub auto_refresh_enabled: bool,
    /// Notification settings
    pub notifications: NotificationSettings,
    /// Telegram bot settings
    pub telegram: TelegramSettings,
    /// Theme preference
    pub theme: String,
    /// Language preference
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 3,
            auto_refresh_enabled: true,
            notifications: NotificationSettings::default(),
            telegram: TelegramSettings::default(),
            theme: "system".to_string(),
            language: "en".to_string(),
        }
    }
}

impl AppSettings {
    /// Get the configuration file path
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("moonraker-host-scanner");
        path.push("config.json");
        path
    }

    /// Load settings from file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();
        
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        
        // Try to parse as AppSettings first
        match serde_json::from_str::<AppSettings>(&content) {
            Ok(settings) => Ok(settings),
            Err(_) => {
                // If parsing fails, try to migrate from old format
                let mut value: serde_json::Value = serde_json::from_str(&content)?;
                
                // Add missing fields if they don't exist
                if let Some(telegram) = value.get_mut("telegram") {
                    if !telegram.get("registered_users").is_some() {
                        telegram["registered_users"] = serde_json::Value::Array(vec![]);
                    }
                }
                
                // Parse the migrated value
                let settings: AppSettings = serde_json::from_value(value)?;
                Ok(settings)
            }
        }
    }

    /// Save settings to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
