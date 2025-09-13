use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::models::{TelegramUser, RegistrationState, VideoRequestState, EmergencyStopRequestState};
use std::time::Duration;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "MHS Bot commands:")]
enum Command {
    #[command(description = "Get list of available hosts")]
    Hosts,
    #[command(description = "Enable notifications")]
    NotificationOn,
    #[command(description = "Disable notifications")]
    NotificationOff,
    #[command(description = "Get image from host webcam")]
    Image,
    #[command(description = "Emergency stop printer")]
    EmergencyStop,
}

#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
    is_running: Arc<AtomicBool>,
    task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    registration_state: Arc<Mutex<RegistrationState>>,
    video_request_state: Arc<Mutex<VideoRequestState>>,
    emergency_stop_request_state: Arc<Mutex<EmergencyStopRequestState>>,
    hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    http_client: reqwest::Client,
}

impl TelegramBot {
    /// Creates a new Telegram bot instance
    /// 
    /// # Arguments
    /// * `bot_token` - The Telegram bot token
    /// * `hosts` - Shared reference to the hosts list
    /// 
    /// # Returns
    /// * `Ok(TelegramBot)` - Successfully created bot instance
    /// * `Err(String)` - Error message if creation failed
    pub async fn new(bot_token: String, hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>) -> Result<Self, String> {
        // Create HTTP client with timeout configuration
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        
        let bot = Self {
            bot: Bot::new(bot_token),
            is_running: Arc::new(AtomicBool::new(false)),
            task_handle: Arc::new(Mutex::new(None)),
            registered_users: Arc::new(Mutex::new(Vec::new())),
            registration_state: Arc::new(Mutex::new(RegistrationState::new())),
            video_request_state: Arc::new(Mutex::new(VideoRequestState::new())),
            emergency_stop_request_state: Arc::new(Mutex::new(EmergencyStopRequestState::new())),
            hosts,
            http_client,
        };
        
        // Load users from file
        bot.load_users_from_file().await?;
        
        Ok(bot)
    }

    /// Starts the Telegram bot
    /// 
    /// # Returns
    /// * `Ok(())` - Bot started successfully
    /// * `Err(String)` - Error message if start failed
    pub async fn start(&self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err("Bot is already running".to_string());
        }

        let bot = self.bot.clone();
        let is_running = self.is_running.clone();
        let task_handle = self.task_handle.clone();

        let registered_users = self.registered_users.clone();
        let registration_state = self.registration_state.clone();
        let video_request_state = self.video_request_state.clone();
        let emergency_stop_request_state = self.emergency_stop_request_state.clone();
        let hosts = self.hosts.clone();
        let http_client = self.http_client.clone();
        
        let handle = tokio::spawn(async move {
            is_running.store(true, Ordering::Relaxed);
            
            // Test bot token by getting bot info first
            match bot.get_me().await {
                Ok(bot_info) => {
                    println!("Bot started successfully: @{}", bot_info.username());
                    
                    // Set bot commands menu only if bot is valid
                    if let Err(e) = bot.set_my_commands(Command::bot_commands()).await {
                        println!("Failed to set bot commands: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to start bot - invalid token: {}", e);
                    is_running.store(false, Ordering::Relaxed);
                    return;
                }
            }
            
            let handler = dptree::entry()
                .branch(Update::filter_message().endpoint(move |bot, msg| {
                    let users = registered_users.clone();
                    let reg_state = registration_state.clone();
                    let video_state = video_request_state.clone();
                    let emergency_state = emergency_stop_request_state.clone();
                    let hosts = hosts.clone();
                    let client = http_client.clone();
                    message_handler(bot, msg, users, reg_state, video_state, emergency_state, hosts, client)
                }))
                .branch(Update::filter_callback_query().endpoint(callback_handler));

            let mut dispatcher = Dispatcher::builder(bot, handler)
                .default_handler(|upd| async move {
                    println!("Unhandled update: {:?}", upd);
                })
                .build();

            dispatcher.dispatch().await;
            
            is_running.store(false, Ordering::Relaxed);
        });

        {
            let mut handle_guard = task_handle.lock().await;
            *handle_guard = Some(handle);
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Err("Bot is not running".to_string());
        }

        self.is_running.store(false, Ordering::Relaxed);
        
        let mut handle_guard = self.task_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub async fn start_registration(&self) -> Result<String, String> {
        let mut reg_state = self.registration_state.lock().await;
        if reg_state.is_active {
            return Err("Registration is already active".to_string());
        }
        
        let code = reg_state.start_registration();
        Ok(code)
    }

    pub async fn stop_registration(&self) -> Result<(), String> {
        let mut reg_state = self.registration_state.lock().await;
        reg_state.finish_registration();
        Ok(())
    }

    pub async fn is_registration_active(&self) -> bool {
        let reg_state = self.registration_state.lock().await;
        reg_state.is_active && !reg_state.is_expired()
    }

    pub async fn get_registration_state(&self) -> crate::models::RegistrationState {
        let reg_state = self.registration_state.lock().await;
        reg_state.clone()
    }


    pub async fn save_users_to_file(&self) -> Result<(), String> {
        let users = self.registered_users.lock().await;
        
        // Save to config file instead of separate file
        let mut settings = crate::models::config::AppSettings::load()
            .map_err(|e| format!("Failed to load settings: {}", e))?;
        settings.telegram.registered_users = (*users).clone();
        settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
        
        Ok(())
    }

    pub async fn load_users_from_file(&self) -> Result<(), String> {
        // Load from config file instead of separate file
        let settings = crate::models::config::AppSettings::load()
            .map_err(|e| format!("Failed to load settings: {}", e))?;
        
        let mut registered_users = self.registered_users.lock().await;
        *registered_users = settings.telegram.registered_users;
        
        Ok(())
    }

    pub async fn get_registered_users(&self) -> Vec<TelegramUser> {
        let users = self.registered_users.lock().await;
        users.clone()
    }

    pub async fn add_user(&self, user: TelegramUser) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        
        // Check if user already exists
        if users.iter().any(|u| u.user_id == user.user_id) {
            return Err("User already exists".to_string());
        }
        
        users.push(user);
        drop(users); // Release the lock before calling save
        
        // Save users to file
        self.save_users_to_file().await?;
        Ok(())
    }

    pub async fn remove_user(&self, user_id: i64) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        users.retain(|user| user.user_id != user_id);
        drop(users); // Release the lock before calling save
        
        // Save users to file
        if let Err(e) = self.save_users_to_file().await {
            println!("Failed to save users to file after removal: {}", e);
        }
        
        Ok(())
    }

    pub async fn is_user_registered(&self, user_id: teloxide::types::UserId) -> bool {
        let users = self.registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id.0 as i64)
    }

    pub async fn get_hosts(&self) -> Result<Vec<crate::models::HostInfo>, String> {
        let hosts = self.hosts.lock().await;
        Ok(hosts.clone())
    }

    pub async fn send_notification_to_all_users(&self, title: &str, body: &str, host_ip: Option<&str>) -> Result<(), String> {
        let users = self.registered_users.lock().await;
        
        if users.is_empty() {
            return Ok(()); // No users to notify
        }

        // Escape special characters for MarkdownV2
        let escaped_title = title.replace("*", "\\*").replace("_", "\\_").replace("[", "\\[").replace("]", "\\]").replace("(", "\\(").replace(")", "\\)").replace("~", "\\~").replace("`", "\\`").replace(">", "\\>").replace("#", "\\#").replace("+", "\\+").replace("-", "\\-").replace("=", "\\=").replace("|", "\\|").replace("{", "\\{").replace("}", "\\}").replace(".", "\\.").replace("!", "\\!");
        let escaped_body = body.replace("*", "\\*").replace("_", "\\_").replace("[", "\\[").replace("]", "\\]").replace("(", "\\(").replace(")", "\\)").replace("~", "\\~").replace("`", "\\`").replace(">", "\\>").replace("#", "\\#").replace("+", "\\+").replace("-", "\\-").replace("=", "\\=").replace("|", "\\|").replace("{", "\\{").replace("}", "\\}").replace(".", "\\.").replace("!", "\\!");
        
        let message = format!("🔔 *{}*\n\n{}", escaped_title, escaped_body);
        
        // Try to get webcam image if host_ip is provided
        let webcam_image = if let Some(ip) = host_ip {
            get_webcam_image(ip, &self.http_client).await.ok()
        } else {
            None
        };
        
        for user in users.iter() {
            // Only send notification if user has notifications enabled
            if !user.notifications_enabled {
                continue;
            }
            
            let result = if let Some(image_data) = &webcam_image {
                // Send message with photo
                self.bot.send_photo(teloxide::types::ChatId(user.user_id), teloxide::types::InputFile::memory(image_data.clone()))
                    .caption(&message)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await
            } else {
                // Send text message only
                self.bot.send_message(teloxide::types::ChatId(user.user_id), &message)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await
            };
            
            if let Err(e) = result {
                eprintln!("Failed to send notification to user {}: {}", user.user_id, e);
            }
        }
        
        Ok(())
    }

    pub async fn update_user_notifications(&self, user_id: i64, notifications_enabled: bool) -> Result<(), String> {
        let mut users = self.registered_users.lock().await;
        
        if let Some(user) = users.iter_mut().find(|u| u.user_id == user_id) {
            user.notifications_enabled = notifications_enabled;
            drop(users); // Release the lock before calling save
            
            // Save users to file
            self.save_users_to_file().await?;
            Ok(())
        } else {
            Err(format!("User {} not found", user_id))
        }
    }
}

// Function to get hosts from the main application
async fn get_hosts_from_app(hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>) -> Result<Vec<crate::models::HostInfo>, String> {
    let hosts_guard = hosts.lock().await;
    Ok(hosts_guard.clone())
}

// Standalone function to save users to file
async fn save_users_to_file(users: &[TelegramUser]) -> Result<(), String> {
    // Save to config file instead of separate file
    let mut settings = crate::models::config::AppSettings::load()
        .map_err(|e| format!("Failed to load settings: {}", e))?;
    settings.telegram.registered_users = users.to_vec();
    settings.save().map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}

async fn message_handler(
    bot: Bot, 
    msg: Message, 
    registered_users: Arc<Mutex<Vec<TelegramUser>>>,
    registration_state: Arc<Mutex<RegistrationState>>,
    video_request_state: Arc<Mutex<VideoRequestState>>,
    emergency_stop_request_state: Arc<Mutex<EmergencyStopRequestState>>,
    hosts: Arc<Mutex<Vec<crate::models::HostInfo>>>,
    http_client: reqwest::Client
) -> ResponseResult<()> {
    let user_id = match msg.from() {
        Some(user) => user.id,
        None => return Ok(()), // Ignore messages without sender
    };
    let is_registered = {
        let users = registered_users.lock().await;
        users.iter().any(|user| user.user_id == user_id.0 as i64)
    };

    if let Some(text) = msg.text() {
        // Handle commands
        if text.starts_with('/') {
            if let Ok(command) = Command::parse(text, "") {
                match command {
                    Command::Hosts => {
                        if is_registered {
                            // Get hosts from the main application
                            match get_hosts_from_app(hosts.clone()).await {
                                Ok(hosts) => {
                                    if hosts.is_empty() {
                                        bot.send_message(msg.chat.id, "No hosts found. Make sure the application is running and has scanned for hosts.")
                                            .await?;
                                    } else {
                                        let mut message = String::from("📡 Available Hosts:\n\n");
                                        for (i, host) in hosts.iter().enumerate() {
                                            let status_emoji = match host.status.as_str() {
                                                "online" => "🟢",
                                                "offline" => "🔴",
                                                "printing" => "🟡",
                                                "paused" => "⏸️",
                                                "error" => "❌",
                                                "cancelling" => "⏹️",
                                                _ => "⚪"
                                            };
                                            
                                            message.push_str(&format!(
                                                "{}. {} {}\n   IP: {}\n   Status: {}\n   Device: {}\n\n",
                                                i + 1,
                                                status_emoji,
                                                host.hostname,
                                                host.ip_address,
                                                host.status,
                                                host.device_status
                                            ));
                                        }
                                        
                                        bot.send_message(msg.chat.id, message)
                                            .await?;
                                    }
                                }
                                Err(e) => {
                                    bot.send_message(msg.chat.id, format!("Error getting hosts: {}", e))
                                        .await?;
                                }
                            }
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                    Command::NotificationOn => {
                        if is_registered {
                            let mut users = registered_users.lock().await;
                            if let Some(user) = users.iter_mut().find(|u| u.user_id == user_id.0 as i64) {
                                user.notifications_enabled = true;
                                drop(users); // Release the lock before calling save
                                
                                // Save users to file
                                let users_to_save = registered_users.lock().await.clone();
                                if let Err(e) = save_users_to_file(&users_to_save).await {
                                    println!("Failed to save users to file: {}", e);
                                }
                                
                                bot.send_message(msg.chat.id, "✅ Notifications enabled! You will now receive status change notifications.")
                                    .await?;
                            } else {
                                bot.send_message(msg.chat.id, "❌ User not found. Please register first.")
                                    .await?;
                            }
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                    Command::NotificationOff => {
                        if is_registered {
                            let mut users = registered_users.lock().await;
                            if let Some(user) = users.iter_mut().find(|u| u.user_id == user_id.0 as i64) {
                                user.notifications_enabled = false;
                                drop(users); // Release the lock before calling save
                                
                                // Save users to file
                                let users_to_save = registered_users.lock().await.clone();
                                if let Err(e) = save_users_to_file(&users_to_save).await {
                                    println!("Failed to save users to file: {}", e);
                                }
                                
                                bot.send_message(msg.chat.id, "🔕 Notifications disabled! You will no longer receive status change notifications.")
                                    .await?;
                            } else {
                                bot.send_message(msg.chat.id, "❌ User not found. Please register first.")
                                    .await?;
                            }
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                    Command::Image => {
                        if is_registered {
                            // Get hosts from the main application
                            match get_hosts_from_app(hosts.clone()).await {
                                Ok(hosts) => {
                                    if hosts.is_empty() {
                                        bot.send_message(msg.chat.id, "No hosts found. Make sure the application is running and has scanned for hosts.")
                                            .await?;
                                    } else {
                                        let mut message = String::from("📷 Select host for image:\n\n");
                                        for (i, host) in hosts.iter().enumerate() {
                                            let status_emoji = match host.status.as_str() {
                                                "online" => "🟢",
                                                "offline" => "🔴",
                                                "printing" => "🟡",
                                                "paused" => "⏸️",
                                                "error" => "❌",
                                                "cancelling" => "⏹️",
                                                _ => "⚪"
                                            };
                                            
                                            message.push_str(&format!(
                                                "{}. {} {}\n   IP: {}\n   Status: {}\n\n",
                                                i + 1,
                                                status_emoji,
                                                host.hostname,
                                                host.ip_address,
                                                host.status
                                            ));
                                        }
                                        message.push_str("Please enter the number of the host you want to get image from:");
                                        
                                        bot.send_message(msg.chat.id, message)
                                            .await?;
                                        
                                        // Start video request state
                                        let mut video_state = video_request_state.lock().await;
                                        video_state.start_video_request(user_id.0 as i64);
                                    }
                                }
                                Err(e) => {
                                    bot.send_message(msg.chat.id, format!("Error getting hosts: {}", e))
                                        .await?;
                                }
                            }
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                    Command::EmergencyStop => {
                        if is_registered {
                            // Get hosts from the main application
                            match get_hosts_from_app(hosts.clone()).await {
                                Ok(hosts) => {
                                    if hosts.is_empty() {
                                        bot.send_message(msg.chat.id, "No hosts found. Make sure the application is running and has scanned for hosts.")
                                            .await?;
                                    } else {
                                        let mut message = String::from("🛑 Select host for emergency stop:\n\n");
                                        for (i, host) in hosts.iter().enumerate() {
                                            let status_emoji = match host.status.as_str() {
                                                "online" => "🟢",
                                                "offline" => "🔴",
                                                "printing" => "🟡",
                                                "paused" => "⏸️",
                                                "error" => "❌",
                                                "cancelling" => "⏹️",
                                                _ => "⚪"
                                            };
                                            
                                            message.push_str(&format!(
                                                "{}. {} {}\n   IP: {}\n   Status: {}\n\n",
                                                i + 1,
                                                status_emoji,
                                                host.hostname,
                                                host.ip_address,
                                                host.status
                                            ));
                                        }
                                        message.push_str("⚠️ WARNING: This will immediately stop the printer!\nPlease enter the number of the host:");
                                        
                                        bot.send_message(msg.chat.id, message)
                                            .await?;
                                        
                                        // Start emergency stop request state
                                        let mut emergency_state = emergency_stop_request_state.lock().await;
                                        emergency_state.start_emergency_stop_request(user_id.0 as i64);
                                    }
                                }
                                Err(e) => {
                                    bot.send_message(msg.chat.id, format!("Error getting hosts: {}", e))
                                        .await?;
                                }
                            }
                        } else {
                            // Ignore unregistered users
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            // Handle registration code or ignore unregistered users
            if !is_registered {
                let mut reg_state = registration_state.lock().await;
                if reg_state.is_active && !reg_state.is_expired() {
                    if reg_state.verify_code(text) {
                        // Registration successful
                        reg_state.finish_registration();
                        
                        // Add user to registered users
                        let from_user = match msg.from() {
                            Some(user) => user,
                            None => return Ok(()), // Ignore messages without sender
                        };
                        let user = TelegramUser::from_teloxide_user(
                            user_id,
                            from_user.username.clone(),
                            from_user.first_name.clone(),
                            from_user.last_name.clone(),
                        );
                        
                        let mut users = registered_users.lock().await;
                        users.push(user.clone());
                        
                        bot.send_message(msg.chat.id, "Registration successful! You are now registered.")
                            .await?;
                        
                        // Save users to file
                        let users_to_save = users.clone();
                        drop(users); // Release the lock before calling save
                        if let Err(e) = save_users_to_file(&users_to_save).await {
                            println!("Failed to save users to file: {}", e);
                        }
                        
                        // Notify frontend that registration is complete
                        println!("Registration completed for user: {}", user_id.0);
                    } else {
                        // Check if max attempts reached
                        if reg_state.attempts >= reg_state.max_attempts {
                            reg_state.finish_registration();
                            bot.send_message(msg.chat.id, "Too many failed attempts. Registration has been cancelled.")
                                .await?;
                        } else {
                            let remaining = reg_state.max_attempts - reg_state.attempts;
                            bot.send_message(msg.chat.id, format!("Invalid code. {} attempts remaining.", remaining))
                                .await?;
                        }
                    }
                } else {
                    // Ignore unregistered users - don't send any response
                    return Ok(());
                }
            } else {
                // User is registered, check if they're in video request state or emergency stop request state
                let video_state = video_request_state.lock().await;
                let emergency_state = emergency_stop_request_state.lock().await;
                
                if video_state.is_active && video_state.user_id == user_id.0 as i64 && !video_state.is_expired() {
                    drop(video_state);
                    drop(emergency_state);
                    
                    // Parse host number for image request
                    if let Ok(host_number) = text.parse::<usize>() {
                        match get_hosts_from_app(hosts.clone()).await {
                            Ok(hosts) => {
                                if host_number > 0 && host_number <= hosts.len() {
                                    let selected_host = &hosts[host_number - 1];
                                    
                                    // Finish video request state
                                    let mut video_state = video_request_state.lock().await;
                                    video_state.finish_video_request();
                                    
                                    bot.send_message(msg.chat.id, format!("📷 Getting image from {}...", selected_host.hostname))
                                        .await?;
                                    
                                    // Get image from webcam
                                    match get_webcam_image(&selected_host.ip_address, &http_client).await {
                                        Ok(image_data) => {
                                            // Send image to user
                                            bot.send_photo(msg.chat.id, teloxide::types::InputFile::memory(image_data))
                                                .caption(format!("📷 Image from {}", selected_host.hostname))
                                                .await?;
                                        }
                                        Err(e) => {
                                            bot.send_message(msg.chat.id, format!("❌ Failed to get image: {}", e))
                                                .await?;
                                        }
                                    }
                                } else {
                                    bot.send_message(msg.chat.id, "❌ Invalid host number. Please try again.")
                                        .await?;
                                }
                            }
                            Err(e) => {
                                bot.send_message(msg.chat.id, format!("Error getting hosts: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "❌ Please enter a valid number.")
                            .await?;
                    }
                } else if emergency_state.is_active && emergency_state.user_id == user_id.0 as i64 && !emergency_state.is_expired() {
                    drop(video_state);
                    drop(emergency_state);
                    
                    // Parse host number for emergency stop request
                    if let Ok(host_number) = text.parse::<usize>() {
                        match get_hosts_from_app(hosts.clone()).await {
                            Ok(hosts) => {
                                if host_number > 0 && host_number <= hosts.len() {
                                    let selected_host = &hosts[host_number - 1];
                                    
                                    // Finish emergency stop request state
                                    let mut emergency_state = emergency_stop_request_state.lock().await;
                                    emergency_state.finish_emergency_stop_request();
                                    
                                    bot.send_message(msg.chat.id, format!("🛑 Sending emergency stop to {}...", selected_host.hostname))
                                        .await?;
                                    
                                    // Send emergency stop command
                                    match send_emergency_stop(&selected_host.ip_address, &http_client).await {
                                        Ok(_) => {
                                            bot.send_message(msg.chat.id, format!("✅ Emergency stop sent to {}!", selected_host.hostname))
                                                .await?;
                                        }
                                        Err(e) => {
                                            bot.send_message(msg.chat.id, format!("❌ Failed to send emergency stop: {}", e))
                                                .await?;
                                        }
                                    }
                                } else {
                                    bot.send_message(msg.chat.id, "❌ Invalid host number. Please try again.")
                                        .await?;
                                }
                            }
                            Err(e) => {
                                bot.send_message(msg.chat.id, format!("Error getting hosts: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "❌ Please enter a valid number.")
                            .await?;
                    }
                } else {
                    // User is registered but not in any request state
                    bot.send_message(msg.chat.id, format!("Hello registered user! Your User ID: {}", user_id))
                        .await?;
                }
            }
        }
    }
    Ok(())
}

/// Validates IP address to prevent SSRF attacks
/// Only allows private network ranges and localhost
fn is_valid_ip_address(ip: &str) -> bool {
    use std::net::IpAddr;
    
    let ip_addr = match ip.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(_) => return false,
    };
    
    match ip_addr {
        IpAddr::V4(ipv4) => {
            // Allow localhost
            if ipv4.is_loopback() {
                return true;
            }
            
            // Allow private network ranges
            if ipv4.is_private() {
                return true;
            }
            
            // Allow link-local addresses
            if ipv4.is_link_local() {
                return true;
            }
            
            false
        }
        IpAddr::V6(ipv6) => {
            // Allow localhost
            if ipv6.is_loopback() {
                return true;
            }
            
            // Allow unique local addresses (fc00::/7)
            if ipv6.is_unique_local() {
                return true;
            }
            
            // Allow link-local addresses (fe80::/10)
            if ipv6.is_unicast_link_local() {
                return true;
            }
            
            false
        }
    }
}

async fn get_webcam_image(ip_address: &str, client: &reqwest::Client) -> Result<Vec<u8>, String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/webcam/?action=snapshot", ip_address);
    
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to request image: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let image_data = response.bytes()
        .await
        .map_err(|e| format!("Failed to read image data: {}", e))?
        .to_vec();
    
    Ok(image_data)
}

async fn send_emergency_stop(ip_address: &str, client: &reqwest::Client) -> Result<(), String> {
    // Validate IP address to prevent SSRF attacks
    if !is_valid_ip_address(ip_address) {
        return Err("Invalid IP address".to_string());
    }
    
    let url = format!("http://{}/printer/emergency_stop", ip_address);
    
    let response = client.post(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send emergency stop request: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data {
        bot.answer_callback_query(q.id).await?;
        if let Some(msg) = q.message {
            bot.edit_message_text(msg.chat.id, msg.id, format!("You pressed: {}", data))
                .await?;
        }
    }
    Ok(())
}
