//! GitHub API client for update checking
//! 
//! This module provides functionality to check for updates
//! by querying the GitHub repository API.

use crate::error::MoonrakerResult;
use crate::updater::models::{GitHubRelease, UpdateCheckResult};
use reqwest::Client;
use std::time::Duration;

const GITHUB_API_BASE: &str = "https://api.github.com";
const REPO_OWNER: &str = "konk22";
const REPO_NAME: &str = "MHS";
const USER_AGENT: &str = "MoonrakerHostScanner/0.0.9";

/// GitHub API client for checking updates
pub struct GitHubUpdater {
    client: Client,
}

impl GitHubUpdater {
    /// Creates a new GitHub updater instance
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(USER_AGENT)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Checks for available updates
    pub async fn check_for_updates(&self) -> MoonrakerResult<UpdateCheckResult> {
        let current_version = env!("CARGO_PKG_VERSION");
        
        match self.get_latest_release().await {
            Ok(latest_release) => {
                let latest_version = latest_release.tag_name.clone();
                let update_available = self.is_newer_version(current_version, &latest_version);
                
                Ok(UpdateCheckResult {
                    update_available,
                    current_version: current_version.to_string(),
                    latest_version: Some(latest_version),
                    latest_release: Some(latest_release),
                    error: None,
                    last_check: chrono::Utc::now().to_rfc3339(),
                })
            }
            Err(e) => {
                Ok(UpdateCheckResult {
                    update_available: false,
                    current_version: current_version.to_string(),
                    latest_version: None,
                    latest_release: None,
                    error: Some(e.to_string()),
                    last_check: chrono::Utc::now().to_rfc3339(),
                })
            }
        }
    }

    /// Gets the latest release from GitHub
    async fn get_latest_release(&self) -> MoonrakerResult<GitHubRelease> {
        let url = format!(
            "{}/repos/{}/{}/releases/latest",
            GITHUB_API_BASE, REPO_OWNER, REPO_NAME
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch latest release: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ).into());
        }

        let release: GitHubRelease = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse release data: {}", e))?;

        Ok(release)
    }

    /// Compares version strings to determine if a newer version is available
    fn is_newer_version(&self, current: &str, latest: &str) -> bool {
        // Remove 'v' prefix if present
        let current = current.trim_start_matches('v');
        let latest = latest.trim_start_matches('v');

        // Parse version numbers
        let current_parts: Vec<u32> = current
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        let latest_parts: Vec<u32> = latest
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        // Compare version parts
        for (current_part, latest_part) in current_parts.iter().zip(latest_parts.iter()) {
            if latest_part > current_part {
                return true;
            } else if latest_part < current_part {
                return false;
            }
        }

        // If all parts are equal, check if latest has more parts
        latest_parts.len() > current_parts.len()
    }

    /// Gets the repository URL
    pub fn get_repository_url(&self) -> String {
        format!("https://github.com/{}/{}", REPO_OWNER, REPO_NAME)
    }

    /// Gets the releases URL
    pub fn get_releases_url(&self) -> String {
        format!("https://github.com/{}/{}/releases", REPO_OWNER, REPO_NAME)
    }
}

impl Default for GitHubUpdater {
    fn default() -> Self {
        Self::new()
    }
}
