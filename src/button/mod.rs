//! Button types and rendering functionality for Stream Deck buttons.
//!
//! This module provides types and functions for creating and rendering
//! buttons on the Stream Deck.

mod render;
mod types;

// Re-export public items
pub use self::render::{render_button, set_button};
pub use self::types::{Button, RenderConfig};