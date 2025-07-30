// Authentication service - handles user login, registration, and token management
use yew::html::Scope;
use yew::Callback;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};

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
        let response = Request::post(&format!("{}/auth/login", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&credentials)
            .map_err(|e| PokerError::NetworkError(e.to_string()))?
            .send()
            .await
            .map_err(|e| PokerError::NetworkError(e.to_string()))?;

        if response.ok() {
            let user: User = response
                .json()
                .await
                .map_err(|e| PokerError::NetworkError(e.to_string()))?;
            
            // Store authentication token
            if let Some(auth_header) = response.headers().get("authorization") {
                let _ = LocalStorage::set("primo_poker_token", auth_header);
            }
            
            self.on_auth_success.emit(user.clone());
            Ok(user)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Login failed".to_string());
            self.on_auth_error.emit(error_msg.clone());
            Err(PokerError::AuthenticationError(error_msg))
        }
    }
    
    pub async fn register(&self, register_data: RegisterData) -> Result<User, PokerError> {
        let response = Request::post(&format!("{}/auth/register", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&register_data)
            .map_err(|e| PokerError::NetworkError(e.to_string()))?
            .send()
            .await
            .map_err(|e| PokerError::NetworkError(e.to_string()))?;

        if response.ok() {
            let user: User = response
                .json()
                .await
                .map_err(|e| PokerError::NetworkError(e.to_string()))?;
            
            // Store authentication token
            if let Some(auth_header) = response.headers().get("authorization") {
                let _ = LocalStorage::set("primo_poker_token", auth_header);
            }
            
            self.on_auth_success.emit(user.clone());
            Ok(user)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Registration failed".to_string());
            self.on_auth_error.emit(error_msg.clone());
            Err(PokerError::AuthenticationError(error_msg))
        }
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
