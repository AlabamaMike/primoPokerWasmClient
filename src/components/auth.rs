use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::types::{LoginCredentials, RegisterData, AppRoute, User};
use crate::services::auth_service::AuthService;

// Login Component
pub struct LoginPage {
    credentials: LoginCredentials,
    loading: bool,
    error_message: Option<String>,
    remember_me: bool,
    show_password: bool,
    auth_service: AuthService,
}

pub enum LoginMsg {
    UpdateUsername(String),
    UpdatePassword(String),
    ToggleRememberMe,
    TogglePasswordVisibility,
    SubmitLogin,
    LoginSuccess,
    LoginError(String),
    ClearError,
}

impl Component for LoginPage {
    type Message = LoginMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Try to load saved credentials if "remember me" was checked
        let saved_username = if LocalStorage::get::<bool>("primo_poker_remember").unwrap_or(false) {
            LocalStorage::get::<String>("primo_poker_username").unwrap_or_default()
        } else {
            String::new()
        };

        let link = ctx.link().clone();
        let on_auth_success = link.callback(|_: User| LoginMsg::LoginSuccess);
        let on_auth_error = link.callback(|error: String| LoginMsg::LoginError(error));

        Self {
            credentials: LoginCredentials {
                username: saved_username,
                password: String::new(),
            },
            loading: false,
            error_message: None,
            remember_me: LocalStorage::get::<bool>("primo_poker_remember").unwrap_or(false),
            show_password: false,
            auth_service: AuthService::new(on_auth_success, on_auth_error),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::UpdateUsername(username) => {
                self.credentials.username = username;
                self.error_message = None;
                true
            }
            LoginMsg::UpdatePassword(password) => {
                self.credentials.password = password;
                self.error_message = None;
                true
            }
            LoginMsg::ToggleRememberMe => {
                self.remember_me = !self.remember_me;
                true
            }
            LoginMsg::TogglePasswordVisibility => {
                self.show_password = !self.show_password;
                true
            }
            LoginMsg::SubmitLogin => {
                if self.credentials.username.trim().is_empty() {
                    self.error_message = Some("Username is required".to_string());
                    return true;
                }
                if self.credentials.password.is_empty() {
                    self.error_message = Some("Password is required".to_string());
                    return true;
                }

                self.loading = true;
                self.error_message = None;

                // Save remember me preference
                let _ = LocalStorage::set("primo_poker_remember", &self.remember_me);
                if self.remember_me {
                    let _ = LocalStorage::set("primo_poker_username", &self.credentials.username);
                } else {
                    LocalStorage::delete("primo_poker_username");
                }

                // Perform async login
                let credentials = self.credentials.clone();
                let auth_service = self.auth_service.clone();
                let link = ctx.link().clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match auth_service.login(credentials).await {
                        Ok(_) => {
                            link.send_message(LoginMsg::LoginSuccess);
                        }
                        Err(error) => {
                            link.send_message(LoginMsg::LoginError(error.to_string()));
                        }
                    }
                });
                true
            }
            LoginMsg::LoginSuccess => {
                self.loading = false;
                // Navigation to lobby will be handled by the app component
                // after it receives the UserLoggedIn message
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Lobby);
                true
            }
            LoginMsg::LoginError(error) => {
                self.loading = false;
                self.error_message = Some(error);
                true
            }
            LoginMsg::ClearError => {
                self.error_message = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        let on_username_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            LoginMsg::UpdateUsername(input.value())
        });

        let on_password_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            LoginMsg::UpdatePassword(input.value())
        });

        let on_submit = link.callback(|e: SubmitEvent| {
            e.prevent_default();
            LoginMsg::SubmitLogin
        });

        let on_remember_toggle = link.callback(|_| LoginMsg::ToggleRememberMe);
        let on_password_toggle = link.callback(|_| LoginMsg::TogglePasswordVisibility);
        let on_clear_error = link.callback(|_| LoginMsg::ClearError);

        html! {
            <div class="auth-container">
                <div class="auth-card">
                    <div class="auth-header">
                        <h1 class="auth-title">{"Sign In"}</h1>
                        <p class="auth-subtitle">{"Welcome back to Primo Poker"}</p>
                    </div>

                    if let Some(ref error) = self.error_message {
                        <div class="error-banner">
                            <span class="error-text">{error}</span>
                            <button class="error-close" onclick={on_clear_error}>{"×"}</button>
                        </div>
                    }

                    <form class="auth-form" onsubmit={on_submit}>
                        <div class="form-group">
                            <label for="username" class="form-label">{"Username or Email"}</label>
                            <input
                                type="text"
                                id="username"
                                class="form-input"
                                placeholder="Enter your username or email"
                                value={self.credentials.username.clone()}
                                onchange={on_username_change}
                                disabled={self.loading}
                                autocomplete="username"
                            />
                        </div>

                        <div class="form-group">
                            <label for="password" class="form-label">{"Password"}</label>
                            <div class="password-input-wrapper">
                                <input
                                    type={if self.show_password { "text" } else { "password" }}
                                    id="password"
                                    class="form-input"
                                    placeholder="Enter your password"
                                    value={self.credentials.password.clone()}
                                    onchange={on_password_change}
                                    disabled={self.loading}
                                    autocomplete="current-password"
                                />
                                <button
                                    type="button"
                                    class="password-toggle"
                                    onclick={on_password_toggle}
                                    title={if self.show_password { "Hide password" } else { "Show password" }}
                                >
                                    if self.show_password {
                                        {"👁️"}
                                    } else {
                                        {"👁️‍🗨️"}
                                    }
                                </button>
                            </div>
                        </div>

                        <div class="form-options">
                            <label class="checkbox-label">
                                <input
                                    type="checkbox"
                                    checked={self.remember_me}
                                    onchange={on_remember_toggle}
                                    disabled={self.loading}
                                />
                                <span class="checkbox-text">{"Remember me"}</span>
                            </label>
                            <Link<AppRoute> to={AppRoute::Register} classes="forgot-password-link">
                                {"Forgot password?"}
                            </Link<AppRoute>>
                        </div>

                        <button
                            type="submit"
                            class={classes!("auth-button", "primary", if self.loading { Some("loading") } else { None })}
                            disabled={self.loading}
                        >
                            if self.loading {
                                <span class="button-spinner"></span>
                                {"Signing in..."}
                            } else {
                                {"Sign In"}
                            }
                        </button>
                    </form>

                    <div class="auth-footer">
                        <p class="auth-footer-text">
                            {"Don't have an account? "}
                            <Link<AppRoute> to={AppRoute::Register} classes="auth-link">
                                {"Sign up"}
                            </Link<AppRoute>>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}

// Registration Component  
pub struct RegisterPage {
    form_data: RegisterData,
    confirm_password: String,
    loading: bool,
    errors: std::collections::HashMap<String, String>,
    terms_accepted: bool,
    show_password: bool,
    show_confirm_password: bool,
    auth_service: AuthService,
}

pub enum RegisterMsg {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdateDisplayName(String),
    UpdatePassword(String),
    UpdateConfirmPassword(String),
    ToggleTermsAccepted,
    TogglePasswordVisibility,
    ToggleConfirmPasswordVisibility,
    SubmitRegistration,
    RegistrationSuccess,
    RegistrationError(String),
    ValidationError(String, String), // field, error
    ClearError(String),
}

impl Component for RegisterPage {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let on_auth_success = link.callback(|_: User| RegisterMsg::RegistrationSuccess);
        let on_auth_error = link.callback(|error: String| RegisterMsg::RegistrationError(error));

        Self {
            form_data: RegisterData {
                username: String::new(),
                email: String::new(),
                password: String::new(),
                display_name: String::new(),
            },
            confirm_password: String::new(),
            loading: false,
            errors: std::collections::HashMap::new(),
            terms_accepted: false,
            show_password: false,
            show_confirm_password: false,
            auth_service: AuthService::new(on_auth_success, on_auth_error),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::UpdateUsername(username) => {
                self.form_data.username = username;
                self.errors.remove("username");
                self.validate_username();
                true
            }
            RegisterMsg::UpdateEmail(email) => {
                self.form_data.email = email;
                self.errors.remove("email");
                self.validate_email();
                true
            }
            RegisterMsg::UpdateDisplayName(display_name) => {
                self.form_data.display_name = display_name;
                self.errors.remove("display_name");
                true
            }
            RegisterMsg::UpdatePassword(password) => {
                self.form_data.password = password;
                self.errors.remove("password");
                self.validate_password();
                if !self.confirm_password.is_empty() {
                    self.validate_password_match();
                }
                true
            }
            RegisterMsg::UpdateConfirmPassword(confirm_password) => {
                self.confirm_password = confirm_password;
                self.errors.remove("confirm_password");
                self.validate_password_match();
                true
            }
            RegisterMsg::ToggleTermsAccepted => {
                self.terms_accepted = !self.terms_accepted;
                if self.terms_accepted {
                    self.errors.remove("terms");
                }
                true
            }
            RegisterMsg::TogglePasswordVisibility => {
                self.show_password = !self.show_password;
                true
            }
            RegisterMsg::ToggleConfirmPasswordVisibility => {
                self.show_confirm_password = !self.show_confirm_password;
                true
            }
            RegisterMsg::SubmitRegistration => {
                self.validate_all_fields();
                
                if !self.errors.is_empty() {
                    return true;
                }

                if !self.terms_accepted {
                    self.errors.insert("terms".to_string(), "You must accept the terms of service".to_string());
                    return true;
                }

                self.loading = true;

                // Perform async registration
                let form_data = self.form_data.clone();
                let auth_service = self.auth_service.clone();
                let link = ctx.link().clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match auth_service.register(form_data).await {
                        Ok(_) => {
                            link.send_message(RegisterMsg::RegistrationSuccess);
                        }
                        Err(error) => {
                            link.send_message(RegisterMsg::RegistrationError(error.to_string()));
                        }
                    }
                });
                true
            }
            RegisterMsg::RegistrationSuccess => {
                self.loading = false;
                // Navigation to lobby will be handled by the app component
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Lobby);
                true
            }
            RegisterMsg::RegistrationError(error) => {
                self.loading = false;
                self.errors.insert("form".to_string(), error);
                true
            }
            RegisterMsg::ValidationError(field, error) => {
                self.errors.insert(field, error);
                true
            }
            RegisterMsg::ClearError(field) => {
                self.errors.remove(&field);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        let on_username_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RegisterMsg::UpdateUsername(input.value())
        });

        let on_email_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RegisterMsg::UpdateEmail(input.value())
        });

        let on_display_name_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RegisterMsg::UpdateDisplayName(input.value())
        });

        let on_password_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RegisterMsg::UpdatePassword(input.value())
        });

        let on_confirm_password_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            RegisterMsg::UpdateConfirmPassword(input.value())
        });

        let on_submit = link.callback(|e: SubmitEvent| {
            e.prevent_default();
            RegisterMsg::SubmitRegistration
        });

        let on_terms_toggle = link.callback(|_| RegisterMsg::ToggleTermsAccepted);
        let on_password_toggle = link.callback(|_| RegisterMsg::TogglePasswordVisibility);
        let on_confirm_password_toggle = link.callback(|_| RegisterMsg::ToggleConfirmPasswordVisibility);

        html! {
            <div class="auth-container">
                <div class="auth-card">
                    <div class="auth-header">
                        <h1 class="auth-title">{"Create Account"}</h1>
                        <p class="auth-subtitle">{"Join Primo Poker today"}</p>
                    </div>

                    if let Some(error) = self.errors.get("form") {
                        <div class="error-banner">
                            <span class="error-text">{error}</span>
                        </div>
                    }

                    <form class="auth-form" onsubmit={on_submit}>
                        <div class="form-group">
                            <label for="reg-username" class="form-label">{"Username"}</label>
                            <input
                                type="text"
                                id="reg-username"
                                class={classes!("form-input", if self.errors.contains_key("username") { Some("error") } else { None })}
                                placeholder="Choose a username"
                                value={self.form_data.username.clone()}
                                onchange={on_username_change}
                                disabled={self.loading}
                                autocomplete="username"
                            />
                            if let Some(error) = self.errors.get("username") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <div class="form-group">
                            <label for="reg-email" class="form-label">{"Email"}</label>
                            <input
                                type="email"
                                id="reg-email"
                                class={classes!("form-input", if self.errors.contains_key("email") { Some("error") } else { None })}
                                placeholder="Enter your email"
                                value={self.form_data.email.clone()}
                                onchange={on_email_change}
                                disabled={self.loading}
                                autocomplete="email"
                            />
                            if let Some(error) = self.errors.get("email") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <div class="form-group">
                            <label for="reg-display-name" class="form-label">{"Display Name"}</label>
                            <input
                                type="text"
                                id="reg-display-name"
                                class={classes!("form-input", if self.errors.contains_key("display_name") { Some("error") } else { None })}
                                placeholder="How others will see you"
                                value={self.form_data.display_name.clone()}
                                onchange={on_display_name_change}
                                disabled={self.loading}
                                autocomplete="name"
                            />
                            if let Some(error) = self.errors.get("display_name") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <div class="form-group">
                            <label for="reg-password" class="form-label">{"Password"}</label>
                            <div class="password-input-wrapper">
                                <input
                                    type={if self.show_password { "text" } else { "password" }}
                                    id="reg-password"
                                    class={classes!("form-input", if self.errors.contains_key("password") { Some("error") } else { None })}
                                    placeholder="Choose a strong password"
                                    value={self.form_data.password.clone()}
                                    onchange={on_password_change}
                                    disabled={self.loading}
                                    autocomplete="new-password"
                                />
                                <button
                                    type="button"
                                    class="password-toggle"
                                    onclick={on_password_toggle}
                                >
                                    if self.show_password { {"👁️"} } else { {"👁️‍🗨️"} }
                                </button>
                            </div>
                            if let Some(error) = self.errors.get("password") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <div class="form-group">
                            <label for="reg-confirm-password" class="form-label">{"Confirm Password"}</label>
                            <div class="password-input-wrapper">
                                <input
                                    type={if self.show_confirm_password { "text" } else { "password" }}
                                    id="reg-confirm-password"
                                    class={classes!("form-input", if self.errors.contains_key("confirm_password") { Some("error") } else { None })}
                                    placeholder="Confirm your password"
                                    value={self.confirm_password.clone()}
                                    onchange={on_confirm_password_change}
                                    disabled={self.loading}
                                    autocomplete="new-password"
                                />
                                <button
                                    type="button"
                                    class="password-toggle"
                                    onclick={on_confirm_password_toggle}
                                >
                                    if self.show_confirm_password { {"👁️"} } else { {"👁️‍🗨️"} }
                                </button>
                            </div>
                            if let Some(error) = self.errors.get("confirm_password") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <div class="form-group">
                            <label class={classes!("checkbox-label", if self.errors.contains_key("terms") { Some("error") } else { None })}>
                                <input
                                    type="checkbox"
                                    checked={self.terms_accepted}
                                    onchange={on_terms_toggle}
                                    disabled={self.loading}
                                />
                                <span class="checkbox-text">
                                    {"I agree to the "}
                                    <a href="/terms" target="_blank" class="terms-link">{"Terms of Service"}</a>
                                    {" and "}
                                    <a href="/privacy" target="_blank" class="terms-link">{"Privacy Policy"}</a>
                                </span>
                            </label>
                            if let Some(error) = self.errors.get("terms") {
                                <span class="field-error">{error}</span>
                            }
                        </div>

                        <button
                            type="submit"
                            class={classes!("auth-button", "primary", if self.loading { Some("loading") } else { None })}
                            disabled={self.loading}
                        >
                            if self.loading {
                                <span class="button-spinner"></span>
                                {"Creating Account..."}
                            } else {
                                {"Create Account"}
                            }
                        </button>
                    </form>

                    <div class="auth-footer">
                        <p class="auth-footer-text">
                            {"Already have an account? "}
                            <Link<AppRoute> to={AppRoute::Login} classes="auth-link">
                                {"Sign in"}
                            </Link<AppRoute>>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}

impl RegisterPage {
    fn validate_username(&mut self) {
        let username = &self.form_data.username;
        if username.len() < 3 {
            self.errors.insert("username".to_string(), "Username must be at least 3 characters".to_string());
        } else if username.len() > 20 {
            self.errors.insert("username".to_string(), "Username must be no more than 20 characters".to_string());
        } else if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            self.errors.insert("username".to_string(), "Username can only contain letters, numbers, _ and -".to_string());
        }
    }

    fn validate_email(&mut self) {
        let email = &self.form_data.email;
        if !email.contains('@') || !email.contains('.') {
            self.errors.insert("email".to_string(), "Please enter a valid email address".to_string());
        }
    }

    fn validate_password(&mut self) {
        let password = &self.form_data.password;
        if password.len() < 8 {
            self.errors.insert("password".to_string(), "Password must be at least 8 characters".to_string());
        } else if !password.chars().any(|c| c.is_uppercase()) {
            self.errors.insert("password".to_string(), "Password must contain at least one uppercase letter".to_string());
        } else if !password.chars().any(|c| c.is_lowercase()) {
            self.errors.insert("password".to_string(), "Password must contain at least one lowercase letter".to_string());
        } else if !password.chars().any(|c| c.is_numeric()) {
            self.errors.insert("password".to_string(), "Password must contain at least one number".to_string());
        }
    }

    fn validate_password_match(&mut self) {
        if self.form_data.password != self.confirm_password {
            self.errors.insert("confirm_password".to_string(), "Passwords do not match".to_string());
        }
    }

    fn validate_all_fields(&mut self) {
        if self.form_data.username.trim().is_empty() {
            self.errors.insert("username".to_string(), "Username is required".to_string());
        } else {
            self.validate_username();
        }

        if self.form_data.email.trim().is_empty() {
            self.errors.insert("email".to_string(), "Email is required".to_string());
        } else {
            self.validate_email();
        }

        if self.form_data.display_name.trim().is_empty() {
            self.errors.insert("display_name".to_string(), "Display name is required".to_string());
        }

        if self.form_data.password.is_empty() {
            self.errors.insert("password".to_string(), "Password is required".to_string());
        } else {
            self.validate_password();
        }

        if self.confirm_password.is_empty() {
            self.errors.insert("confirm_password".to_string(), "Please confirm your password".to_string());
        } else {
            self.validate_password_match();
        }
    }
}
