// Game state management
use crate::types::{PlayerInfo, RoomInfo, GamePhase, Card, GameAction, HandWinner, PotWinner, Uuid};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub current_room: Option<RoomInfo>,
    pub players: HashMap<Uuid, PlayerInfo>,
    pub game_phase: GamePhase,
    pub community_cards: Vec<Card>,
    pub pot: i64,
    pub side_pots: Vec<SidePot>,
    pub current_bet: i64,
    pub dealer_position: Option<u8>,
    pub active_player: Option<Uuid>,
    pub last_action: Option<GameAction>,
    pub hand_history: Vec<GameAction>,
    pub hand_number: u32,
    pub time_bank: Option<i32>, // seconds remaining for current player
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidePot {
    pub amount: i64,
    pub eligible_players: Vec<Uuid>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_room: None,
            players: HashMap::new(),
            game_phase: GamePhase::WaitingForPlayers,
            community_cards: Vec::new(),
            pot: 0,
            side_pots: Vec::new(),
            current_bet: 0,
            dealer_position: None,
            active_player: None,
            last_action: None,
            hand_history: Vec::new(),
            hand_number: 0,
            time_bank: None,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn join_room(&mut self, room: RoomInfo, players: Vec<PlayerInfo>) {
        self.current_room = Some(room);
        self.players.clear();
        for player in players {
            self.players.insert(player.id, player);
        }
    }
    
    pub fn leave_room(&mut self) {
        *self = Self::default();
    }
    
    pub fn add_player(&mut self, player: PlayerInfo) {
        self.players.insert(player.id, player);
    }
    
    pub fn remove_player(&mut self, player_id: Uuid) {
        self.players.remove(&player_id);
    }
    
    pub fn update_game_phase(&mut self, phase: GamePhase, community_cards: Vec<Card>, pot: i64) {
        self.game_phase = phase;
        self.community_cards = community_cards;
        self.pot = pot;
    }
    
    pub fn set_active_player(&mut self, player_id: Option<Uuid>, time_limit: Option<i32>) {
        self.active_player = player_id;
        self.time_bank = time_limit;
    }
    
    pub fn add_action(&mut self, action: GameAction) {
        self.hand_history.push(action.clone());
        self.last_action = Some(action);
    }
    
    pub fn complete_hand(&mut self, winners: Vec<HandWinner>, pot_distribution: Vec<PotWinner>) {
        // Update player chip counts based on winnings
        for pot_winner in pot_distribution {
            if let Some(player) = self.players.get_mut(&pot_winner.player_id) {
                player.chips += pot_winner.amount;
            }
        }
        
        // Reset hand state
        self.game_phase = GamePhase::WaitingForPlayers;
        self.community_cards.clear();
        self.pot = 0;
        self.side_pots.clear();
        self.current_bet = 0;
        self.active_player = None;
        self.last_action = None;
        self.hand_history.clear();
        self.hand_number += 1;
        self.time_bank = None;
        
        // Reset player hand states
        for player in self.players.values_mut() {
            player.hand.clear();
            player.current_bet = 0;
            player.is_all_in = false;
            player.is_folded = false;
            player.action_time_left = None;
        }
    }
    
    pub fn get_seated_players(&self) -> Vec<&PlayerInfo> {
        let mut seated: Vec<_> = self.players.values().collect();
        seated.sort_by_key(|p| p.seat_position);
        seated
    }
    
    pub fn get_active_players(&self) -> Vec<&PlayerInfo> {
        self.players.values().filter(|p| p.is_active && !p.is_folded).collect()
    }
    
    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
    
    pub fn is_in_hand(&self) -> bool {
        matches!(
            self.game_phase,
            GamePhase::PreFlop | GamePhase::Flop | GamePhase::Turn | GamePhase::River | GamePhase::Showdown
        )
    }
    
    pub fn can_act(&self, player_id: Uuid) -> bool {
        self.active_player == Some(player_id) && self.is_in_hand()
    }
}
