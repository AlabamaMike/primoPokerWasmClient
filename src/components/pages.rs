use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Welcome to PrimoPoker"}</h1>
            <p>{"Your premium WebAssembly poker experience"}</p>
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
