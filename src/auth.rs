// Authentication state management and services
use yew::prelude::*;
use crate::types::User;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    Unauthenticated,
    Authenticating,
    Authenticated(User),
    AuthenticationFailed(String),
}

impl Default for AuthState {
    fn default() -> Self {
        AuthState::Unauthenticated
    }
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated(_))
    }
    
    pub fn get_user(&self) -> Option<&User> {
        match self {
            AuthState::Authenticated(user) => Some(user),
            _ => None,
        }
    }
    
    pub fn get_error(&self) -> Option<&String> {
        match self {
            AuthState::AuthenticationFailed(error) => Some(error),
            _ => None,
        }
    }
}

pub struct AuthToken {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl AuthToken {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}
