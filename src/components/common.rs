// Common UI components used throughout the application
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::types::AppRoute;
use crate::auth::AuthState;

// Header Component
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub auth_state: AuthState,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let navigator = use_navigator().unwrap();
    
    let on_logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // Clear stored user data on logout
            LocalStorage::delete("primo_poker_user");
            LocalStorage::delete("primo_poker_remember");
            LocalStorage::delete("primo_poker_username");
            
            // Navigate to home page
            navigator.push(&AppRoute::Home);
        })
    };
    
    html! {
        <header class="app-header">
            <nav class="navbar">
                <div class="nav-brand">
                    <Link<AppRoute> to={AppRoute::Home} classes="brand-link">
                        <img src="/assets/logo.png" alt="PrimoPoker" class="logo" />
                        <span class="brand-text">{"PrimoPoker"}</span>
                    </Link<AppRoute>>
                </div>
                
                <div class="nav-links">
                    if props.auth_state.is_authenticated() {
                        <Link<AppRoute> to={AppRoute::Lobby} classes="nav-link">
                            {"Lobby"}
                        </Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Profile} classes="nav-link">
                            {"Profile"}
                        </Link<AppRoute>>
                        
                        if let Some(user) = props.auth_state.get_user() {
                            <div class="user-info">
                                <span class="username">{&user.display_name}</span>
                                <span class="chips">{format!("${}", user.chips)}</span>
                                <button onclick={on_logout} class="logout-btn">{"Logout"}</button>
                            </div>
                        }
                    } else {
                        <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                            {"Login"}
                        </Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Register} classes="nav-link btn-primary">
                            {"Register"}
                        </Link<AppRoute>>
                    }
                </div>
            </nav>
        </header>
    }
}

// Footer Component
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <div class="footer-content">
                <div class="footer-section">
                    <h4>{"PrimoPoker"}</h4>
                    <p>{"High-performance real-time poker gaming"}</p>
                </div>
                
                <div class="footer-section">
                    <h4>{"Game"}</h4>
                    <ul>
                        <li><a href="#rules">{"Rules"}</a></li>
                        <li><a href="#strategy">{"Strategy"}</a></li>
                        <li><a href="#tournaments">{"Tournaments"}</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>{"Support"}</h4>
                    <ul>
                        <li><a href="#help">{"Help Center"}</a></li>
                        <li><a href="#contact">{"Contact Us"}</a></li>
                        <li><a href="#faq">{"FAQ"}</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>{"Legal"}</h4>
                    <ul>
                        <li><a href="#terms">{"Terms of Service"}</a></li>
                        <li><a href="#privacy">{"Privacy Policy"}</a></li>
                        <li><a href="#responsible">{"Responsible Gaming"}</a></li>
                    </ul>
                </div>
            </div>
            
            <div class="footer-bottom">
                <p>{"© 2025 PrimoPoker. All rights reserved."}</p>
            </div>
        </footer>
    }
}

// Loading Spinner Component
#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    #[prop_or_default]
    pub message: Option<String>,
    #[prop_or(false)]
    pub fullscreen: bool,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    let class = if props.fullscreen {
        "loading-spinner fullscreen"
    } else {
        "loading-spinner"
    };
    
    html! {
        <div class={class}>
            <div class="spinner">
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
            </div>
            
            if let Some(ref message) = props.message {
                <p class="loading-message">{message}</p>
            }
        </div>
    }
}

// Button Component
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub button_type: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = vec!["btn"];
    
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    
    if props.disabled {
        classes.push("disabled");
    }
    
    let class_str = classes.join(" ");
    
    let button_type = props.button_type.as_deref().unwrap_or("button").to_string();
    
    html! {
        <button
            class={class_str}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={button_type}
        >
            {for props.children.iter()}
        </button>
    }
}

// Input Field Component
#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    pub name: String,
    pub label: String,
    pub input_type: String,
    pub value: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub error: Option<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };
    
    let input_class = if props.error.is_some() {
        "form-input error"
    } else {
        "form-input"
    };
    
    html! {
        <div class="form-field">
            <label for={props.name.clone()} class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{"*"}</span>
                }
            </label>
            
            <input
                id={props.name.clone()}
                name={props.name.clone()}
                type={props.input_type.clone()}
                class={input_class}
                value={props.value.clone()}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                required={props.required}
                disabled={props.disabled}
                onchange={onchange}
            />
            
            if let Some(ref error) = props.error {
                <div class="form-error">
                    {error}
                </div>
            }
        </div>
    }
}

// Modal Component
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    pub is_open: bool,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or(true)]
    pub closable: bool,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_backdrop_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_close.emit(());
        })
    };
    
    let on_content_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });
    
    if !props.is_open {
        return html! {};
    }
    
    html! {
        <div class="modal-overlay" onclick={on_backdrop_click}>
            <div class="modal-content" onclick={on_content_click}>
                if props.closable {
                    <button class="modal-close" onclick={props.on_close.reform(|_| ())}>
                        {"×"}
                    </button>
                }
                
                if let Some(ref title) = props.title {
                    <div class="modal-header">
                        <h2>{title}</h2>
                    </div>
                }
                
                <div class="modal-body">
                    {for props.children.iter()}
                </div>
            </div>
        </div>
    }
}
