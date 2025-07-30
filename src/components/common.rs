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
                        <div class="brand-logo">
                            <img src="/assets/logos/primo-poker-logo.svg" alt="Primo Poker" class="logo" />
                        </div>
                    </Link<AppRoute>>
                </div>
                
                <div class="nav-links">
                    // Demo link available to all users
                    <Link<AppRoute> to={AppRoute::Demo} classes="nav-link">
                        {"Components Demo"}
                    </Link<AppRoute>>
                    
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
            // Use the new poker chip loading animation
            <div class="loading-spinner-graphic"></div>
            
            if let Some(ref message) = props.message {
                <p class="loading-message">{message}</p>
            }
        </div>
    }
}

// Enhanced Button Component with variants
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(ButtonType::Primary)]
    pub variant: ButtonType,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or("button".to_string())]
    pub button_type: String,
    #[prop_or_default]
    pub class: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl ButtonType {
    fn as_class(&self) -> &'static str {
        match self {
            ButtonType::Primary => "btn-primary",
            ButtonType::Secondary => "btn-secondary",
            ButtonType::Success => "btn-success",
            ButtonType::Danger => "btn-danger",
            ButtonType::Warning => "btn-warning",
            ButtonType::Info => "btn-info",
            ButtonType::Light => "btn-light",
            ButtonType::Dark => "btn-dark",
            ButtonType::Link => "btn-link",
        }
    }
}

impl ButtonSize {
    fn as_class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "btn-sm",
            ButtonSize::Medium => "btn-md",
            ButtonSize::Large => "btn-lg",
            ButtonSize::ExtraLarge => "btn-xl",
        }
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = vec!["btn", props.variant.as_class(), props.size.as_class()];
    
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    
    if props.disabled || props.loading {
        classes.push("btn-disabled");
    }
    
    if props.full_width {
        classes.push("btn-full-width");
    }
    
    if props.loading {
        classes.push("btn-loading");
    }
    
    let class_str = classes.join(" ");
    
    html! {
        <button
            class={class_str}
            onclick={props.onclick.clone()}
            disabled={props.disabled || props.loading}
            type={props.button_type.clone()}
        >
            if props.loading {
                <span class="btn-spinner"></span>
            }
            <span class="btn-content">
                {for props.children.iter()}
            </span>
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

// Toast Notification System
#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Option<u32>, // Duration in milliseconds, None for persistent
}

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn as_class(&self) -> &'static str {
        match self {
            ToastType::Success => "toast-success",
            ToastType::Error => "toast-error",
            ToastType::Warning => "toast-warning",
            ToastType::Info => "toast-info",
        }
    }
    
    fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "✓",
            ToastType::Error => "✕",
            ToastType::Warning => "⚠",
            ToastType::Info => "ℹ",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<Toast>,
    pub on_dismiss: Callback<String>, // Toast ID to dismiss
}

#[function_component(ToastContainer)]
pub fn toast_container(props: &ToastContainerProps) -> Html {
    html! {
        <div class="toast-container">
            {for props.toasts.iter().map(|toast| {
                let toast_id = toast.id.clone();
                let on_dismiss = {
                    let callback = props.on_dismiss.clone();
                    let id = toast_id.clone();
                    Callback::from(move |_| callback.emit(id.clone()))
                };
                
                html! {
                    <div key={toast.id.clone()} class={format!("toast {}", toast.toast_type.as_class())}>
                        <div class="toast-icon">
                            {toast.toast_type.icon()}
                        </div>
                        <div class="toast-content">
                            <span class="toast-message">{&toast.message}</span>
                        </div>
                        <button class="toast-close" onclick={on_dismiss}>
                            {"×"}
                        </button>
                    </div>
                }
            })}
        </div>
    }
}

// Badge Component
#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub children: Children,
    #[prop_or(BadgeType::Primary)]
    pub variant: BadgeType,
    #[prop_or_default]
    pub outline: bool,
    #[prop_or_default]
    pub class: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum BadgeType {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl BadgeType {
    fn as_class(&self) -> &'static str {
        match self {
            BadgeType::Primary => "badge-primary",
            BadgeType::Secondary => "badge-secondary",
            BadgeType::Success => "badge-success",
            BadgeType::Danger => "badge-danger",
            BadgeType::Warning => "badge-warning",
            BadgeType::Info => "badge-info",
            BadgeType::Light => "badge-light",
            BadgeType::Dark => "badge-dark",
        }
    }
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let mut classes = vec!["badge", props.variant.as_class()];
    
    if props.outline {
        classes.push("badge-outline");
    }
    
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    
    let class_str = classes.join(" ");
    
    html! {
        <span class={class_str}>
            {for props.children.iter()}
        </span>
    }
}

// Card Component
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub clickable: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let mut classes = vec!["card"];
    
    if props.clickable {
        classes.push("card-clickable");
    }
    
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    
    let class_str = classes.join(" ");
    
    html! {
        <div class={class_str} onclick={props.onclick.clone()}>
            if props.title.is_some() || props.subtitle.is_some() {
                <div class="card-header">
                    if let Some(ref title) = props.title {
                        <h3 class="card-title">{title}</h3>
                    }
                    if let Some(ref subtitle) = props.subtitle {
                        <p class="card-subtitle">{subtitle}</p>
                    }
                </div>
            }
            <div class="card-body">
                {for props.children.iter()}
            </div>
        </div>
    }
}

// Enhanced Form Input Component
#[derive(Properties, PartialEq)]
pub struct EnhancedInputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
}

#[function_component(EnhancedInput)]
pub fn enhanced_input(props: &EnhancedInputProps) -> Html {
    let mut classes = vec!["form-input"];
    if props.error {
        classes.push("error");
    }
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    let class_str = classes.join(" ");

    html! {
        <input
            type={props.input_type.clone()}
            class={class_str}
            value={props.value.clone()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            required={props.required}
            oninput={props.oninput.clone()}
            onchange={props.onchange.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
        />
    }
}

// Skeleton Loading Component
#[derive(Properties, PartialEq)]
pub struct SkeletonProps {
    #[prop_or(100)]
    pub width: u32,
    #[prop_or(20)]
    pub height: u32,
    #[prop_or(false)]
    pub circle: bool,
    #[prop_or(1)]
    pub count: u32,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    let mut classes = vec!["skeleton"];
    if props.circle {
        classes.push("skeleton-circle");
    }
    if let Some(ref class) = props.class {
        classes.push(class);
    }
    let class_str = classes.join(" ");

    let skeleton_style = format!(
        "width: {}px; height: {}px;",
        if props.circle { props.height } else { props.width },
        props.height
    );

    html! {
        <div class="skeleton-container">
            {for (0..props.count).map(|_| html! {
                <div class={class_str.clone()} style={skeleton_style.clone()}></div>
            })}
        </div>
    }
}

// Accessibility-enhanced components
#[derive(Properties, PartialEq)]
pub struct ScreenReaderOnlyProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ScreenReaderOnly)]
pub fn screen_reader_only(props: &ScreenReaderOnlyProps) -> Html {
    html! {
        <span class="sr-only">
            {for props.children.iter()}
        </span>
    }
}

// Validation utilities
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub struct FormValidator;

impl FormValidator {
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        // Simple email validation - in a real app you'd use a proper regex
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            Ok(())
        } else {
            Err(ValidationError {
                field: "email".to_string(),
                message: "Please enter a valid email address".to_string(),
            })
        }
    }

    pub fn validate_required(value: &str, field_name: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            Err(ValidationError {
                field: field_name.to_string(),
                message: format!("{} is required", field_name),
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_min_length(value: &str, min_length: usize, field_name: &str) -> Result<(), ValidationError> {
        if value.len() < min_length {
            Err(ValidationError {
                field: field_name.to_string(),
                message: format!("{} must be at least {} characters", field_name, min_length),
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        if password.len() < 8 {
            return Err(ValidationError {
                field: "password".to_string(),
                message: "Password must be at least 8 characters long".to_string(),
            });
        }

        if !has_upper || !has_lower || !has_digit || !has_special {
            return Err(ValidationError {
                field: "password".to_string(),
                message: "Password must contain uppercase, lowercase, number, and special character".to_string(),
            });
        }

        Ok(())
    }
}

