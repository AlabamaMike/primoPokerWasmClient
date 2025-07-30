use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::types::{User, GameRoom, RoomFilter, GameType, AppRoute};

pub struct LobbyPage {
    user: Option<User>,
    available_rooms: Vec<GameRoom>,
    filtered_rooms: Vec<GameRoom>,
    loading: bool,
    websocket_connected: bool,
    filter_criteria: RoomFilter,
    show_create_room_modal: bool,
    error_message: Option<String>,
}

pub enum LobbyMsg {
    LoadUserData,
    UserDataLoaded(User),
    RoomsUpdated(Vec<GameRoom>),
    WebSocketConnected,
    WebSocketDisconnected,
    UpdateFilter(RoomFilter),
    JoinRoom(String),
    ShowCreateRoomModal,
    HideCreateRoomModal,
    CreateRoom,
    Error(String),
    ClearError,
}

impl Component for LobbyPage {
    type Message = LobbyMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        
        // Try to load user data from local storage
        if let Ok(user) = LocalStorage::get::<User>("primo_poker_user") {
            link.send_message(LobbyMsg::UserDataLoaded(user));
        } else {
            // Redirect to login if no user data found
            let navigator = link.navigator().unwrap();
            navigator.push(&AppRoute::Login);
        }

        Self {
            user: None,
            available_rooms: Vec::new(),
            filtered_rooms: Vec::new(),
            loading: true,
            websocket_connected: false,
            filter_criteria: RoomFilter {
                min_stakes: None,
                max_stakes: None,
                max_players_filter: None,
                game_types: vec![GameType::TexasHoldem], // Default filter
                show_full_rooms: true,
                show_empty_rooms: true,
            },
            show_create_room_modal: false,
            error_message: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LobbyMsg::LoadUserData => {
                if let Ok(user) = LocalStorage::get::<User>("primo_poker_user") {
                    ctx.link().send_message(LobbyMsg::UserDataLoaded(user));
                }
                false
            }
            LobbyMsg::UserDataLoaded(user) => {
                self.user = Some(user);
                self.loading = false;
                
                // Connect to lobby WebSocket for real-time updates
                // self.websocket_service.connect_lobby(); // TODO: Implement
                true
            }
            LobbyMsg::RoomsUpdated(rooms) => {
                self.available_rooms = rooms;
                self.apply_filters();
                true
            }
            LobbyMsg::WebSocketConnected => {
                self.websocket_connected = true;
                self.error_message = None;
                true
            }
            LobbyMsg::WebSocketDisconnected => {
                self.websocket_connected = false;
                self.error_message = Some("Connection lost. Attempting to reconnect...".to_string());
                true
            }
            LobbyMsg::UpdateFilter(new_filter) => {
                self.filter_criteria = new_filter;
                self.apply_filters();
                true
            }
            LobbyMsg::JoinRoom(room_id) => {
                // Navigate to game room
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Game { room_id });
                false
            }
            LobbyMsg::ShowCreateRoomModal => {
                self.show_create_room_modal = true;
                true
            }
            LobbyMsg::HideCreateRoomModal => {
                self.show_create_room_modal = false;
                true
            }
            LobbyMsg::CreateRoom => {
                // TODO: Implement room creation
                self.show_create_room_modal = false;
                true
            }
            LobbyMsg::Error(error) => {
                self.error_message = Some(error);
                true
            }
            LobbyMsg::ClearError => {
                self.error_message = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        if self.loading {
            return html! {
                <div class="lobby-loading">
                    <div class="loading-spinner"></div>
                    <p>{"Loading lobby..."}</p>
                </div>
            };
        }

        let user = match &self.user {
            Some(user) => user,
            None => return html! { <div>{"Error: User not found"}</div> }
        };

        let on_join_room = link.callback(|room_id: String| LobbyMsg::JoinRoom(room_id));
        let on_show_create_modal = link.callback(|_| LobbyMsg::ShowCreateRoomModal);
        let on_hide_create_modal = link.callback(|_| LobbyMsg::HideCreateRoomModal);
        let on_clear_error = link.callback(|_| LobbyMsg::ClearError);

        html! {
            <div class="lobby-container">
                // Header Section
                <div class="lobby-header">
                    <div class="user-info">
                        <div class="user-avatar">
                            if let Some(ref avatar_url) = user.avatar_url {
                                <img src={avatar_url.clone()} alt="Avatar" />
                            } else {
                                <div class="default-avatar">{user.display_name.chars().next().unwrap_or('?')}</div>
                            }
                        </div>
                        <div class="user-details">
                            <h2 class="welcome-message">{format!("Welcome back, {}!", user.display_name)}</h2>
                            <div class="user-stats">
                                <span class="chip-count">{"💰 "}{user.chips}{" chips"}</span>
                                <span class="level">{"⭐ Level "}{user.level}</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="header-actions">
                        <div class="connection-status">
                            if self.websocket_connected {
                                <span class="status-indicator connected">{"🟢 Connected"}</span>
                            } else {
                                <span class="status-indicator disconnected">{"🔴 Disconnected"}</span>
                            }
                        </div>
                        <Link<AppRoute> to={AppRoute::Profile} classes="profile-button">
                            {"Profile"}
                        </Link<AppRoute>>
                    </div>
                </div>

                // Error notification
                if let Some(ref error) = self.error_message {
                    <div class="error-notification">
                        <span class="error-text">{error}</span>
                        <button onclick={on_clear_error} class="close-btn">{"×"}</button>
                    </div>
                }

                // Main Content
                <div class="lobby-content">
                    // Filters Section
                    <div class="filters-section">
                        <h3>{"Filter Games"}</h3>
                        <div class="filter-controls">
                            <div class="filter-group">
                                <label>{"Game Type:"}</label>
                                <select>
                                    <option value="texas_holdem">{"Texas Hold'em"}</option>
                                    <option value="omaha">{"Omaha"}</option>
                                    <option value="seven_card_stud">{"Seven Card Stud"}</option>
                                </select>
                            </div>
                            
                            <div class="filter-group">
                                <label>{"Stakes Range:"}</label>
                                <div class="stakes-inputs">
                                    <input type="number" placeholder="Min stakes" />
                                    <span>{" - "}</span>
                                    <input type="number" placeholder="Max stakes" />
                                </div>
                            </div>
                            
                            <div class="filter-group">
                                <label>{"Players:"}</label>
                                <select>
                                    <option value="">{"Any"}</option>
                                    <option value="2">{"2 Players"}</option>
                                    <option value="6">{"6 Players"}</option>
                                    <option value="9">{"9 Players"}</option>
                                </select>
                            </div>
                            
                            <div class="filter-toggles">
                                <label class="toggle-label">
                                    <input type="checkbox" checked={self.filter_criteria.show_empty_rooms} />
                                    <span>{"Show empty rooms"}</span>
                                </label>
                                <label class="toggle-label">
                                    <input type="checkbox" checked={self.filter_criteria.show_full_rooms} />
                                    <span>{"Show full rooms"}</span>
                                </label>
                            </div>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="action-buttons">
                        <button 
                            class="create-room-btn primary"
                            onclick={on_show_create_modal.clone()}
                        >
                            {"🏠 Create Room"}
                        </button>
                        <button class="quick-join-btn secondary">
                            {"⚡ Quick Join"}
                        </button>
                    </div>

                    // Room List
                    <div class="rooms-section">
                        <div class="rooms-header">
                            <h3>{format!("Available Games ({})", self.filtered_rooms.len())}</h3>
                            <button class="refresh-btn">{"🔄 Refresh"}</button>
                        </div>
                        
                        if self.filtered_rooms.is_empty() {
                            <div class="no-rooms">
                                <div class="empty-state">
                                    <h4>{"No games match your filters"}</h4>
                                    <p>{"Try adjusting your filter criteria or create a new room"}</p>
                                    <button 
                                        class="create-room-btn primary"
                                        onclick={on_show_create_modal}
                                    >
                                        {"Create New Room"}
                                    </button>
                                </div>
                            </div>
                        } else {
                            <div class="rooms-table">
                                <div class="table-header">
                                    <span class="col-name">{"Room Name"}</span>
                                    <span class="col-game">{"Game"}</span>
                                    <span class="col-stakes">{"Stakes"}</span>
                                    <span class="col-players">{"Players"}</span>
                                    <span class="col-action">{"Action"}</span>
                                </div>
                                
                                <div class="table-body">
                                    { for self.filtered_rooms.iter().map(|room| {
                                        let room_id = room.id.clone();
                                        let join_callback = {
                                            let room_id = room_id.clone();
                                            on_join_room.reform(move |_| room_id.clone())
                                        };
                                        
                                        html! {
                                            <div class="room-row">
                                                <div class="col-name">
                                                    <div class="room-name">{&room.name}</div>
                                                    if room.is_private {
                                                        <span class="private-badge">{"🔒 Private"}</span>
                                                    }
                                                </div>
                                                <div class="col-game">
                                                    <span class="game-type">{format!("{:?}", room.game_type)}</span>
                                                </div>
                                                <div class="col-stakes">
                                                    <span class="stakes">{format!("{}/{}", room.small_blind, room.big_blind)}</span>
                                                    <div class="buy-in">{format!("Buy-in: {}-{}", room.min_buy_in, room.max_buy_in)}</div>
                                                </div>
                                                <div class="col-players">
                                                    <span class={classes!(
                                                        "player-count",
                                                        if room.current_players == room.max_players { Some("full") } else { None },
                                                        if room.current_players == 0 { Some("empty") } else { None }
                                                    )}>
                                                        {format!("{}/{}", room.current_players, room.max_players)}
                                                    </span>
                                                </div>
                                                <div class="col-action">
                                                    if room.current_players < room.max_players {
                                                        <button 
                                                            class="join-btn primary"
                                                            onclick={join_callback}
                                                        >
                                                            {"Join"}
                                                        </button>
                                                    } else {
                                                        <button class="join-btn disabled" disabled=true>
                                                            {"Full"}
                                                        </button>
                                                    }
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            </div>
                        }
                    </div>
                </div>

                // Create Room Modal
                if self.show_create_room_modal {
                    <div class="modal-overlay" onclick={on_hide_create_modal.clone()}>
                        <div class="modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                            <div class="modal-header">
                                <h3>{"Create New Room"}</h3>
                                <button class="close-btn" onclick={on_hide_create_modal.clone()}>{"×"}</button>
                            </div>
                            <div class="modal-body">
                                <div class="form-group">
                                    <label>{"Room Name"}</label>
                                    <input type="text" placeholder="Enter room name" />
                                </div>
                                <div class="form-group">
                                    <label>{"Game Type"}</label>
                                    <select>
                                        <option value="texas_holdem">{"Texas Hold'em"}</option>
                                        <option value="omaha">{"Omaha"}</option>
                                    </select>
                                </div>
                                <div class="form-group">
                                    <label>{"Max Players"}</label>
                                    <select>
                                        <option value="2">{"2 Players"}</option>
                                        <option value="6">{"6 Players"}</option>
                                        <option value="9">{"9 Players"}</option>
                                    </select>
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>{"Small Blind"}</label>
                                        <input type="number" value="10" />
                                    </div>
                                    <div class="form-group">
                                        <label>{"Big Blind"}</label>
                                        <input type="number" value="20" />
                                    </div>
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>{"Min Buy-in"}</label>
                                        <input type="number" value="1000" />
                                    </div>
                                    <div class="form-group">
                                        <label>{"Max Buy-in"}</label>
                                        <input type="number" value="10000" />
                                    </div>
                                </div>
                                <div class="form-group">
                                    <label class="checkbox-label">
                                        <input type="checkbox" />
                                        <span>{"Private Room (requires password)"}</span>
                                    </label>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button class="cancel-btn" onclick={on_hide_create_modal}>{"Cancel"}</button>
                                <button class="create-btn primary">{"Create Room"}</button>
                            </div>
                        </div>
                    </div>
                }
            </div>
        }
    }
}

impl LobbyPage {
    fn apply_filters(&mut self) {
        self.filtered_rooms = self.available_rooms
            .iter()
            .filter(|room| {
                // Game type filter
                if !self.filter_criteria.game_types.is_empty() 
                    && !self.filter_criteria.game_types.contains(&room.game_type) {
                    return false;
                }

                // Stakes filter
                if let Some(min_stakes) = self.filter_criteria.min_stakes {
                    if room.big_blind < min_stakes {
                        return false;
                    }
                }
                if let Some(max_stakes) = self.filter_criteria.max_stakes {
                    if room.big_blind > max_stakes {
                        return false;
                    }
                }

                // Player count filter
                if let Some(max_players) = self.filter_criteria.max_players_filter {
                    if room.max_players != max_players {
                        return false;
                    }
                }

                // Empty/Full room filters
                if room.current_players == 0 && !self.filter_criteria.show_empty_rooms {
                    return false;
                }
                if room.current_players == room.max_players && !self.filter_criteria.show_full_rooms {
                    return false;
                }

                true
            })
            .cloned()
            .collect();
    }
}

// Mock data for development - replace with real WebSocket data
impl LobbyPage {
    #[allow(dead_code)]
    fn load_mock_rooms(&mut self) {
        use chrono::Utc;
        
        self.available_rooms = vec![
            GameRoom {
                id: "room1".to_string(),
                name: "High Stakes Hold'em".to_string(),
                game_type: GameType::TexasHoldem,
                current_players: 4,
                max_players: 9,
                small_blind: 50,
                big_blind: 100,
                min_buy_in: 5000,
                max_buy_in: 50000,
                is_private: false,
                is_active: true,
                created_at: Utc::now(),
            },
            GameRoom {
                id: "room2".to_string(),
                name: "Beginner Friendly".to_string(),
                game_type: GameType::TexasHoldem,
                current_players: 2,
                max_players: 6,
                small_blind: 5,
                big_blind: 10,
                min_buy_in: 500,
                max_buy_in: 2000,
                is_private: false,
                is_active: true,
                created_at: Utc::now(),
            },
            GameRoom {
                id: "room3".to_string(),
                name: "Private VIP Room".to_string(),
                game_type: GameType::TexasHoldem,
                current_players: 6,
                max_players: 6,
                small_blind: 100,
                big_blind: 200,
                min_buy_in: 10000,
                max_buy_in: 100000,
                is_private: true,
                is_active: true,
                created_at: Utc::now(),
            },
        ];
        self.apply_filters();
    }
}
