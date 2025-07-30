use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::AppRoute;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Welcome to PrimoPoker"}</h1>
            <p>{"Your premium WebAssembly poker experience"}</p>
            
            <div class="navigation-links">
                <h2>{"Navigation Test"}</h2>
                <Link<AppRoute> to={AppRoute::Login}>{"Login"}</Link<AppRoute>>
                {" | "}
                <Link<AppRoute> to={AppRoute::Register}>{"Register"}</Link<AppRoute>>
                {" | "}
                <Link<AppRoute> to={AppRoute::Lobby}>{"Lobby"}</Link<AppRoute>>
                {" | "}
                <Link<AppRoute> to={AppRoute::Profile}>{"Profile"}</Link<AppRoute>>
            </div>
        </div>
    }
}

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-page">
            <h1>{"404 - Page Not Found"}</h1>
            <p>{"The page you're looking for doesn't exist."}</p>
        </div>
    }
}
