use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramUser {
    pub user_id: i64, // Serialized as i64 for frontend compatibility
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub notifications_enabled: bool,
}

impl TelegramUser {
    pub fn from_teloxide_user(user_id: UserId, username: Option<String>, first_name: String, last_name: Option<String>) -> Self {
        Self {
            user_id: user_id.0 as i64,
            username,
            first_name: Some(first_name),
            last_name,
            registered_at: chrono::Utc::now(),
            notifications_enabled: true, // Default to enabled
        }
    }
}

impl TelegramUser {
    pub fn display_name(&self) -> String {
        if let Some(username) = &self.username {
            format!("@{}", username)
        } else if let Some(first_name) = &self.first_name {
            if let Some(last_name) = &self.last_name {
                format!("{} {}", first_name, last_name)
            } else {
                first_name.clone()
            }
        } else {
            format!("User {}", self.user_id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationState {
    pub is_active: bool,
    pub code: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub attempts: u32,
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRequestState {
    pub is_active: bool,
    pub user_id: i64,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyStopRequestState {
    pub is_active: bool,
    pub user_id: i64,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionState {
    pub user_id: i64,
    pub current_menu: MenuState,
    pub last_message_id: Option<teloxide::types::MessageId>,
    pub selected_host_id: Option<String>,
    pub emergency_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuState {
    Main,
    Hosts,
    HostDetails(String), // host_id
    Settings,
    EmergencyConfirm(String), // host_id
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostCache {
    pub hosts: Vec<crate::models::HostInfo>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl RegistrationState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            code: None,
            expires_at: None,
            attempts: 0,
            max_attempts: 3,
        }
    }

    pub fn start_registration(&mut self) -> String {
        use rand::Rng;
        
        // Generate a secure 6-digit code
        let mut rng = rand::thread_rng();
        let code = format!("{:06}", rng.gen_range(100000..=999999));
        
        self.is_active = true;
        self.code = Some(code.clone());
        self.expires_at = Some(chrono::Utc::now() + chrono::Duration::seconds(300)); // 5 minutes
        self.attempts = 0; // Reset attempts counter
        
        code
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn verify_code(&mut self, input_code: &str) -> bool {
        if !self.is_active || self.is_expired() {
            return false;
        }
        
        if self.attempts >= self.max_attempts {
            return false;
        }
        
        if let Some(code) = &self.code {
            if code == input_code {
                return true;
            } else {
                self.attempts += 1;
                return false;
            }
        } else {
            false
        }
    }

    pub fn finish_registration(&mut self) {
        self.is_active = false;
        self.code = None;
        self.expires_at = None;
        self.attempts = 0;
    }
}

impl VideoRequestState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            user_id: 0,
            expires_at: None,
        }
    }

    pub fn start_video_request(&mut self, user_id: i64) {
        self.is_active = true;
        self.user_id = user_id;
        self.expires_at = Some(chrono::Utc::now() + chrono::Duration::seconds(60));
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn finish_video_request(&mut self) {
        self.is_active = false;
        self.user_id = 0;
        self.expires_at = None;
    }
}

impl EmergencyStopRequestState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            user_id: 0,
            expires_at: None,
        }
    }

    pub fn start_emergency_stop_request(&mut self, user_id: i64) {
        self.is_active = true;
        self.user_id = user_id;
        self.expires_at = Some(chrono::Utc::now() + chrono::Duration::seconds(60));
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn finish_emergency_stop_request(&mut self) {
        self.is_active = false;
        self.user_id = 0;
        self.expires_at = None;
    }
}

impl UserSessionState {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            current_menu: MenuState::Main,
            last_message_id: None,
            selected_host_id: None,
            emergency_confirmation: false,
        }
    }

    pub fn set_menu(&mut self, menu: MenuState) {
        self.current_menu = menu;
    }

    pub fn set_message_id(&mut self, message_id: teloxide::types::MessageId) {
        self.last_message_id = Some(message_id);
    }
}

impl HostCache {
    pub fn new() -> Self {
        Self {
            hosts: Vec::new(),
            last_updated: chrono::Utc::now(),
        }
    }

    pub fn update_hosts(&mut self, hosts: Vec<crate::models::HostInfo>) {
        self.hosts = hosts;
        self.last_updated = chrono::Utc::now();
    }

    pub fn is_stale(&self) -> bool {
        chrono::Utc::now() - self.last_updated > chrono::Duration::seconds(30)
    }
}
