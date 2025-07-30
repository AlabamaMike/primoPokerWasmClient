use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GamePageProps {
    pub room_id: String,
}

// Playing Card Component
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub suit: String,
    pub rank: String,
    pub face_up: bool,
    pub classes: Classes,
}

#[function_component(PlayingCard)]
pub fn playing_card(props: &CardProps) -> Html {
    let card_classes = classes!(
        "playing-card",
        if props.face_up { "face-up" } else { "face-down" },
        props.classes.clone()
    );

    html! {
        <div class={card_classes} style="
            width: 35px;
            height: 50px;
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.7rem;
            font-weight: bold;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            margin: 0 1px;
        ">
            if props.face_up {
                <div class="card-face" style="
                    background: white;
                    color: #1f2937;
                    width: 100%;
                    height: 100%;
                    border-radius: 6px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                ">
                    <div class="card-rank" style="font-size: 0.9rem;">{&props.rank}</div>
                    <div class="card-suit" style="font-size: 0.8rem;">{&props.suit}</div>
                </div>
            } else {
                <div class="card-back" style="
                    background: linear-gradient(45deg, #1e40af, #3b82f6);
                    width: 100%;
                    height: 100%;
                    border-radius: 6px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">
                    <div class="card-back-pattern" style="
                        width: 80%;
                        height: 80%;
                        background: repeating-linear-gradient(
                            45deg,
                            rgba(255, 255, 255, 0.1),
                            rgba(255, 255, 255, 0.1) 2px,
                            transparent 2px,
                            transparent 6px
                        );
                        border-radius: 4px;
                    "></div>
                </div>
            }
        </div>
    }
}

// Player Seat Component
#[derive(Properties, PartialEq)]
pub struct PlayerSeatProps {
    pub position: u8,
    pub player_name: Option<String>,
    pub chips: Option<u32>,
    pub is_active: bool,
    pub is_dealer: bool,
    pub current_bet: Option<u32>,
}

#[function_component(PlayerSeat)]
pub fn player_seat(props: &PlayerSeatProps) -> Html {
    let seat_classes = classes!(
        "player-seat",
        format!("position-{}", props.position),
        if props.is_active { "active" } else { "" },
        if props.is_dealer { "dealer" } else { "" }
    );

    // Calculate position styles based on seat position
    let position_style = match props.position {
        1 => "position: absolute; top: 20px; left: 50%; transform: translate(-50%, 0);",
        2 => "position: absolute; top: 25%; right: 80px; transform: translate(0, -50%);",
        3 => "position: absolute; bottom: 25%; right: 80px; transform: translate(0, 50%);",
        4 => "position: absolute; bottom: 25%; left: 80px; transform: translate(0, 50%);",
        5 => "position: absolute; top: 25%; left: 80px; transform: translate(0, -50%);",
        6 => "position: absolute; bottom: 20px; left: 50%; transform: translate(-50%, 0);",
        _ => "position: relative;",
    };

    let base_style = "
        background: rgba(15, 23, 42, 0.8);
        border: 2px solid rgba(16, 185, 129, 0.5);
        border-radius: 12px;
        padding: 0.8rem;
        text-align: center;
        min-width: 100px;
        backdrop-filter: blur(10px);
        color: white;
    ";

    let combined_style = format!("{} {}", position_style, base_style);

    html! {
        <div class={seat_classes} style={combined_style}>
            if let Some(ref name) = props.player_name {
                <>
                    <div class="player-avatar" style="margin-bottom: 0.5rem; position: relative;">
                        <div class="avatar-image" style="
                            width: 40px;
                            height: 40px;
                            border-radius: 50%;
                            background: linear-gradient(135deg, #10b981, #059669);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: white;
                            font-weight: bold;
                            margin: 0 auto;
                        ">{name.chars().next().unwrap_or('?')}</div>
                        if props.is_dealer {
                            <div class="dealer-button" style="
                                position: absolute;
                                top: -5px;
                                right: -5px;
                                width: 20px;
                                height: 20px;
                                background: #fbbf24;
                                border-radius: 50%;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 0.7rem;
                                font-weight: bold;
                                color: #1f2937;
                            ">{"D"}</div>
                        }
                    </div>
                    <div class="player-info">
                        <div class="player-name" style="
                            color: #fbbf24;
                            font-weight: bold;
                            font-size: 0.9rem;
                            margin-bottom: 0.3rem;
                        ">{name}</div>
                        if let Some(chips) = props.chips {
                            <div class="player-chips" style="
                                color: #10b981;
                                font-family: monospace;
                                font-size: 0.8rem;
                                margin-bottom: 0.3rem;
                            ">{format!("${}", chips)}</div>
                        }
                        if let Some(bet) = props.current_bet {
                            <div class="current-bet" style="
                                color: #ef4444;
                                font-size: 0.8rem;
                                font-weight: bold;
                            ">{format!("${}", bet)}</div>
                        }
                    </div>
                    <div class="player-cards">
                        <PlayingCard suit="hidden" rank="hidden" face_up={false} classes={classes!("player-card")} />
                        <PlayingCard suit="hidden" rank="hidden" face_up={false} classes={classes!("player-card")} />
                    </div>
                </>
            } else {
                <div class="empty-seat" style="
                    background: rgba(71, 85, 105, 0.3);
                    border: 2px dashed rgba(148, 163, 184, 0.5);
                    border-radius: 12px;
                    padding: 1rem;
                    text-align: center;
                    min-width: 100px;
                    min-height: 80px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: rgba(148, 163, 184, 0.7);
                    font-size: 0.8rem;
                    cursor: pointer;
                ">
                    {"Join Game"}
                </div>
            }
        </div>
    }
}

// Community Cards Component
#[function_component(CommunityCards)]
pub fn community_cards() -> Html {
    html! {
        <div class="community-cards" style="
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 12px;
            padding: 20px;
            background: rgba(0, 0, 0, 0.2);
            border-radius: 12px;
            backdrop-filter: blur(4px);
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
        ">
            <div class="community-label" style="
                color: #f9fafb;
                font-size: 0.9rem;
                font-weight: 600;
                text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
                margin-bottom: 5px;
            ">{"Community Cards"}</div>
            <div class="cards-row" style="
                display: flex;
                gap: 8px;
                align-items: center;
            ">
                // Flop
                <PlayingCard suit="♠" rank="A" face_up={true} classes={classes!("community-card")} />
                <PlayingCard suit="♥" rank="♥" face_up={true} classes={classes!("community-card")} />
                <PlayingCard suit="♦" rank="K" face_up={true} classes={classes!("community-card")} />
                // Turn
                <PlayingCard suit="♣" rank="J" face_up={true} classes={classes!("community-card")} />
                // River
                <PlayingCard suit="hidden" rank="hidden" face_up={false} classes={classes!("community-card")} />
            </div>
        </div>
    }
}

// Game Actions Component
#[function_component(GameActions)]
pub fn game_actions() -> Html {
    html! {
        <div class="game-actions" style="
            position: fixed;
            bottom: 20px;
            left: 50%;
            transform: translateX(-50%);
            display: flex;
            gap: 12px;
            align-items: center;
            padding: 15px 20px;
            background: rgba(0, 0, 0, 0.8);
            border-radius: 15px;
            backdrop-filter: blur(10px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
            z-index: 100;
        ">
            <button class="action-btn fold-btn" style="
                background: linear-gradient(135deg, #dc2626, #b91c1c);
                color: white;
                border: none;
                padding: 12px 20px;
                border-radius: 8px;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.2s;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            ">{"Fold"}</button>
            <button class="action-btn call-btn" style="
                background: linear-gradient(135deg, #059669, #047857);
                color: white;
                border: none;
                padding: 12px 20px;
                border-radius: 8px;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.2s;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            ">{"Call $50"}</button>
            <button class="action-btn raise-btn" style="
                background: linear-gradient(135deg, #d97706, #b45309);
                color: white;
                border: none;
                padding: 12px 20px;
                border-radius: 8px;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.2s;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            ">{"Raise"}</button>
            <div class="bet-slider" style="
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 8px;
                margin-left: 15px;
            ">
                <input 
                    type="range" 
                    min="50" 
                    max="1000" 
                    value="100" 
                    class="slider" 
                    style="
                        width: 120px;
                        height: 6px;
                        background: #374151;
                        border-radius: 3px;
                        outline: none;
                        cursor: pointer;
                    "
                />
                <span class="bet-amount" style="
                    color: #fbbf24;
                    font-weight: 600;
                    font-size: 0.9rem;
                ">{"$100"}</span>
            </div>
        </div>
    }
}

// Main Game Page Component
#[function_component(GamePage)]
pub fn game_page(props: &GamePageProps) -> Html {
    html! {
        <div class="game-page" style="
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 25%, #0f5132 75%, #134e4a 100%);
            min-height: 100vh;
            color: white;
            display: flex;
            flex-direction: column;
        ">
            <div class="game-header" style="
                background: rgba(15, 23, 42, 0.95);
                backdrop-filter: blur(20px);
                border-bottom: 2px solid rgba(251, 191, 36, 0.3);
                padding: 1rem 2rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
            ">
                <h1 class="room-title" style="
                    color: #fbbf24;
                    font-size: 1.8rem;
                    font-weight: 700;
                    margin: 0;
                ">{format!("Texas Hold'em - Room {}", props.room_id)}</h1>
                <div class="game-info" style="display: flex; gap: 2rem; align-items: center;">
                    <span class="blinds" style="color: #cbd5e1; font-weight: 600;">{"Blinds: $25/$50"}</span>
                    <span class="pot-size" style="color: #cbd5e1; font-weight: 600;">{"Pot: $200"}</span>
                </div>
            </div>

            <div class="poker-table-container" style="
                flex: 1;
                display: flex;
                justify-content: center;
                align-items: center;
                padding: 2rem;
                position: relative;
            ">
                <div class="poker-table" style="
                    width: 800px;
                    height: 500px;
                    background: radial-gradient(ellipse 400px 250px at center, rgba(16, 185, 129, 0.1) 0%, rgba(21, 128, 61, 0.05) 100%);
                    border: 3px solid #10b981;
                    border-radius: 50%;
                    position: relative;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    box-shadow: 
                        0 0 50px rgba(16, 185, 129, 0.3),
                        inset 0 0 30px rgba(0, 0, 0, 0.3);
                ">
                    // Player seats positioned around the table
                    <PlayerSeat 
                        position={1} 
                        player_name={Some("Alice".to_string())} 
                        chips={Some(1500)} 
                        is_active={false} 
                        is_dealer={true}
                        current_bet={Some(50)}
                    />
                    <PlayerSeat 
                        position={2} 
                        player_name={Some("Bob".to_string())} 
                        chips={Some(800)} 
                        is_active={true} 
                        is_dealer={false}
                        current_bet={Some(100)}
                    />
                    <PlayerSeat 
                        position={3} 
                        player_name={Some("Charlie".to_string())} 
                        chips={Some(2200)} 
                        is_active={false} 
                        is_dealer={false}
                        current_bet={None}
                    />
                    <PlayerSeat position={4} player_name={Option::<String>::None} chips={Option::<u32>::None} is_active={false} is_dealer={false} current_bet={Option::<u32>::None} />
                    <PlayerSeat position={5} player_name={Option::<String>::None} chips={Option::<u32>::None} is_active={false} is_dealer={false} current_bet={Option::<u32>::None} />
                    
                    // Player's own seat (position 6)
                    <div class="player-seat position-6">
                        <div class="player-info">
                            <div class="player-name">{"You"}</div>
                            <div class="player-chips">{"$5000"}</div>
                        </div>
                        <div class="player-cards">
                            <PlayingCard suit="♠" rank="A" face_up={true} classes={classes!("hand-card")} />
                            <PlayingCard suit="♥" rank="K" face_up={true} classes={classes!("hand-card")} />
                        </div>
                    </div>

                    // Center area with community cards and pot
                    <div class="table-center">
                        <CommunityCards />
                        <div class="pot-display">
                            <div class="pot-label">{"Total Pot"}</div>
                            <div class="pot-amount">{"$200"}</div>
                        </div>
                    </div>
                </div>
            </div>

            // Game actions at the bottom
            <div class="player-interface">
                <GameActions />
            </div>
        </div>
    }
}
