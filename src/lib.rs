//! # StreamDeck
//!
//! A high-level framework for creating Stream Deck applications in Rust.
//!
//! This library provides a flexible and type-safe way to create applications
//! for the Elgato Stream Deck. It handles button rendering, navigation between
//! views, and event processing.

// Re-export modules
pub mod button;
pub mod error;
pub mod navigation;
pub mod theme;
pub mod view;

// Re-export commonly used items
pub use button::RenderConfig;
pub use elgato_streamdeck;
pub use generic_array;
pub use md_icons;
pub use navigation::NavigationEntry;
pub use theme::Theme;
pub use view::{Button, ButtonState, DisplayManager, View};

/// Run a Stream Deck application with the specified configuration.
///
/// This function takes a theme, render configuration, Stream Deck instance,
/// and application context, and runs the main event loop.
pub use crate::run::run;

// Internal modules
mod run;
