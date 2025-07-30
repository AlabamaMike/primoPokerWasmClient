use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use chrono::Utc;

use crate::types::{User, GameRoom, RoomFilter, GameType, AppRoute};
use super::{RoomFilters, RoomCard};

#[derive(Clone, Default)]
pub struct CreateRoomForm {
    pub name: String,
    pub game_type: GameType,
    pub max_players: u8,
    pub small_blind: i64,
    pub big_blind: i64,
    pub min_buy_in: i64,
    pub max_buy_in: i64,
    pub is_private: bool,
    pub password: String,
}

pub struct LobbyPage {
    user: Option<User>,
    available_rooms: Vec<GameRoom>,
    filtered_rooms: Vec<GameRoom>,
    loading: bool,
    websocket_connected: bool,
    filter_criteria: RoomFilter,
    show_create_room_modal: bool,
    error_message: Option<String>,
    create_room_form: CreateRoomForm,
}

pub enum LobbyMsg {
    LoadUserData,
    UserDataLoaded(User),
    RoomsUpdated(Vec<GameRoom>),
    WebSocketConnected,
    WebSocketDisconnected,
    UpdateFilter(RoomFilter),
    ResetFilters,
    UpdateFilterGameType(GameType),
    UpdateFilterStakes(Option<i64>, Option<i64>),
    UpdateFilterPlayers(Option<u8>),
    ToggleEmptyRooms,
    ToggleFullRooms,
    JoinRoom(String),
    ShowCreateRoomModal,
    HideCreateRoomModal,
    UpdateCreateRoomForm(String, String), // field_name, value
    CreateRoom,
    RefreshRooms,
    QuickJoin,
    Error(String),
    ClearError,
}

impl Component for LobbyPage {
    type Message = LobbyMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        
        // Try to load authenticated user data from local storage
        if let Ok(user) = LocalStorage::get::<User>("primo_poker_user") {
            link.send_message(LobbyMsg::UserDataLoaded(user));
        }
        // Note: Removed immediate redirect to allow lobby to load even without user data
        // The app component handles authentication navigation

        // Create mock room data for testing
        let mock_rooms = vec![
            GameRoom {
                id: "room1".to_string(),
                name: "High Stakes Hold'em".to_string(), 
                game_type: GameType::TexasHoldem,
                small_blind: 25,
                big_blind: 50,
                min_buy_in: 1000,
                max_buy_in: 10000,
                current_players: 2,
                max_players: 9,
                is_private: false,
                is_active: true,
                created_at: Utc::now(),
            },
            GameRoom {
                id: "room2".to_string(),
                name: "Beginner Texas Hold'em".to_string(),
                game_type: GameType::TexasHoldem,
                small_blind: 1,
                big_blind: 2,
                min_buy_in: 50,
                max_buy_in: 200,
                current_players: 1,
                max_players: 6,
                is_private: false,
                is_active: true,
                created_at: Utc::now(),
            },
            GameRoom {
                id: "room3".to_string(),
                name: "Omaha Hi-Lo".to_string(),
                game_type: GameType::Omaha,
                small_blind: 10,
                big_blind: 20,
                min_buy_in: 400,
                max_buy_in: 2000,
                current_players: 3,
                max_players: 8,
                is_private: false,
                is_active: true,
                created_at: Utc::now(),
            },
        ];

        Self {
            user: None,
            available_rooms: mock_rooms.clone(),
            filtered_rooms: mock_rooms, // Initialize with mock data
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
            create_room_form: CreateRoomForm {
                name: String::new(),
                game_type: GameType::TexasHoldem,
                max_players: 6,
                small_blind: 10,
                big_blind: 20,
                min_buy_in: 1000,
                max_buy_in: 10000,
                is_private: false,
                password: String::new(),
            },
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
                
                // Load mock room data for development
                self.load_mock_rooms();
                
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
            LobbyMsg::ResetFilters => {
                self.filter_criteria = RoomFilter {
                    min_stakes: None,
                    max_stakes: None,
                    max_players_filter: None,
                    game_types: vec![GameType::TexasHoldem],
                    show_full_rooms: true,
                    show_empty_rooms: true,
                };
                self.apply_filters();
                true
            }
            LobbyMsg::UpdateFilterGameType(game_type) => {
                self.filter_criteria.game_types = vec![game_type];
                self.apply_filters();
                true
            }
            LobbyMsg::UpdateFilterStakes(min_stakes, max_stakes) => {
                self.filter_criteria.min_stakes = min_stakes;
                self.filter_criteria.max_stakes = max_stakes;
                self.apply_filters();
                true
            }
            LobbyMsg::UpdateFilterPlayers(max_players) => {
                self.filter_criteria.max_players_filter = max_players;
                self.apply_filters();
                true
            }
            LobbyMsg::ToggleEmptyRooms => {
                self.filter_criteria.show_empty_rooms = !self.filter_criteria.show_empty_rooms;
                self.apply_filters();
                true
            }
            LobbyMsg::ToggleFullRooms => {
                self.filter_criteria.show_full_rooms = !self.filter_criteria.show_full_rooms;
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
                // Reset form when closing
                self.create_room_form = CreateRoomForm::default();
                self.create_room_form.game_type = GameType::TexasHoldem;
                self.create_room_form.max_players = 6;
                self.create_room_form.small_blind = 10;
                self.create_room_form.big_blind = 20;
                self.create_room_form.min_buy_in = 1000;
                self.create_room_form.max_buy_in = 10000;
                true
            }
            LobbyMsg::UpdateCreateRoomForm(field, value) => {
                match field.as_str() {
                    "name" => self.create_room_form.name = value,
                    "game_type" => {
                        self.create_room_form.game_type = match value.as_str() {
                            "texas_holdem" => GameType::TexasHoldem,
                            "omaha" => GameType::Omaha,
                            "seven_card_stud" => GameType::SevenCardStud,
                            _ => GameType::TexasHoldem,
                        };
                    }
                    "max_players" => {
                        if let Ok(players) = value.parse::<u8>() {
                            self.create_room_form.max_players = players;
                        }
                    }
                    "small_blind" => {
                        if let Ok(blind) = value.parse::<i64>() {
                            self.create_room_form.small_blind = blind;
                        }
                    }
                    "big_blind" => {
                        if let Ok(blind) = value.parse::<i64>() {
                            self.create_room_form.big_blind = blind;
                        }
                    }
                    "min_buy_in" => {
                        if let Ok(buy_in) = value.parse::<i64>() {
                            self.create_room_form.min_buy_in = buy_in;
                        }
                    }
                    "max_buy_in" => {
                        if let Ok(buy_in) = value.parse::<i64>() {
                            self.create_room_form.max_buy_in = buy_in;
                        }
                    }
                    "is_private" => {
                        self.create_room_form.is_private = value == "true";
                    }
                    "password" => self.create_room_form.password = value,
                    _ => {}
                }
                true
            }
            LobbyMsg::CreateRoom => {
                // Validate form
                if self.create_room_form.name.trim().is_empty() {
                    self.error_message = Some("Room name is required".to_string());
                    return true;
                }
                if self.create_room_form.small_blind >= self.create_room_form.big_blind {
                    self.error_message = Some("Big blind must be larger than small blind".to_string());
                    return true;
                }
                if self.create_room_form.min_buy_in >= self.create_room_form.max_buy_in {
                    self.error_message = Some("Max buy-in must be larger than min buy-in".to_string());
                    return true;
                }

                // Create new room (mock implementation)
                let new_room = GameRoom {
                    id: format!("room_{}", chrono::Utc::now().timestamp()),
                    name: self.create_room_form.name.clone(),
                    game_type: self.create_room_form.game_type.clone(),
                    current_players: 0,
                    max_players: self.create_room_form.max_players,
                    small_blind: self.create_room_form.small_blind,
                    big_blind: self.create_room_form.big_blind,
                    min_buy_in: self.create_room_form.min_buy_in,
                    max_buy_in: self.create_room_form.max_buy_in,
                    is_private: self.create_room_form.is_private,
                    is_active: true,
                    created_at: chrono::Utc::now(),
                };

                // Add to available rooms
                self.available_rooms.push(new_room.clone());
                self.apply_filters();
                
                // Close modal and reset form
                self.show_create_room_modal = false;
                self.create_room_form = CreateRoomForm::default();
                self.create_room_form.game_type = GameType::TexasHoldem;
                self.create_room_form.max_players = 6;
                self.create_room_form.small_blind = 10;
                self.create_room_form.big_blind = 20;
                self.create_room_form.min_buy_in = 1000;
                self.create_room_form.max_buy_in = 10000;

                // Navigate to the newly created room
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Game { room_id: new_room.id });
                
                true
            }
            LobbyMsg::RefreshRooms => {
                // Refresh room list (mock implementation)
                self.load_mock_rooms();
                true
            }
            LobbyMsg::QuickJoin => {
                // Find first available room
                if let Some(room) = self.filtered_rooms.iter().find(|r| r.current_players < r.max_players) {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&AppRoute::Game { room_id: room.id.clone() });
                } else {
                    self.error_message = Some("No available rooms for quick join".to_string());
                }
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
        let on_refresh_rooms = link.callback(|_| LobbyMsg::RefreshRooms);
        let on_quick_join = link.callback(|_| LobbyMsg::QuickJoin);
        let on_create_room = link.callback(|_| LobbyMsg::CreateRoom);
        
        // Filter callbacks
        let on_game_type_change = link.callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let game_type = match select.value().as_str() {
                "texas_holdem" => GameType::TexasHoldem,
                "omaha" => GameType::Omaha,
                "seven_card_stud" => GameType::SevenCardStud,
                _ => GameType::TexasHoldem,
            };
            LobbyMsg::UpdateFilterGameType(game_type)
        });
        
        let on_toggle_empty = link.callback(|_: MouseEvent| LobbyMsg::ToggleEmptyRooms);
        let on_toggle_full = link.callback(|_: MouseEvent| LobbyMsg::ToggleFullRooms);
        
        // Create room form callbacks
        let on_form_input = link.callback(|(field, value): (String, String)| {
            LobbyMsg::UpdateCreateRoomForm(field, value)
        });

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
                    // Enhanced Filters Section
                    <RoomFilters 
                        current_filter={self.filter_criteria.clone()}
                        on_filter_change={link.callback(LobbyMsg::UpdateFilter)}
                        on_reset_filters={link.callback(|_| LobbyMsg::ResetFilters)}
                    />

                    // Action Buttons
                    <div class="action-buttons">
                        <button 
                            class="create-room-btn primary"
                            onclick={on_show_create_modal.clone()}
                        >
                            {"🏠 Create Room"}
                        </button>
                        <button class="quick-join-btn secondary" onclick={on_quick_join}>
                            {"⚡ Quick Join"}
                        </button>
                    </div>

                    // Room List
                    <div class="rooms-section">
                        <div class="rooms-header">
                            <h3>{format!("Available Games ({})", self.filtered_rooms.len())}</h3>
                            <button class="refresh-btn" onclick={on_refresh_rooms}>{"🔄 Refresh"}</button>
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
                            <div class="rooms-grid">
                                { for self.filtered_rooms.iter().map(|room| {
                                    html! {
                                        <RoomCard 
                                            room={room.clone()}
                                            on_join={on_join_room.clone()}
                                        />
                                    }
                                }) }
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
                                    <input 
                                        type="text" 
                                        placeholder="Enter room name"
                                        value={self.create_room_form.name.clone()}
                                        onchange={
                                            let on_form_input = on_form_input.clone();
                                            Callback::from(move |e: Event| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                on_form_input.emit(("name".to_string(), input.value()))
                                            })
                                        }
                                    />
                                </div>
                                <div class="form-group">
                                    <label>{"Game Type"}</label>
                                    <select 
                                        onchange={
                                            let on_form_input = on_form_input.clone();
                                            Callback::from(move |e: Event| {
                                                let select: HtmlInputElement = e.target_unchecked_into();
                                                on_form_input.emit(("game_type".to_string(), select.value()))
                                            })
                                        }
                                    >
                                        <option value="texas_holdem" selected={matches!(self.create_room_form.game_type, GameType::TexasHoldem)}>
                                            {"Texas Hold'em"}
                                        </option>
                                        <option value="omaha" selected={matches!(self.create_room_form.game_type, GameType::Omaha)}>
                                            {"Omaha"}
                                        </option>
                                    </select>
                                </div>
                                <div class="form-group">
                                    <label>{"Max Players"}</label>
                                    <select 
                                        onchange={
                                            let on_form_input = on_form_input.clone();
                                            Callback::from(move |e: Event| {
                                                let select: HtmlInputElement = e.target_unchecked_into();
                                                on_form_input.emit(("max_players".to_string(), select.value()))
                                            })
                                        }
                                    >
                                        <option value="2" selected={self.create_room_form.max_players == 2}>{"2 Players"}</option>
                                        <option value="6" selected={self.create_room_form.max_players == 6}>{"6 Players"}</option>
                                        <option value="9" selected={self.create_room_form.max_players == 9}>{"9 Players"}</option>
                                    </select>
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>{"Small Blind"}</label>
                                        <input 
                                            type="number" 
                                            value={self.create_room_form.small_blind.to_string()}
                                            onchange={
                                                let on_form_input = on_form_input.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_form_input.emit(("small_blind".to_string(), input.value()))
                                                })
                                            }
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label>{"Big Blind"}</label>
                                        <input 
                                            type="number" 
                                            value={self.create_room_form.big_blind.to_string()}
                                            onchange={
                                                let on_form_input = on_form_input.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_form_input.emit(("big_blind".to_string(), input.value()))
                                                })
                                            }
                                        />
                                    </div>
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>{"Min Buy-in"}</label>
                                        <input 
                                            type="number" 
                                            value={self.create_room_form.min_buy_in.to_string()}
                                            onchange={
                                                let on_form_input = on_form_input.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_form_input.emit(("min_buy_in".to_string(), input.value()))
                                                })
                                            }
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label>{"Max Buy-in"}</label>
                                        <input 
                                            type="number" 
                                            value={self.create_room_form.max_buy_in.to_string()}
                                            onchange={
                                                let on_form_input = on_form_input.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_form_input.emit(("max_buy_in".to_string(), input.value()))
                                                })
                                            }
                                        />
                                    </div>
                                </div>
                                <div class="form-group">
                                    <label class="checkbox-label">
                                        <input 
                                            type="checkbox" 
                                            checked={self.create_room_form.is_private}
                                            onchange={
                                                let on_form_input = on_form_input.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_form_input.emit(("is_private".to_string(), input.checked().to_string()))
                                                })
                                            }
                                        />
                                        <span>{"Private Room (requires password)"}</span>
                                    </label>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button class="cancel-btn" onclick={on_hide_create_modal}>{"Cancel"}</button>
                                <button class="create-btn primary" onclick={on_create_room}>{"Create Room"}</button>
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
