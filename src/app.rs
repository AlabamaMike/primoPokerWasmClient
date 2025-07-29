// Main App Component - Root of the Yew application
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::components::{Header, Footer, LoadingSpinner};
use crate::services::auth_service::AuthService;
use crate::services::websocket_service::WebSocketService;
use crate::auth::AuthState;
use crate::game::GameState;
use crate::types::{AppRoute, User};

pub struct App {
    auth_state: AuthState,
    game_state: GameState,
    loading: bool,
    error_message: Option<String>,
    _auth_service: AuthService,
    _websocket_service: WebSocketService,
}

pub enum AppMsg {
    UserLoggedIn(User),
    UserLoggedOut,
    GameStateUpdated(GameState),
    ConnectionEstablished,
    ConnectionLost,
    Error(String),
    ClearError,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let auth_service = AuthService::new(ctx.link().clone());
        let websocket_service = WebSocketService::new(ctx.link().clone());
        
        // Try to restore authentication state from local storage
        let auth_state = if let Ok(stored_user) = LocalStorage::get::<User>("primo_poker_user") {
            AuthState::Authenticated(stored_user)
        } else {
            AuthState::Unauthenticated
        };

        Self {
            auth_state,
            game_state: GameState::default(),
            loading: false,
            error_message: None,
            _auth_service: auth_service,
            _websocket_service: websocket_service,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UserLoggedIn(user) => {
                // Store user data in local storage
                let _ = LocalStorage::set("primo_poker_user", &user);
                self.auth_state = AuthState::Authenticated(user);
                self.loading = false;
                true
            }
            AppMsg::UserLoggedOut => {
                // Clear stored user data
                LocalStorage::delete("primo_poker_user");
                self.auth_state = AuthState::Unauthenticated;
                self.game_state = GameState::default();
                true
            }
            AppMsg::GameStateUpdated(new_state) => {
                self.game_state = new_state;
                true
            }
            AppMsg::ConnectionEstablished => {
                self.loading = false;
                self.error_message = None;
                true
            }
            AppMsg::ConnectionLost => {
                self.error_message = Some("Connection to server lost. Attempting to reconnect...".to_string());
                true
            }
            AppMsg::Error(error) => {
                self.error_message = Some(error);
                self.loading = false;
                true
            }
            AppMsg::ClearError => {
                self.error_message = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clear_error = ctx.link().callback(|_| AppMsg::ClearError);

        html! {
            <BrowserRouter>
                <div class="app">
                    <Header auth_state={self.auth_state.clone()} />
                    
                    // Error notification
                    if let Some(ref error) = self.error_message {
                        <div class="error-notification">
                            <span>{ error }</span>
                            <button onclick={on_clear_error} class="close-btn">{"×"}</button>
                        </div>
                    }
                    
                    // Loading spinner
                    if self.loading {
                        <LoadingSpinner />
                    }
                    
                    // Main content area
                    <main class="main-content">
                        <Switch<AppRoute> render={switch} />
                    </main>
                    
                    <Footer />
                </div>
            </BrowserRouter>
        }
    }
}

// Route switching function
fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <crate::components::pages::HomePage /> },
        AppRoute::Login => html! { <crate::components::auth::LoginPage /> },
        AppRoute::Register => html! { <crate::components::auth::RegisterPage /> },
        AppRoute::Lobby => html! { <crate::components::lobby::LobbyPage /> },
        AppRoute::Game { room_id } => html! { 
            <crate::components::game::GamePage room_id={room_id} /> 
        },
        AppRoute::Profile => html! { <crate::components::profile::ProfilePage /> },
        AppRoute::NotFound => html! { <crate::components::pages::NotFoundPage /> },
    }
}
