//! Update models and data structures

use serde::{Deserialize, Serialize};

/// GitHub release information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRelease {
    /// Release tag name (e.g., "v0.0.12")
    pub tag_name: String,
    /// Release name
    pub name: String,
    /// Release body/description
    pub body: Option<String>,
    /// Whether this is a draft release
    pub draft: bool,
    /// Whether this is a pre-release
    pub prerelease: bool,
    /// Release creation date
    pub created_at: String,
    /// Release publish date
    pub published_at: Option<String>,
    /// Release assets
    pub assets: Vec<GitHubAsset>,
}

/// GitHub release asset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubAsset {
    /// Asset name
    pub name: String,
    /// Asset download URL
    pub browser_download_url: String,
    /// Asset size in bytes
    pub size: u64,
    /// Asset content type
    pub content_type: String,
}

/// Update check result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCheckResult {
    /// Whether an update is available
    pub update_available: bool,
    /// Current version
    pub current_version: String,
    /// Latest available version
    pub latest_version: Option<String>,
    /// Latest release information
    pub latest_release: Option<GitHubRelease>,
    /// Error message if check failed
    pub error: Option<String>,
    /// Last check timestamp
    pub last_check: String,
}

impl Default for UpdateCheckResult {
    fn default() -> Self {
        Self {
            update_available: false,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: None,
            latest_release: None,
            error: None,
            last_check: chrono::Utc::now().to_rfc3339(),
        }
    }
}
