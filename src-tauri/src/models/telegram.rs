use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramUser {
    pub user_id: i64, // Serialized as i64 for frontend compatibility
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

impl TelegramUser {
    pub fn from_teloxide_user(user_id: UserId, username: Option<String>, first_name: String, last_name: Option<String>) -> Self {
        Self {
            user_id: user_id.0 as i64,
            username,
            first_name: Some(first_name),
            last_name,
            registered_at: chrono::Utc::now(),
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
}

impl RegistrationState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            code: None,
            expires_at: None,
        }
    }

    pub fn start_registration(&mut self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut hasher = DefaultHasher::new();
        timestamp.hash(&mut hasher);
        let hash = hasher.finish();
        
        let code = format!("{:06}", hash % 1000000);
        
        self.is_active = true;
        self.code = Some(code.clone());
        self.expires_at = Some(chrono::Utc::now() + chrono::Duration::seconds(60));
        
        code
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn verify_code(&self, input_code: &str) -> bool {
        if let Some(code) = &self.code {
            code == input_code
        } else {
            false
        }
    }

    pub fn finish_registration(&mut self) {
        self.is_active = false;
        self.code = None;
        self.expires_at = None;
    }
}
