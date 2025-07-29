use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html! {
        <div class="login-page">
            <h1>{"Login"}</h1>
            <p>{"Login form will be implemented here"}</p>
        </div>
    }
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    html! {
        <div class="register-page">
            <h1>{"Register"}</h1>
            <p>{"Registration form will be implemented here"}</p>
        </div>
    }
}
