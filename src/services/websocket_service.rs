// WebSocket service - handles real-time communication with the PrimoPoker backend
use yew::prelude::*;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_timers::future::TimeoutFuture;
use futures::{SinkExt, StreamExt};
use wasm_bindgen_futures::spawn_local;

use crate::types::{ClientMessage, ServerMessage, PokerError};
use crate::app::AppMsg;
use crate::game::GameState;

const WS_URL: &str = "wss://ws.primopoker.com"; // Replace with actual WebSocket URL
const HEARTBEAT_INTERVAL: u32 = 30; // seconds
const RECONNECT_DELAY: u32 = 5; // seconds

pub struct WebSocketService {
    link: Scope<crate::app::App>,
    websocket: Option<WebSocket>,
    is_connected: bool,
    reconnection_attempts: u32,
    max_reconnection_attempts: u32,
}

impl WebSocketService {
    pub fn new(link: Scope<crate::app::App>) -> Self {
        let mut service = Self {
            link,
            websocket: None,
            is_connected: false,
            reconnection_attempts: 0,
            max_reconnection_attempts: 10,
        };
        
        // Start connection attempt
        service.connect();
        service
    }
    
    pub fn connect(&mut self) {
        let link = self.link.clone();
        
        spawn_local(async move {
            match WebSocket::open(WS_URL) {
                Ok(ws) => {
                    log::info!("WebSocket connection established");
                    link.send_message(AppMsg::ConnectionEstablished);
                    
                    let (mut write, mut read) = ws.split();
                    
                    // Start heartbeat
                    let heartbeat_link = link.clone();
                    spawn_local(async move {
                        loop {
                            TimeoutFuture::new(HEARTBEAT_INTERVAL * 1000).await;
                            
                            let heartbeat_msg = ClientMessage::Heartbeat;
                            if let Ok(json) = serde_json::to_string(&heartbeat_msg) {
                                if write.send(Message::Text(json)).await.is_err() {
                                    log::error!("Failed to send heartbeat");
                                    break;
                                }
                            }
                        }
                    });
                    
                    // Handle incoming messages
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if let Err(e) = Self::handle_message(&link, &text).await {
                                    log::error!("Error handling message: {}", e);
                                    link.send_message(AppMsg::Error(e.to_string()));
                                }
                            }
                            Ok(Message::Bytes(_)) => {
                                log::warn!("Received unexpected binary message");
                            }
                            Err(e) => {
                                log::error!("WebSocket error: {:?}", e);
                                link.send_message(AppMsg::ConnectionLost);
                                break;
                            }
                        }
                    }
                    
                    log::info!("WebSocket connection closed");
                    link.send_message(AppMsg::ConnectionLost);
                }
                Err(e) => {
                    log::error!("Failed to connect to WebSocket: {:?}", e);
                    link.send_message(AppMsg::Error(format!("Connection failed: {}", e)));
                    
                    // Attempt reconnection
                    Self::attempt_reconnection(link).await;
                }
            }
        });
    }
    
    async fn handle_message(
        link: &Scope<crate::app::App>,
        message: &str,
    ) -> Result<(), PokerError> {
        let server_message: ServerMessage = serde_json::from_str(message)
            .map_err(|e| PokerError::SerializationError(e))?;
        
        match server_message {
            ServerMessage::AuthenticationSuccess { user } => {
                link.send_message(AppMsg::UserLoggedIn(user));
            }
            ServerMessage::AuthenticationFailed { error } => {
                link.send_message(AppMsg::Error(format!("Authentication failed: {}", error)));
            }
            ServerMessage::RoomJoined { room, players } => {
                let mut game_state = GameState::new();
                game_state.join_room(room, players);
                link.send_message(AppMsg::GameStateUpdated(game_state));
            }
            ServerMessage::RoomLeft => {
                let game_state = GameState::new();
                link.send_message(AppMsg::GameStateUpdated(game_state));
            }
            ServerMessage::PlayerJoined { player } => {
                // This would need to be handled by updating the current game state
                log::info!("Player joined: {}", player.username);
            }
            ServerMessage::PlayerLeft { player_id } => {
                // This would need to be handled by updating the current game state
                log::info!("Player left: {}", player_id);
            }
            ServerMessage::GameStateUpdate { phase, community_cards, pot } => {
                // This would need to be handled by updating the current game state
                log::info!("Game state updated: {:?}", phase);
            }
            ServerMessage::PlayerActionRequired { time_limit } => {
                log::info!("Action required with {} seconds", time_limit);
            }
            ServerMessage::PlayerActionUpdate { action } => {
                log::info!("Player action: {:?}", action);
            }
            ServerMessage::HandResult { winners, pot_distribution } => {
                log::info!("Hand completed with {} winners", winners.len());
            }
            ServerMessage::ChatMessage { player_id, username, message, timestamp } => {
                log::info!("Chat from {}: {}", username, message);
            }
            ServerMessage::Pong => {
                // Heartbeat response - connection is alive
            }
            ServerMessage::Error { message } => {
                link.send_message(AppMsg::Error(message));
            }
            ServerMessage::Disconnect { reason } => {
                link.send_message(AppMsg::Error(format!("Disconnected: {}", reason)));
                link.send_message(AppMsg::ConnectionLost);
            }
            _ => {
                log::info!("Unhandled server message: {:?}", server_message);
            }
        }
        
        Ok(())
    }
    
    async fn attempt_reconnection(link: Scope<crate::app::App>) {
        TimeoutFuture::new(RECONNECT_DELAY * 1000).await;
        
        // This would trigger a reconnection attempt
        // In a real implementation, you'd want to track reconnection attempts
        // and implement exponential backoff
        log::info!("Attempting to reconnect...");
    }
    
    pub async fn send_message(&mut self, message: ClientMessage) -> Result<(), PokerError> {
        if let Some(ref mut ws) = self.websocket {
            let json = serde_json::to_string(&message)
                .map_err(|e| PokerError::SerializationError(e))?;
            
            // This is a simplified version - in reality you'd need to handle the split websocket
            // ws.send(Message::Text(json)).await
            //     .map_err(|e| PokerError::WebSocketError(format!("Send failed: {:?}", e)))?;
            
            Ok(())
        } else {
            Err(PokerError::WebSocketError("Not connected".to_string()))
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    pub fn disconnect(&mut self) {
        if let Some(ws) = self.websocket.take() {
            // Close the websocket connection
            drop(ws);
            self.is_connected = false;
        }
    }
}
