use yew::prelude::*;
use yew_router::prelude::*;

use crate::types::{GameRoom, GameType, AppRoute};

#[derive(Properties, PartialEq)]
pub struct RoomCardProps {
    pub room: GameRoom,
    pub on_join: Callback<String>,
}

#[function_component(RoomCard)]
pub fn room_card(props: &RoomCardProps) -> Html {
    let room = &props.room;
    let navigator = use_navigator().unwrap();
    
    let on_join_click = {
        let room_id = room.id.clone();
        let on_join = props.on_join.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |_: MouseEvent| {
            on_join.emit(room_id.clone());
            navigator.push(&AppRoute::Game { room_id: room_id.clone() });
        })
    };

    // Calculate occupancy percentage
    let occupancy_percent = if room.max_players > 0 {
        (room.current_players as f32 / room.max_players as f32 * 100.0) as u32
    } else {
        0
    };

    // Determine room status
    let (status_class, status_text) = if room.current_players == 0 {
        ("empty", "Empty")
    } else if room.current_players == room.max_players {
        ("full", "Full")
    } else {
        ("active", "Active")
    };

    // Game type icon and name
    let (game_icon, game_name) = match room.game_type {
        GameType::TexasHoldem => ("🃏", "Texas Hold'em"),
        GameType::Omaha => ("🎯", "Omaha"),
        GameType::OmahaHiLo => ("🎯", "Omaha Hi-Lo"),
        GameType::SevenCardStud => ("🎲", "Seven Card Stud"),
    };

    // Stakes format
    let stakes_display = format!("${}/{}", room.small_blind, room.big_blind);
    let buy_in_range = format!("${} - ${}", room.min_buy_in, room.max_buy_in);

    // Disable join button if room is full
    let can_join = room.current_players < room.max_players && room.is_active;

    html! {
        <div class={classes!("room-card", status_class)}>
            // Room Header
            <div class="room-header">
                <div class="room-title">
                    <h4 class="room-name">{&room.name}</h4>
                    {if room.is_private {
                        html! { <span class="private-icon" title="Private Room">{"🔒"}</span> }
                    } else {
                        html! {}
                    }}
                </div>
                <div class={classes!("room-status", status_class)}>
                    <span class="status-indicator"></span>
                    <span class="status-text">{status_text}</span>
                </div>
            </div>

            // Game Info
            <div class="room-info">
                <div class="game-type">
                    <span class="game-icon">{game_icon}</span>
                    <span class="game-name">{game_name}</span>
                </div>
                
                <div class="stakes-info">
                    <div class="stakes">
                        <label>{"Stakes:"}</label>
                        <span class="stakes-value">{stakes_display}</span>
                    </div>
                    <div class="buy-in">
                        <label>{"Buy-in:"}</label>
                        <span class="buy-in-value">{buy_in_range}</span>
                    </div>
                </div>
            </div>

            // Player Info
            <div class="player-info">
                <div class="player-count">
                    <div class="count-display">
                        <span class="current-players">{room.current_players}</span>
                        <span class="separator">{"/"}</span>
                        <span class="max-players">{room.max_players}</span>
                        <span class="players-label">{" players"}</span>
                    </div>
                    <div class="occupancy-bar">
                        <div 
                            class="occupancy-fill"
                            style={format!("width: {}%", occupancy_percent)}
                        ></div>
                    </div>
                </div>
            </div>

            // Actions
            <div class="room-actions">
                if can_join {
                    <button 
                        class="join-btn primary"
                        onclick={on_join_click}
                    >
                        {"🚀 Join Game"}
                    </button>
                } else if room.current_players == room.max_players {
                    <button class="join-btn disabled" disabled={true}>
                        {"Room Full"}
                    </button>
                } else if !room.is_active {
                    <button class="join-btn disabled" disabled={true}>
                        {"Inactive"}
                    </button>
                } else {
                    <button class="join-btn secondary">
                        {"👁 Spectate"}
                    </button>
                }
                
                // Additional actions dropdown
                <div class="room-menu">
                    <button class="menu-toggle">{"⋯"}</button>
                    <div class="menu-dropdown">
                        <button class="menu-item">{"⭐ Add to Favorites"}</button>
                        <button class="menu-item">{"📊 Room Stats"}</button>
                        {if !room.is_private {
                            html! {
                                <button class="menu-item">{"🔗 Share Room"}</button>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>
            </div>

            // Room Metadata (for advanced users)
            <div class="room-metadata">
                <div class="metadata-item">
                    <span class="metadata-label">{"ID:"}</span>
                    <span class="metadata-value room-id">{&room.id}</span>
                </div>
                <div class="metadata-item">
                    <span class="metadata-label">{"Created:"}</span>
                    <span class="metadata-value">
                        {room.created_at.format("%H:%M").to_string()}
                    </span>
                </div>
            </div>
        </div>
    }
}
