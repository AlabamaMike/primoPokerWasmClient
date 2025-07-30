use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use gloo_storage::{LocalStorage, Storage};

use crate::types::{User, PlayerStatus, PlayerStats};

#[derive(Properties, PartialEq)]
pub struct PlayerSidebarProps {
    pub current_user: Option<User>,
    pub is_collapsed: bool,
    pub on_toggle_collapse: Callback<()>,
    pub on_player_select: Callback<String>, // player_id
}

pub struct PlayerSidebar {
    search_query: String,
    online_friends: Vec<OnlinePlayer>,
    recent_players: Vec<RecentPlayer>,
    selected_tab: SidebarTab,
    is_loading: bool,
}

#[derive(Clone, PartialEq)]
pub enum SidebarTab {
    Friends,
    Recent,
    Search,
}

#[derive(Clone, PartialEq)]
pub struct OnlinePlayer {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub status: PlayerStatus,
    pub current_room: Option<String>,
    pub stats: PlayerStats,
    pub is_friend: bool,
}

#[derive(Clone, PartialEq)]
pub struct RecentPlayer {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub games_played_together: u32,
    pub stats: PlayerStats,
}

pub enum PlayerSidebarMsg {
    UpdateSearchQuery(String),
    SelectTab(SidebarTab),
    LoadOnlineFriends,
    LoadRecentPlayers,
    FriendsLoaded(Vec<OnlinePlayer>),
    RecentPlayersLoaded(Vec<RecentPlayer>),
    AddFriend(String),
    RemoveFriend(String),
    ViewPlayerProfile(String),
    JoinPlayerRoom(String),
    Error(String),
}

impl Component for PlayerSidebar {
    type Message = PlayerSidebarMsg;
    type Properties = PlayerSidebarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        
        // Load initial data
        link.send_message(PlayerSidebarMsg::LoadOnlineFriends);
        link.send_message(PlayerSidebarMsg::LoadRecentPlayers);

        Self {
            search_query: String::new(),
            online_friends: Vec::new(),
            recent_players: Vec::new(),
            selected_tab: SidebarTab::Friends,
            is_loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlayerSidebarMsg::UpdateSearchQuery(query) => {
                self.search_query = query;
                // TODO: Trigger search API call
                true
            }
            PlayerSidebarMsg::SelectTab(tab) => {
                self.selected_tab = tab;
                true
            }
            PlayerSidebarMsg::LoadOnlineFriends => {
                // Mock data for now - in real implementation, this would be an API call
                let mock_friends = vec![
                    OnlinePlayer {
                        id: "friend1".to_string(),
                        username: "pokerpro22".to_string(),
                        display_name: "PokerPro22".to_string(),
                        avatar_url: None,
                        status: PlayerStatus::Playing,
                        current_room: Some("room1".to_string()),
                        stats: PlayerStats {
                            games_played: 1250,
                            games_won: 342,
                            games_lost: 908,
                            total_winnings: 15420,
                            biggest_win: 2850,
                            win_rate: 27.4,
                            avg_session_length: 145,
                            average_pot_size: 380.0,
                            bluff_frequency: 18.5,
                            fold_percentage: 42.1,
                            all_in_frequency: 7.2,
                        },
                        is_friend: true,
                    },
                    OnlinePlayer {
                        id: "friend2".to_string(),
                        username: "bluffmaster".to_string(),
                        display_name: "BluffMaster".to_string(),
                        avatar_url: None,
                        status: PlayerStatus::Online,
                        current_room: None,
                        stats: PlayerStats {
                            games_played: 892,
                            games_won: 203,
                            games_lost: 689,
                            total_winnings: 8765,
                            biggest_win: 1950,
                            win_rate: 22.8,
                            avg_session_length: 98,
                            average_pot_size: 320.0,
                            bluff_frequency: 28.3,
                            fold_percentage: 38.7,
                            all_in_frequency: 12.4,
                        },
                        is_friend: true,
                    },
                ];
                
                ctx.link().send_message(PlayerSidebarMsg::FriendsLoaded(mock_friends));
                false
            }
            PlayerSidebarMsg::LoadRecentPlayers => {
                // Mock data for now
                let mock_recent = vec![
                    RecentPlayer {
                        id: "recent1".to_string(),
                        username: "cardshark99".to_string(),
                        display_name: "CardShark99".to_string(),
                        avatar_url: None,
                        last_seen: chrono::Utc::now() - chrono::Duration::minutes(15),
                        games_played_together: 12,
                        stats: PlayerStats {
                            games_played: 445,
                            games_won: 89,
                            games_lost: 356,
                            total_winnings: 3420,
                            biggest_win: 890,
                            win_rate: 20.0,
                            avg_session_length: 67,
                            average_pot_size: 215.0,
                            bluff_frequency: 15.2,
                            fold_percentage: 52.1,
                            all_in_frequency: 4.8,
                        },
                    },
                    RecentPlayer {
                        id: "recent2".to_string(),
                        username: "allin_annie".to_string(),
                        display_name: "AllIn Annie".to_string(),
                        avatar_url: None,
                        last_seen: chrono::Utc::now() - chrono::Duration::hours(2),
                        games_played_together: 3,
                        stats: PlayerStats {
                            games_played: 234,
                            games_won: 67,
                            games_lost: 167,
                            total_winnings: 1895,
                            biggest_win: 650,
                            win_rate: 28.6,
                            avg_session_length: 134,
                            average_pot_size: 285.0,
                            bluff_frequency: 31.7,
                            fold_percentage: 35.8,
                            all_in_frequency: 19.2,
                        },
                    },
                ];
                
                ctx.link().send_message(PlayerSidebarMsg::RecentPlayersLoaded(mock_recent));
                false
            }
            PlayerSidebarMsg::FriendsLoaded(friends) => {
                self.online_friends = friends;
                self.is_loading = false;
                true
            }
            PlayerSidebarMsg::RecentPlayersLoaded(recent) => {
                self.recent_players = recent;
                self.is_loading = false;
                true
            }
            PlayerSidebarMsg::AddFriend(player_id) => {
                // TODO: API call to add friend
                web_sys::console::log_1(&format!("Adding friend: {}", player_id).into());
                false
            }
            PlayerSidebarMsg::RemoveFriend(player_id) => {
                // TODO: API call to remove friend
                web_sys::console::log_1(&format!("Removing friend: {}", player_id).into());
                false
            }
            PlayerSidebarMsg::ViewPlayerProfile(player_id) => {
                ctx.props().on_player_select.emit(player_id);
                false
            }
            PlayerSidebarMsg::JoinPlayerRoom(room_id) => {
                // TODO: Navigate to player's room
                web_sys::console::log_1(&format!("Joining room: {}", room_id).into());
                false
            }
            PlayerSidebarMsg::Error(error) => {
                web_sys::console::error_1(&error.into());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        if props.is_collapsed {
            return self.render_collapsed_view(ctx);
        }

        let on_search_input = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            PlayerSidebarMsg::UpdateSearchQuery(input.value())
        });

        let on_toggle_collapse = props.on_toggle_collapse.clone();

        html! {
            <div class="player-sidebar">
                // Sidebar Header
                <div class="sidebar-header">
                    <h3>{"👥 Players"}</h3>
                    <button 
                        class="collapse-btn"
                        onclick={move |_| on_toggle_collapse.emit(())}
                        title="Collapse sidebar"
                    >
                        {"◀"}
                    </button>
                </div>

                // Search Bar
                <div class="search-section">
                    <div class="search-input-container">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search players..."
                            value={self.search_query.clone()}
                            oninput={on_search_input}
                        />
                        <span class="search-icon">{"🔍"}</span>
                    </div>
                </div>

                // Tab Navigation
                <div class="sidebar-tabs">
                    <button 
                        class={classes!(
                            "tab-button",
                            if matches!(self.selected_tab, SidebarTab::Friends) { Some("active") } else { None }
                        )}
                        onclick={link.callback(|_| PlayerSidebarMsg::SelectTab(SidebarTab::Friends))}
                    >
                        {"🤝 Friends"}{format!(" ({})", self.online_friends.len())}
                    </button>
                    <button 
                        class={classes!(
                            "tab-button",
                            if matches!(self.selected_tab, SidebarTab::Recent) { Some("active") } else { None }
                        )}
                        onclick={link.callback(|_| PlayerSidebarMsg::SelectTab(SidebarTab::Recent))}
                    >
                        {"🕒 Recent"}{format!(" ({})", self.recent_players.len())}
                    </button>
                </div>

                // Content Area
                <div class="sidebar-content">
                    {self.render_tab_content(ctx)}
                </div>
            </div>
        }
    }
}

impl PlayerSidebar {
    fn render_collapsed_view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let on_toggle_collapse = props.on_toggle_collapse.clone();

        html! {
            <div class="player-sidebar collapsed">
                <button 
                    class="expand-btn"
                    onclick={move |_| on_toggle_collapse.emit(())}
                    title="Show players"
                >
                    {"▶ 👥"}
                </button>
                <div class="collapsed-stats">
                    <div class="stat-item">
                        <span class="stat-value">{self.online_friends.len()}</span>
                        <span class="stat-label">{"Friends"}</span>
                    </div>
                </div>
            </div>
        }
    }

    fn render_tab_content(&self, ctx: &Context<Self>) -> Html {
        if self.is_loading {
            return html! {
                <div class="loading-state">
                    <div class="loading-spinner"></div>
                    <p>{"Loading players..."}</p>
                </div>
            };
        }

        match self.selected_tab {
            SidebarTab::Friends => self.render_friends_tab(ctx),
            SidebarTab::Recent => self.render_recent_tab(ctx),
            SidebarTab::Search => self.render_search_results(ctx),
        }
    }

    fn render_friends_tab(&self, ctx: &Context<Self>) -> Html {
        if self.online_friends.is_empty() {
            return html! {
                <div class="empty-state">
                    <div class="empty-icon">{"😔"}</div>
                    <h4>{"No friends online"}</h4>
                    <p>{"Invite friends to play poker together!"}</p>
                </div>
            };
        }

        html! {
            <div class="player-list">
                {for self.online_friends.iter().map(|friend| {
                    self.render_online_player(ctx, friend)
                })}
            </div>
        }
    }

    fn render_recent_tab(&self, ctx: &Context<Self>) -> Html {
        if self.recent_players.is_empty() {
            return html! {
                <div class="empty-state">
                    <div class="empty-icon">{"🎲"}</div>
                    <h4>{"No recent players"}</h4>
                    <p>{"Players you've played with recently will appear here."}</p>
                </div>
            };
        }

        html! {
            <div class="player-list">
                {for self.recent_players.iter().map(|player| {
                    self.render_recent_player(ctx, player)
                })}
            </div>
        }
    }

    fn render_search_results(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="search-results">
                <p>{"Search functionality coming soon..."}</p>
            </div>
        }
    }

    fn render_online_player(&self, ctx: &Context<Self>, player: &OnlinePlayer) -> Html {
        let link = ctx.link();
        let player_id = player.id.clone();
        let room_id = player.current_room.clone();

        let status_class = match player.status {
            PlayerStatus::Online => "online",
            PlayerStatus::Playing => "playing",
            PlayerStatus::Away => "away",
        };

        let status_text = match player.status {
            PlayerStatus::Online => "Online",
            PlayerStatus::Playing => "Playing",
            PlayerStatus::Away => "Away",
        };

        html! {
            <div class="player-card">
                <div class="player-info">
                    <div class="player-avatar">
                        {if let Some(ref avatar_url) = player.avatar_url {
                            html! { <img src={avatar_url.clone()} alt="Avatar" /> }
                        } else {
                            html! { <div class="default-avatar">{player.display_name.chars().next().unwrap_or('?')}</div> }
                        }}
                        <div class={classes!("status-indicator", status_class)}></div>
                    </div>
                    <div class="player-details">
                        <div class="player-name">{&player.display_name}</div>
                        <div class="player-status">{status_text}</div>
                        {if let Some(ref room) = player.current_room {
                            html! { <div class="current-room">{"In room: "}{room}</div> }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>
                
                <div class="player-stats-mini">
                    <div class="stat">
                        <span class="stat-value">{format!("{:.1}%", player.stats.win_rate)}</span>
                        <span class="stat-label">{"Win Rate"}</span>
                    </div>
                    <div class="stat">
                        <span class="stat-value">{player.stats.games_played}</span>
                        <span class="stat-label">{"Games"}</span>
                    </div>
                </div>

                <div class="player-actions">
                    <button 
                        class="action-btn primary"
                        onclick={link.callback(move |_| PlayerSidebarMsg::ViewPlayerProfile(player_id.clone()))}
                        title="View profile"
                    >
                        {"👤"}
                    </button>
                    {if let Some(ref room) = room_id {
                        let room_clone = room.clone();
                        html! {
                            <button 
                                class="action-btn secondary"
                                onclick={link.callback(move |_| PlayerSidebarMsg::JoinPlayerRoom(room_clone.clone()))}
                                title="Join game"
                            >
                                {"🎯"}
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }

    fn render_recent_player(&self, ctx: &Context<Self>, player: &RecentPlayer) -> Html {
        let link = ctx.link();
        let player_id = player.id.clone();
        let player_id_profile = player_id.clone();
        let player_id_friend = player_id.clone();

        let time_ago = Self::format_time_ago(player.last_seen);

        html! {
            <div class="player-card recent">
                <div class="player-info">
                    <div class="player-avatar">
                        {if let Some(ref avatar_url) = player.avatar_url {
                            html! { <img src={avatar_url.clone()} alt="Avatar" /> }
                        } else {
                            html! { <div class="default-avatar">{player.display_name.chars().next().unwrap_or('?')}</div> }
                        }}
                    </div>
                    <div class="player-details">
                        <div class="player-name">{&player.display_name}</div>
                        <div class="last-seen">{"Last seen: "}{time_ago}</div>
                        <div class="games-together">{format!("{} games together", player.games_played_together)}</div>
                    </div>
                </div>
                
                <div class="player-stats-mini">
                    <div class="stat">
                        <span class="stat-value">{format!("{:.1}%", player.stats.win_rate)}</span>
                        <span class="stat-label">{"Win Rate"}</span>
                    </div>
                </div>

                <div class="player-actions">
                    <button 
                        class="action-btn primary"
                        onclick={link.callback(move |_| PlayerSidebarMsg::ViewPlayerProfile(player_id_profile.clone()))}
                        title="View profile"
                    >
                        {"👤"}
                    </button>
                    <button 
                        class="action-btn secondary"
                        onclick={link.callback(move |_| PlayerSidebarMsg::AddFriend(player_id_friend.clone()))}
                        title="Add friend"
                    >
                        {"➕"}
                    </button>
                </div>
            </div>
        }
    }

    fn format_time_ago(time: chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(time);

        if duration.num_minutes() < 60 {
            format!("{}m ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{}h ago", duration.num_hours())
        } else {
            format!("{}d ago", duration.num_days())
        }
    }
}
