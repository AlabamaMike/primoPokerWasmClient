// PrimoPoker WebAssembly Client - Main Library Entry Point
// High-performance real-time poker client for web browsers

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Module declarations
pub mod app;
pub mod components;
pub mod services;
pub mod game;
pub mod auth;
pub mod networking;
pub mod graphics;
pub mod utils;
pub mod types;

// Import the main app component
use app::App;

// This is the entry point for the web app
#[wasm_bindgen(start)]
pub fn run_app() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());
    
    log::info!("Starting PrimoPoker WebAssembly Client");
    
    // Mount the Yew application to the DOM
    yew::Renderer::<App>::new().render();
}

// Export functions that can be called from JavaScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for easier console logging
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Re-export commonly used types
pub use types::*;
pub use game::GameState;
pub use auth::AuthState;
