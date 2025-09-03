//! Configuration data structures
//! 
//! This module contains configuration-related data structures and constants
//! used throughout the application.

use serde::{Deserialize, Serialize};

/// Application configuration constants
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 5;
pub const DEFAULT_PORT_SCAN_TIMEOUT_MS: u64 = 1000; // Увеличиваем таймаут
pub const MOONRAKER_PORT: u16 = 7125;
pub const WEBCAM_PORT: u16 = 8080;

// Новые константы для оптимизированного сканирования
pub const PORT_SCAN_CONCURRENCY: usize = 200; // Максимум одновременных портовых проверок
pub const API_SCAN_CONCURRENCY: usize = 50;   // Максимум одновременных API запросов
pub const PORT_SCAN_RETRY_COUNT: u32 = 1;     // Количество повторных попыток для портов
pub const API_SCAN_RETRY_COUNT: u32 = 1;      // Количество повторных попыток для API
pub const SLOW_NETWORK_TIMEOUT_MS: u64 = 1500; // Таймаут для медленных сетей

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

/// Application settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// Auto-refresh interval in seconds
    pub auto_refresh_interval: u64,
    /// Whether auto-refresh is enabled
    pub auto_refresh_enabled: bool,
    /// Notification settings
    pub notifications: NotificationSettings,
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
            theme: "system".to_string(),
            language: "en".to_string(),
        }
    }
}
