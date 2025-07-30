// Authentication service - handles user login, registration, and token management
use yew::html::Scope;
use yew::Callback;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::future::TimeoutFuture;

use crate::types::{User, LoginCredentials, RegisterData, PokerError};
use crate::app::AppMsg;

const API_BASE_URL: &str = "https://api.primopoker.com"; // Replace with actual API URL

#[derive(Clone)]
pub struct AuthService {
    on_auth_success: Callback<User>,
    on_auth_error: Callback<String>,
}

impl AuthService {
    pub fn new(on_auth_success: Callback<User>, on_auth_error: Callback<String>) -> Self {
        Self { 
            on_auth_success,
            on_auth_error 
        }
    }
    
    pub fn new_for_app(link: Scope<crate::app::App>) -> Self {
        let on_success = link.callback(|user: User| AppMsg::UserLoggedIn(user));
        let on_error = link.callback(|error: String| AppMsg::Error(error));
        Self::new(on_success, on_error)
    }
    
    
    pub async fn login(&self, credentials: LoginCredentials) -> Result<User, PokerError> {
        // Mock authentication for Phase 1 testing
        // In production, this would make real API calls
        
        // Simulate network delay
        TimeoutFuture::new(500).await;
        
        // Basic validation
        if credentials.username.trim().is_empty() || credentials.password.is_empty() {
            let error_msg = "Invalid credentials".to_string();
            self.on_auth_error.emit(error_msg.clone());
            return Err(PokerError::AuthenticationError(error_msg));
        }
        
        // For testing, accept any non-empty credentials
        let user = User {
            id: uuid::Uuid::new_v4(),
            username: credentials.username.clone(),
            email: format!("{}@primopoker.com", credentials.username),
            display_name: credentials.username.clone(),
            avatar_url: None,
            chips: 10000, // Starting chips
            level: 1,
            experience: 0,
            created_at: chrono::Utc::now(),
            last_active: chrono::Utc::now(),
        };
        
        // Store mock token
        let _ = LocalStorage::set("primo_poker_token", "mock_jwt_token_12345");
        
        self.on_auth_success.emit(user.clone());
        Ok(user)
    }
    
    pub async fn register(&self, register_data: RegisterData) -> Result<User, PokerError> {
        // Mock registration for Phase 1 testing
        // In production, this would make real API calls
        
        // Simulate network delay
        TimeoutFuture::new(750).await;
        
        // Basic validation
        if register_data.username.trim().is_empty() 
            || register_data.email.trim().is_empty()
            || register_data.password.is_empty() 
            || register_data.display_name.trim().is_empty() {
            let error_msg = "All fields are required".to_string();
            self.on_auth_error.emit(error_msg.clone());
            return Err(PokerError::AuthenticationError(error_msg));
        }
        
        // Mock email validation
        if !register_data.email.contains('@') {
            let error_msg = "Invalid email address".to_string();
            self.on_auth_error.emit(error_msg.clone());
            return Err(PokerError::AuthenticationError(error_msg));
        }
        
        // For testing, create a new user
        let user = User {
            id: uuid::Uuid::new_v4(),
            username: register_data.username.clone(),
            email: register_data.email.clone(),
            display_name: register_data.display_name.clone(),
            avatar_url: None,
            chips: 10000, // Starting chips
            level: 1,
            experience: 0,
            created_at: chrono::Utc::now(),
            last_active: chrono::Utc::now(),
        };
        
        // Store mock token
        let _ = LocalStorage::set("primo_poker_token", "mock_jwt_token_12345");
        
        self.on_auth_success.emit(user.clone());
        Ok(user)
    }
    
    pub fn get_stored_token(&self) -> Option<String> {
        LocalStorage::get("primo_poker_token").ok()
    }
    
    pub async fn verify_token(&self, token: &str) -> Result<User, PokerError> {
        let response = Request::get(&format!("{}/auth/verify", API_BASE_URL))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| PokerError::NetworkError(e.to_string()))?;

        if response.ok() {
            let user: User = response
                .json()
                .await
                .map_err(|e| PokerError::NetworkError(e.to_string()))?;
            Ok(user)
        } else {
            Err(PokerError::AuthenticationError("Token verification failed".to_string()))
        }
    }
    
    pub async fn refresh_token(&self) -> Result<String, PokerError> {
        if let Some(current_token) = self.get_stored_token() {
            let response = Request::post(&format!("{}/auth/refresh", API_BASE_URL))
                .header("Authorization", &format!("Bearer {}", current_token))
                .send()
                .await
                .map_err(|e| PokerError::NetworkError(e.to_string()))?;

            if response.ok() {
                let data: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| PokerError::NetworkError(e.to_string()))?;
                
                if let Some(new_token) = data["token"].as_str() {
                    let _ = LocalStorage::set("primo_poker_token", new_token);
                    Ok(new_token.to_string())
                } else {
                    Err(PokerError::AuthenticationError("Invalid refresh response".to_string()))
                }
            } else {
                Err(PokerError::AuthenticationError("Token refresh failed".to_string()))
            }
        } else {
            Err(PokerError::AuthenticationError("No token to refresh".to_string()))
        }
    }
}
