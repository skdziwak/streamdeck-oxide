//! Button types for the view system.
//!
//! This module provides types for representing buttons in the view system.

use crate::Theme;

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
#[derive(Clone)]
pub struct Button {
    /// The text to display on the button.
    pub(crate) text: String,
    /// The icon to display on the button.
    pub(crate) icon: Option<&'static str>,
    /// The state of the button.
    pub(crate) state: ButtonState,
    /// Alternative theme
    pub(crate) theme: Option<Theme>,
}

impl Button {
    /// Create a new button with the given text, icon, and state.
    pub fn new(text: String, icon: Option<&'static str>, state: ButtonState) -> Self {
        Button {
            text,
            icon,
            state,
            theme: None,
        }
    }

    /// Create a new button with the given text.
    pub fn text(text: String) -> Self {
        Button {
            text,
            icon: None,
            state: ButtonState::Default,
            theme: None,
        }
    }

    /// Create a new button with the given text and icon.
    pub fn with_icon(text: String, icon: &'static str) -> Self {
        Button {
            text,
            icon: Some(icon),
            state: ButtonState::Default,
            theme: None,
        }
    }

    /// Create a new button with the given text and state.
    pub fn with_state(text: String, state: ButtonState) -> Self {
        Button {
            text,
            icon: None,
            state,
            theme: None,
        }
    }

    /// Create a new button with the given text, icon, and state.
    pub fn with_icon_and_state(text: String, icon: &'static str, state: ButtonState) -> Self {
        Button {
            text,
            icon: Some(icon),
            state,
            theme: None,
        }
    }

    /// Update the text of the button.
    pub fn updated_text(&self, text: String) -> Self {
        Button {
            text,
            icon: self.icon,
            state: self.state,
            theme: self.theme.clone(),
        }
    }

    /// Update the icon of the button.
    pub fn updated_icon(&self, icon: &'static str) -> Self {
        Button {
            text: self.text.clone(),
            icon: Some(icon),
            state: self.state,
            theme: self.theme.clone(),
        }
    }

    /// Update the state of the button.
    pub fn updated_state(&self, state: ButtonState) -> Self {
        Button {
            text: self.text.clone(),
            icon: self.icon,
            state,
            theme: self.theme.clone(),
        }
    }

    /// Update the theme of the button.
    pub fn with_theme(self, theme: Theme) -> Self {
        Button {
            theme: Some(theme),
            ..self
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Button {
            text: "".to_string(),
            icon: None,
            state: ButtonState::Default,
            theme: None,
        }
    }
}
