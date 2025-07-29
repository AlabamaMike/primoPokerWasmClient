use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GamePageProps {
    pub room_id: String,
}

#[function_component(GamePage)]
pub fn game_page(props: &GamePageProps) -> Html {
    html! {
        <div class="game-page">
            <h1>{format!("Game Room: {}", props.room_id)}</h1>
            <p>{"Game interface will be implemented here"}</p>
        </div>
    }
}
