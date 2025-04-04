//! Button types for the view system.
//!
//! This module provides types for representing buttons in the view system.

/// The state of a button.
#[derive(Clone, Copy)]
pub enum ButtonState {
    /// The default state.
    Default,
    /// The button is pressed.
    Pressed,
    /// The button is active.
    Active,
    /// The button is inactive.
    Inactive,
    /// The button is in an error state.
    Error,
}

/// A button in the view system.
///
/// This struct represents a button in the view system. It contains
/// the text, icon, and state of the button.
#[derive(Clone, Copy)]
pub struct Button {
    /// The text to display on the button.
    pub(crate) text: &'static str,
    /// The icon to display on the button.
    pub(crate) icon: Option<&'static str>,
    /// The state of the button.
    pub(crate) state: ButtonState,
}

impl Button {
    /// Create a new button with the given text, icon, and state.
    pub fn new(text: &'static str, icon: Option<&'static str>, state: ButtonState) -> Self {
        Button { text, icon, state }
    }

    /// Create a new button with the given text.
    pub fn text(text: &'static str) -> Self {
        Button {
            text,
            icon: None,
            state: ButtonState::Default,
        }
    }

    /// Create a new button with the given text and icon.
    pub fn with_icon(text: &'static str, icon: &'static str) -> Self {
        Button {
            text,
            icon: Some(icon),
            state: ButtonState::Default,
        }
    }

    /// Create a new button with the given text and state.
    pub fn with_state(text: &'static str, state: ButtonState) -> Self {
        Button {
            text,
            icon: None,
            state,
        }
    }

    /// Create a new button with the given text, icon, and state.
    pub fn with_icon_and_state(text: &'static str, icon: &'static str, state: ButtonState) -> Self {
        Button {
            text,
            icon: Some(icon),
            state,
        }
    }

    /// Update the text of the button.
    pub fn updated_text(&self, text: &'static str) -> Self {
        Button {
            text,
            icon: self.icon,
            state: self.state,
        }
    }

    /// Update the icon of the button.
    pub fn updated_icon(&self, icon: &'static str) -> Self {
        Button {
            text: self.text,
            icon: Some(icon),
            state: self.state,
        }
    }

    /// Update the state of the button.
    pub fn updated_state(&self, state: ButtonState) -> Self {
        Button {
            text: self.text,
            icon: self.icon,
            state,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Button {
            text: "",
            icon: None,
            state: ButtonState::Default,
        }
    }
}
