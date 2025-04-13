//! Theme definitions for Stream Deck buttons.
//!
//! This module provides types and functions for defining and customizing
//! the appearance of Stream Deck buttons.

use resvg::tiny_skia::Color;

/// Defines the visual theme for Stream Deck buttons.
///
/// This struct contains color definitions for various button states
/// and can be customized to match your application's visual style.
#[derive(Clone, Copy)]
pub struct Theme {
    /// Background color for default buttons
    pub(crate) background: Color,
    /// Background color for active buttons
    pub(crate) active_background: Color,
    /// Background color for inactive buttons
    pub(crate) inactive_background: Color,
    /// Background color for pressed buttons
    pub(crate) pressed_background: Color,
    /// Background color for error buttons
    pub(crate) error_background: Color,
    /// Foreground (text/icon) color for default buttons
    pub(crate) foreground_color: Color,
    /// Foreground (text/icon) color for active buttons
    pub(crate) active_foreground_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(20, 20, 25, 255),
            active_background: Color::from_rgba8(235, 51, 148, 255),
            inactive_background: Color::from_rgba8(41, 41, 51, 255),
            pressed_background: Color::from_rgba8(51, 217, 230, 255),
            error_background: Color::from_rgba8(255, 89, 0, 255),
            foreground_color: Color::from_rgba8(242, 242, 255, 255),
            active_foreground_color: Color::from_rgba8(255, 255, 255, 255),
        }
    }
}

impl Theme {
    /// Create a new theme with custom colors.
    pub fn new(
        background: Color,
        active_background: Color,
        inactive_background: Color,
        pressed_background: Color,
        error_background: Color,
        foreground_color: Color,
        active_foreground_color: Color,
    ) -> Self {
        Self {
            background,
            active_background,
            inactive_background,
            pressed_background,
            error_background,
            foreground_color,
            active_foreground_color,
        }
    }

    /// Create a dark theme.
    pub fn dark() -> Self {
        Self::default()
    }

    /// Create a light theme.
    pub fn light() -> Self {
        Self {
            background: Color::from_rgba8(240, 240, 245, 255),
            active_background: Color::from_rgba8(0, 122, 255, 255),
            inactive_background: Color::from_rgba8(200, 200, 210, 255),
            pressed_background: Color::from_rgba8(0, 180, 180, 255),
            error_background: Color::from_rgba8(255, 59, 48, 255),
            foreground_color: Color::from_rgba8(30, 30, 30, 255),
            active_foreground_color: Color::from_rgba8(255, 255, 255, 255),
        }
    }
}
