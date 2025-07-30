use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::types::{User, PlayerStats, PlayerStatus};

#[derive(Properties, PartialEq)]
pub struct PlayerProfileModalProps {
    pub player: User,
    pub stats: PlayerStats,
    pub is_friend: bool,
    pub show: bool,
    pub on_close: Callback<()>,
    pub on_add_friend: Callback<String>,
    pub on_remove_friend: Callback<String>,
    pub on_send_message: Callback<String>,
    pub on_invite_to_game: Callback<String>,
}

pub enum PlayerProfileModalMsg {
    Close,
    AddFriend,
    RemoveFriend,
    SendMessage,
    InviteToGame,
    CopyPlayerId,
}

pub struct PlayerProfileModal {
    _marker: std::marker::PhantomData<()>,
}

impl Component for PlayerProfileModal {
    type Message = PlayerProfileModalMsg;
    type Properties = PlayerProfileModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        
        match msg {
            PlayerProfileModalMsg::Close => {
                props.on_close.emit(());
                false
            }
            PlayerProfileModalMsg::AddFriend => {
                props.on_add_friend.emit(props.player.id.to_string());
                false
            }
            PlayerProfileModalMsg::RemoveFriend => {
                props.on_remove_friend.emit(props.player.id.to_string());
                false
            }
            PlayerProfileModalMsg::SendMessage => {
                props.on_send_message.emit(props.player.id.to_string());
                false
            }
            PlayerProfileModalMsg::InviteToGame => {
                props.on_invite_to_game.emit(props.player.id.to_string());
                false
            }
            PlayerProfileModalMsg::CopyPlayerId => {
                // Simple console log for now - clipboard API is complex in WASM
                web_sys::console::log_1(&format!("Player ID: {}", props.player.id).into());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        if !props.show {
            return html! {};
        }

        let link = ctx.link();
        let on_backdrop_click = link.callback(|_| PlayerProfileModalMsg::Close);
        let on_modal_click = |e: MouseEvent| e.stop_propagation();

        // Calculate additional stats
        let total_games = props.stats.games_won + props.stats.games_lost;
        let win_rate = props.stats.win_rate; // Use the precalculated win rate

        let status_text = match props.player.status.as_ref().unwrap_or(&PlayerStatus::Away) {
            PlayerStatus::Online => "🟢 Online",
            PlayerStatus::Playing => "🟡 In Game", 
            PlayerStatus::Away => "⚫ Away",
        };

        html! {
            <div class="modal-overlay" onclick={on_backdrop_click}>
                <div class="player-profile-modal" onclick={on_modal_click}>
                    // Header
                    <div class="modal-header">
                        <div class="player-header-info">
                            <div class="player-avatar-large">
                                {if let Some(ref avatar_url) = props.player.avatar_url {
                                    html! { <img src={avatar_url.clone()} alt="Avatar" /> }
                                } else {
                                    html! { <div class="default-avatar-large">{props.player.display_name.chars().next().unwrap_or('?')}</div> }
                                }}
                            </div>
                            <div class="player-main-info">
                                <h2 class="player-name">{&props.player.display_name}</h2>
                                <p class="player-username">{"@"}{&props.player.username}</p>
                                <p class="player-status">{status_text}</p>
                            </div>
                        </div>
                        <button 
                            class="close-btn"
                            onclick={link.callback(|_| PlayerProfileModalMsg::Close)}
                        >
                            {"×"}
                        </button>
                    </div>

                    // Player Stats Section
                    <div class="modal-section">
                        <h3>{"Statistics"}</h3>
                        <div class="stats-grid">
                            <div class="stat-card">
                                <div class="stat-value">{total_games}</div>
                                <div class="stat-label">{"Games Played"}</div>
                            </div>
                            <div class="stat-card">
                                <div class="stat-value">{format!("{:.1}%", win_rate)}</div>
                                <div class="stat-label">{"Win Rate"}</div>
                            </div>
                            <div class="stat-card">
                                <div class="stat-value">{props.stats.games_won}</div>
                                <div class="stat-label">{"Games Won"}</div>
                            </div>
                            <div class="stat-card">
                                <div class="stat-value">{props.stats.total_winnings}</div>
                                <div class="stat-label">{"Total Winnings"}</div>
                            </div>
                            <div class="stat-card">
                                <div class="stat-value">{format!("{:.1}", props.stats.average_pot_size)}</div>
                                <div class="stat-label">{"Avg Pot Size"}</div>
                            </div>
                            <div class="stat-card">
                                <div class="stat-value">{format!("{:.1}%", props.stats.fold_percentage)}</div>
                                <div class="stat-label">{"Fold %"}</div>
                            </div>
                        </div>
                    </div>

                    // Player Info Section
                    <div class="modal-section">
                        <h3>{"Player Information"}</h3>
                        <div class="info-grid">
                            <div class="info-item">
                                <span class="info-label">{"Player ID:"}</span>
                                <span class="info-value">
                                    {props.player.id.to_string()}
                                    <button 
                                        class="copy-btn"
                                        onclick={link.callback(|_| PlayerProfileModalMsg::CopyPlayerId)}
                                        title="Copy Player ID"
                                    >
                                        {"📋"}
                                    </button>
                                </span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">{"Balance:"}</span>
                                <span class="info-value">{format!("${}", props.player.balance)}</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">{"Member Since:"}</span>
                                <span class="info-value">{props.player.created_at.format("%B %Y").to_string()}</span>
                            </div>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="modal-actions">
                        {if props.is_friend {
                            html! {
                                <>
                                    <button 
                                        class="action-btn secondary"
                                        onclick={link.callback(|_| PlayerProfileModalMsg::RemoveFriend)}
                                    >
                                        {"➖ Remove Friend"}
                                    </button>
                                    <button 
                                        class="action-btn primary"
                                        onclick={link.callback(|_| PlayerProfileModalMsg::SendMessage)}
                                    >
                                        {"💬 Send Message"}
                                    </button>
                                </>
                            }
                        } else {
                            html! {
                                <button 
                                    class="action-btn primary"
                                    onclick={link.callback(|_| PlayerProfileModalMsg::AddFriend)}
                                >
                                    {"➕ Add Friend"}
                                </button>
                            }
                        }}
                        <button 
                            class="action-btn accent"
                            onclick={link.callback(|_| PlayerProfileModalMsg::InviteToGame)}
                        >
                            {"🎮 Invite to Game"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
