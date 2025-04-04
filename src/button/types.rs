use image::DynamicImage;
use image::Rgba;
use resvg::tiny_skia::Color;

/// Represents different types of buttons for the Stream Deck
#[derive(Clone)]
pub enum Button {
    /// A button with just an SVG icon
    Icon {
        svg_data: &'static str,
        background: Color,
        foreground: Color,
    },
    /// A button with an SVG icon and text label
    IconWithText {
        svg_data: &'static str,
        text: String,
        background: Color,
        foreground: Color,
    },
    /// A button with only text
    Text {
        text: String,
        background: Color,
        foreground: Color,
    },
    /// A button with a custom image
    CustomImage { image: DynamicImage },
    /// A button with a gradient background
    Gradient {
        start_color: Rgba<u8>,
        end_color: Rgba<u8>,
    },
}

/// Configuration for rendering buttons
pub struct RenderConfig {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) font_data: &'static [u8],
    pub(crate) font_scale: f32,
}

impl RenderConfig {
    /// Create a new render config
    pub fn new(width: u32, height: u32, font_data: &'static [u8], font_scale: f32) -> Self {
        RenderConfig {
            width,
            height,
            font_data,
            font_scale,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            width: 72,
            height: 72,
            font_data: include_bytes!("../../fonts/Roboto-Medium.ttf"),
            font_scale: 14.0,
        }
    }
}

impl Button {
    /// Create a new text button
    pub fn text(text: impl Into<String>, background: Color, foreground: Color) -> Self {
        Button::Text {
            text: text.into(),
            background,
            foreground,
        }
    }

    /// Create a new icon button
    pub fn icon(svg_data: &'static str, background: Color, foreground: Color) -> Self {
        Button::Icon {
            svg_data,
            background,
            foreground,
        }
    }

    /// Create a new icon with text button
    pub fn icon_with_text(
        svg_data: &'static str,
        text: impl Into<String>,
        background: Color,
        foreground: Color,
    ) -> Self {
        Button::IconWithText {
            svg_data,
            text: text.into(),
            background,
            foreground,
        }
    }

    /// Create a new custom image button
    pub fn custom_image(image: DynamicImage) -> Self {
        Button::CustomImage { image }
    }

    /// Create a new gradient button
    pub fn gradient(start_color: Rgba<u8>, end_color: Rgba<u8>) -> Self {
        Button::Gradient {
            start_color,
            end_color,
        }
    }
}
