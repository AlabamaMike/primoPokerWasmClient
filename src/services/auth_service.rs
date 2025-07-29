// Authentication service - handles user login, registration, and token management
use yew::prelude::*;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde_json::json;

use crate::types::{User, LoginCredentials, RegisterData, PokerError};
use crate::app::AppMsg;

const API_BASE_URL: &str = "https://api.primopoker.com"; // Replace with actual API URL

pub struct AuthService {
    link: Scope<crate::app::App>,
}

impl AuthService {
    pub fn new(link: Scope<crate::app::App>) -> Self {
        Self { link }
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
                .map_err(|e| PokerError::SerializationError(serde_json::Error::from(e)))?;
            
            // Store authentication token
            if let Some(auth_header) = response.headers().get("authorization") {
                let _ = LocalStorage::set("primo_poker_token", auth_header);
            }
            
            self.link.send_message(AppMsg::UserLoggedIn(user.clone()));
            Ok(user)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Login failed".to_string());
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
                .map_err(|e| PokerError::SerializationError(serde_json::Error::from(e)))?;
            
            // Store authentication token
            if let Some(auth_header) = response.headers().get("authorization") {
                let _ = LocalStorage::set("primo_poker_token", auth_header);
            }
            
            self.link.send_message(AppMsg::UserLoggedIn(user.clone()));
            Ok(user)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Registration failed".to_string());
            Err(PokerError::AuthenticationError(error_msg))
        }
    }
    
    pub fn logout(&self) {
        // Clear stored authentication data
        LocalStorage::delete("primo_poker_token");
        LocalStorage::delete("primo_poker_user");
        
        self.link.send_message(AppMsg::UserLoggedOut);
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
                .map_err(|e| PokerError::SerializationError(serde_json::Error::from(e)))?;
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
                    .map_err(|e| PokerError::SerializationError(serde_json::Error::from(e)))?;
                
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
