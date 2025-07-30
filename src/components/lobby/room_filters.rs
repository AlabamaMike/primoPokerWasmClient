use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use gloo_storage::{LocalStorage, Storage};

use crate::types::{RoomFilter, GameType};

#[derive(Properties, PartialEq)]
pub struct RoomFiltersProps {
    pub current_filter: RoomFilter,
    pub on_filter_change: Callback<RoomFilter>,
    pub on_reset_filters: Callback<()>,
}

pub struct RoomFilters {
    filter: RoomFilter,
    stakes_min_input: String,
    stakes_max_input: String,
}

pub enum RoomFiltersMsg {
    UpdateGameType(GameType),
    UpdateStakesMin(String),
    UpdateStakesMax(String),
    UpdateMaxPlayers(Option<u8>),
    ToggleEmptyRooms,
    ToggleFullRooms,
    ToggleGameType(GameType),
    ResetFilters,
    SaveFilters,
}

impl Component for RoomFilters {
    type Message = RoomFiltersMsg;
    type Properties = RoomFiltersProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let filter = props.current_filter.clone();
        
        // Load saved filters from localStorage if available
        let saved_filter = LocalStorage::get::<RoomFilter>("primo_poker_room_filters")
            .unwrap_or_else(|_| filter.clone());

        Self {
            filter: saved_filter.clone(),
            stakes_min_input: saved_filter.min_stakes.map(|v| v.to_string()).unwrap_or_default(),
            stakes_max_input: saved_filter.max_stakes.map(|v| v.to_string()).unwrap_or_default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RoomFiltersMsg::UpdateGameType(game_type) => {
                // Replace current game types with the selected one
                self.filter.game_types = vec![game_type];
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::ToggleGameType(game_type) => {
                // Toggle game type in the list (for multi-select support)
                if self.filter.game_types.contains(&game_type) {
                    self.filter.game_types.retain(|gt| *gt != game_type);
                } else {
                    self.filter.game_types.push(game_type);
                }
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::UpdateStakesMin(value) => {
                self.stakes_min_input = value.clone();
                self.filter.min_stakes = if value.is_empty() {
                    None
                } else {
                    value.parse::<i64>().ok()
                };
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::UpdateStakesMax(value) => {
                self.stakes_max_input = value.clone();
                self.filter.max_stakes = if value.is_empty() {
                    None
                } else {
                    value.parse::<i64>().ok()
                };
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::UpdateMaxPlayers(max_players) => {
                self.filter.max_players_filter = max_players;
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::ToggleEmptyRooms => {
                self.filter.show_empty_rooms = !self.filter.show_empty_rooms;
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::ToggleFullRooms => {
                self.filter.show_full_rooms = !self.filter.show_full_rooms;
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::ResetFilters => {
                self.filter = RoomFilter {
                    min_stakes: None,
                    max_stakes: None,
                    max_players_filter: None,
                    game_types: vec![GameType::TexasHoldem],
                    show_full_rooms: true,
                    show_empty_rooms: true,
                };
                self.stakes_min_input.clear();
                self.stakes_max_input.clear();
                
                // Clear saved filters
                let _ = LocalStorage::delete("primo_poker_room_filters");
                
                ctx.props().on_reset_filters.emit(());
                self.emit_filter_change(ctx);
                true
            }
            RoomFiltersMsg::SaveFilters => {
                // Save current filters to localStorage
                let _ = LocalStorage::set("primo_poker_room_filters", &self.filter);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // Callbacks
        let on_game_type_change = link.callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let game_type = match select.value().as_str() {
                "texas_holdem" => GameType::TexasHoldem,
                "omaha" => GameType::Omaha,
                "omaha_hi_lo" => GameType::OmahaHiLo,
                "seven_card_stud" => GameType::SevenCardStud,
                _ => GameType::TexasHoldem,
            };
            RoomFiltersMsg::UpdateGameType(game_type)
        });

        let on_stakes_min_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RoomFiltersMsg::UpdateStakesMin(input.value())
        });

        let on_stakes_max_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RoomFiltersMsg::UpdateStakesMax(input.value())
        });

        let on_max_players_change = link.callback(|e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let max_players = match select.value().as_str() {
                "" => None,
                value => value.parse::<u8>().ok(),
            };
            RoomFiltersMsg::UpdateMaxPlayers(max_players)
        });

        let on_toggle_empty = link.callback(|_| RoomFiltersMsg::ToggleEmptyRooms);
        let on_toggle_full = link.callback(|_| RoomFiltersMsg::ToggleFullRooms);
        let on_reset = link.callback(|_| RoomFiltersMsg::ResetFilters);

        // Stake range presets
        let preset_stakes = vec![
            ("Micro", 1, 5),
            ("Low", 5, 25),
            ("Mid", 25, 100),
            ("High", 100, 500),
            ("Nosebleed", 500, 2000),
        ];

        html! {
            <div class="room-filters">
                <div class="filters-header">
                    <h3>{"🎯 Filter Games"}</h3>
                    <button class="reset-filters-btn" onclick={on_reset} title="Reset all filters">
                        {"↺ Reset"}
                    </button>
                </div>

                <div class="filter-sections">
                    // Game Type Filter
                    <div class="filter-section">
                        <label class="filter-label">{"🎮 Game Type"}</label>
                        <div class="game-type-options">
                            <select onchange={on_game_type_change} class="game-type-select">
                                <option value="texas_holdem" selected={matches!(self.filter.game_types.first(), Some(GameType::TexasHoldem))}>
                                    {"Texas Hold'em"}
                                </option>
                                <option value="omaha" selected={matches!(self.filter.game_types.first(), Some(GameType::Omaha))}>
                                    {"Omaha"}
                                </option>
                                <option value="omaha_hi_lo" selected={matches!(self.filter.game_types.first(), Some(GameType::OmahaHiLo))}>
                                    {"Omaha Hi-Lo"}
                                </option>
                                <option value="seven_card_stud" selected={matches!(self.filter.game_types.first(), Some(GameType::SevenCardStud))}>
                                    {"Seven Card Stud"}
                                </option>
                            </select>
                        </div>
                    </div>

                    // Stakes Range Filter
                    <div class="filter-section">
                        <label class="filter-label">{"💰 Stakes Range"}</label>
                        
                        // Preset buttons
                        <div class="stakes-presets">
                            {for preset_stakes.iter().map(|(name, min, max)| {
                                let min_val = *min;
                                let max_val = *max;
                                let onclick = link.callback(move |_| {
                                    // This will trigger two separate updates
                                    // In a real implementation, you'd want to batch these
                                    RoomFiltersMsg::UpdateStakesMin(min_val.to_string())
                                });
                                html! {
                                    <button 
                                        class="preset-btn"
                                        onclick={onclick}
                                        title={format!("${} - ${}", min_val, max_val)}
                                    >
                                        {*name}
                                    </button>
                                }
                            })}
                        </div>

                        // Manual input
                        <div class="stakes-inputs">
                            <div class="input-group">
                                <label class="input-label">{"Min Big Blind"}</label>
                                <input 
                                    type="number" 
                                    class="stakes-input"
                                    placeholder="Min" 
                                    value={self.stakes_min_input.clone()}
                                    onchange={on_stakes_min_change}
                                    min="0"
                                />
                            </div>
                            <span class="range-separator">{" - "}</span>
                            <div class="input-group">
                                <label class="input-label">{"Max Big Blind"}</label>
                                <input 
                                    type="number" 
                                    class="stakes-input"
                                    placeholder="Max"
                                    value={self.stakes_max_input.clone()}
                                    onchange={on_stakes_max_change}
                                    min="0"
                                />
                            </div>
                        </div>
                    </div>

                    // Player Count Filter
                    <div class="filter-section">
                        <label class="filter-label">{"👥 Max Players"}</label>
                        <select onchange={on_max_players_change} class="players-select">
                            <option value="" selected={self.filter.max_players_filter.is_none()}>
                                {"Any"}
                            </option>
                            <option value="2" selected={self.filter.max_players_filter == Some(2)}>
                                {"2 Players (Heads-up)"}
                            </option>
                            <option value="6" selected={self.filter.max_players_filter == Some(6)}>
                                {"6 Players (6-max)"}
                            </option>
                            <option value="9" selected={self.filter.max_players_filter == Some(9)}>
                                {"9 Players (Full ring)"}
                            </option>
                        </select>
                    </div>

                    // Room Status Toggles
                    <div class="filter-section">
                        <label class="filter-label">{"🏠 Room Status"}</label>
                        <div class="toggle-options">
                            <label class="toggle-option">
                                <input 
                                    type="checkbox" 
                                    checked={self.filter.show_empty_rooms}
                                    onchange={on_toggle_empty}
                                />
                                <span class="toggle-label">{"Show empty rooms"}</span>
                            </label>
                            <label class="toggle-option">
                                <input 
                                    type="checkbox" 
                                    checked={self.filter.show_full_rooms}
                                    onchange={on_toggle_full}
                                />
                                <span class="toggle-label">{"Show full rooms"}</span>
                            </label>
                        </div>
                    </div>
                </div>

                // Active Filters Display
                <div class="active-filters">
                    <div class="active-filters-header">{"Active Filters:"}</div>
                    <div class="filter-tags">
                        {self.render_active_filter_tags(ctx)}
                    </div>
                </div>
            </div>
        }
    }
}

impl RoomFilters {
    fn emit_filter_change(&self, ctx: &Context<Self>) {
        // Save filters automatically when they change
        let _ = LocalStorage::set("primo_poker_room_filters", &self.filter);
        
        // Emit the change to parent component
        ctx.props().on_filter_change.emit(self.filter.clone());
    }

    fn render_active_filter_tags(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut tags = Vec::new();

        // Game type tags
        for game_type in &self.filter.game_types {
            let game_name = match game_type {
                GameType::TexasHoldem => "Texas Hold'em",
                GameType::Omaha => "Omaha",
                GameType::OmahaHiLo => "Omaha Hi-Lo",
                GameType::SevenCardStud => "Seven Card Stud",
            };
            tags.push(html! {
                <span class="filter-tag game-type-tag">
                    {game_name}
                </span>
            });
        }

        // Stakes range tag
        match (self.filter.min_stakes, self.filter.max_stakes) {
            (Some(min), Some(max)) => {
                tags.push(html! {
                    <span class="filter-tag stakes-tag">
                        {format!("Stakes: ${} - ${}", min, max)}
                    </span>
                });
            }
            (Some(min), None) => {
                tags.push(html! {
                    <span class="filter-tag stakes-tag">
                        {format!("Min Stakes: ${}", min)}
                    </span>
                });
            }
            (None, Some(max)) => {
                tags.push(html! {
                    <span class="filter-tag stakes-tag">
                        {format!("Max Stakes: ${}", max)}
                    </span>
                });
            }
            (None, None) => {}
        }

        // Players tag
        if let Some(max_players) = self.filter.max_players_filter {
            let player_desc = match max_players {
                2 => "Heads-up",
                6 => "6-max",
                9 => "Full ring",
                _ => "Custom",
            };
            tags.push(html! {
                <span class="filter-tag players-tag">
                    {format!("{} players", player_desc)}
                </span>
            });
        }

        // Room status tags
        if !self.filter.show_empty_rooms {
            tags.push(html! {
                <span class="filter-tag status-tag">
                    {"Hide empty"}
                </span>
            });
        }
        if !self.filter.show_full_rooms {
            tags.push(html! {
                <span class="filter-tag status-tag">
                    {"Hide full"}
                </span>
            });
        }

        if tags.is_empty() {
            html! { <span class="no-filters">{"No active filters"}</span> }
        } else {
            html! { {for tags} }
        }
    }
}
