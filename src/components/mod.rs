// UI Components module
pub mod common;
pub mod auth;
pub mod lobby;
pub mod game;
pub mod pages;
pub mod profile;
pub mod demo;

// Re-export commonly used components
pub use common::{Header, Footer, LoadingSpinner, Button, Badge, Card, Modal, ToastContainer, EnhancedInput, Skeleton, ScreenReaderOnly, ValidationError, FormValidator};
