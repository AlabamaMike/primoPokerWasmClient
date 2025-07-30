use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::types::{AppRoute, User};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // Check if user is authenticated
    let is_authenticated = LocalStorage::get::<User>("primo_poker_user").is_ok();
    
    html! {
        <div class="home-page">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="hero-title">{"Welcome to Primo Poker"}</h1>
                    <p class="hero-subtitle">{"Your premium WebAssembly poker experience"}</p>
                    
                    <div class="hero-actions">
                        if is_authenticated {
                            <Link<AppRoute> to={AppRoute::Lobby} classes="btn btn-primary btn-large">
                                {"Enter Lobby"}
                            </Link<AppRoute>>
                            <Link<AppRoute> to={AppRoute::Profile} classes="btn btn-secondary">
                                {"View Profile"}
                            </Link<AppRoute>>
                        } else {
                            <Link<AppRoute> to={AppRoute::Login} classes="btn btn-primary btn-large">
                                {"Sign In"}
                            </Link<AppRoute>>
                            <Link<AppRoute> to={AppRoute::Register} classes="btn btn-secondary">
                                {"Create Account"}
                            </Link<AppRoute>>
                        }
                    </div>
                </div>
            </div>
            
            <div class="features-section">
                <h2>{"Why Choose Primo Poker?"}</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <h3>{"⚡ Lightning Fast"}</h3>
                        <p>{"Built with WebAssembly for optimal performance"}</p>
                    </div>
                    <div class="feature-card">
                        <h3>{"🎯 Fair Play"}</h3>
                        <p>{"Provably fair algorithms and secure gameplay"}</p>
                    </div>
                    <div class="feature-card">
                        <h3>{"🌍 Play Anywhere"}</h3>
                        <p>{"Cross-platform support - play on any device"}</p>
                    </div>
                    <div class="feature-card">
                        <h3>{"👥 Social Gaming"}</h3>
                        <p>{"Connect with players from around the world"}</p>
                    </div>
                </div>
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
