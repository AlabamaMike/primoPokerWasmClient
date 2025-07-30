// Core types and data structures for PrimoPoker client
use serde::{Deserialize, Serialize};
use yew_router::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Router configuration
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/lobby")]
    Lobby,
    #[at("/game/:room_id")]
    Game { room_id: String },
    #[at("/profile")]
    Profile,
    #[at("/demo")]
    Demo,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// User-related types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub chips: i64,
    pub balance: i64, // Additional balance field for modal
    pub level: i32,
    pub experience: i64,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub status: Option<PlayerStatus>, // Player status for social features
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
}

// Game-related types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Card {
    Unknown,
    Card { suit: Suit, rank: Rank },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerInfo {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub chips: i64,
    pub seat_position: u8,
    pub is_active: bool,
    pub is_dealer: bool,
    pub is_small_blind: bool,
    pub is_big_blind: bool,
    pub hand: Vec<Card>,
    pub current_bet: i64,
    pub is_all_in: bool,
    pub is_folded: bool,
    pub action_time_left: Option<i32>, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameType {
    TexasHoldem,
    Omaha,
    OmahaHiLo,
    SevenCardStud,
}

impl Default for GameType {
    fn default() -> Self {
        GameType::TexasHoldem
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    WaitingForPlayers,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
    HandComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Bet(i64),
    Raise(i64),
    AllIn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameAction {
    pub player_id: Uuid,
    pub action: PlayerAction,
    pub amount: i64,
    pub timestamp: DateTime<Utc>,
}

// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    // Authentication
    Authenticate { token: String },
    
    // Lobby actions
    JoinRoom { room_id: String },
    LeaveRoom,
    CreateRoom { room_config: RoomConfig },
    GetRoomList,
    
    // Game actions
    PlayerAction { action: PlayerAction },
    RequestTimeExtension,
    
    // Chat
    SendMessage { message: String, room_id: Option<String> },
    
    // Connection
    Ping,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    // Authentication
    AuthenticationSuccess { user: User },
    AuthenticationFailed { error: String },
    
    // Room updates
    RoomJoined { room: RoomInfo, players: Vec<PlayerInfo> },
    RoomLeft,
    RoomListUpdate { rooms: Vec<RoomInfo> },
    PlayerJoined { player: PlayerInfo },
    PlayerLeft { player_id: Uuid },
    
    // Game updates
    GameStateUpdate { phase: GamePhase, community_cards: Vec<Card>, pot: i64 },
    PlayerActionRequired { time_limit: i32 },
    PlayerActionUpdate { action: GameAction },
    HandResult { winners: Vec<HandWinner>, pot_distribution: Vec<PotWinner> },
    
    // Chat
    ChatMessage { player_id: Uuid, username: String, message: String, timestamp: DateTime<Utc> },
    
    // Connection
    Pong,
    Error { message: String },
    Disconnect { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub name: String,
    pub game_type: GameType,
    pub max_players: u8,
    pub small_blind: i64,
    pub big_blind: i64,
    pub min_buy_in: i64,
    pub max_buy_in: i64,
    pub is_private: bool,
    pub password: Option<String>,
}

// Lobby and room types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameRoom {
    pub id: String,
    pub name: String,
    pub game_type: GameType,
    pub current_players: u8,
    pub max_players: u8,
    pub small_blind: i64,
    pub big_blind: i64,
    pub min_buy_in: i64,
    pub max_buy_in: i64,
    pub is_private: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

pub type RoomInfo = GameRoom; // Alias for compatibility

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomFilter {
    pub min_stakes: Option<i64>,
    pub max_stakes: Option<i64>,
    pub max_players_filter: Option<u8>,
    pub game_types: Vec<GameType>,
    pub show_full_rooms: bool,
    pub show_empty_rooms: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandWinner {
    pub player_id: Uuid,
    pub hand_type: HandType,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PotWinner {
    pub player_id: Uuid,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

// UI-specific types
#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub target: String,
    pub duration: f32,
    pub ease_function: EaseFunction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EaseFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationMessage {
    pub id: Uuid,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum PokerError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Game error: {0}")]
    GameError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<PokerError> for String {
    fn from(error: PokerError) -> Self {
        error.to_string()
    }
}

// Default implementations
impl Default for Card {
    fn default() -> Self {
        Card::Unknown
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Unknown => write!(f, "🂠"),
            Card::Card { suit, rank } => write!(f, "{}{}", rank, suit),
        }
    }
}

// Social and Player Status Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerStatus {
    Online,
    Playing,
    Away,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerStats {
    pub games_played: u32,
    pub games_won: u32,
    pub games_lost: u32,
    pub total_winnings: i64,
    pub biggest_win: i64,
    pub win_rate: f64,
    pub avg_session_length: u32, // minutes
    pub average_pot_size: f64,
    pub bluff_frequency: f64,
    pub fold_percentage: f64,
    pub all_in_frequency: f64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            games_played: 0,
            games_won: 0,
            games_lost: 0,
            total_winnings: 0,
            biggest_win: 0,
            win_rate: 0.0,
            avg_session_length: 0,
            average_pot_size: 0.0,
            bluff_frequency: 0.0,
            fold_percentage: 0.0,
            all_in_frequency: 0.0,
        }
    }
}
