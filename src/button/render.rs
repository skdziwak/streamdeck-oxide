use ab_glyph::{FontRef, PxScale};
use elgato_streamdeck::{AsyncStreamDeck, StreamDeckError};
use image::GenericImage;
use image::{DynamicImage, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use resvg::tiny_skia::{Color, Pixmap, PremultipliedColorU8, Transform};
use resvg::usvg::{self, Tree};
use std::error::Error;

use super::types::{Button, RenderConfig};

/// Renders a button to a DynamicImage
pub fn render_button(
    button: &Button,
    config: &RenderConfig,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    match button {
        Button::Icon {
            svg_data,
            background,
            foreground,
        } => render_svg(svg_data.as_bytes(), config, *background, *foreground),
        Button::IconWithText {
            svg_data,
            text,
            foreground,
            background,
        } => render_svg_with_text(svg_data.as_bytes(), text, *foreground, *background, config),
        Button::CustomImage { image } => Ok(image.clone()),
        Button::Gradient {
            start_color,
            end_color,
        } => render_gradient(*start_color, *end_color, config),
        Button::Text {
            text,
            foreground,
            background,
        } => render_text(text, *foreground, *background, config),
    }
}

/// Sets a button on the Stream Deck at the specified index
pub async fn set_button(
    deck: &AsyncStreamDeck,
    index: u8,
    button: &Button,
    config: &RenderConfig,
) -> Result<(), StreamDeckError> {
    let image = render_button(button, config).map_err(|e| {
        eprintln!("Error rendering button: {}", e);
        StreamDeckError::BadData
    })?;
    deck.set_button_image(index, image).await?;
    Ok(())
}

// Helper functions for rendering different button types
fn render_text(
    text: &str,
    foreground: Color,
    background: Color,
    config: &RenderConfig,
) -> Result<DynamicImage, Box<dyn Error>> {
    let mut image = DynamicImage::new_rgba8(config.width, config.height);

    let background = Rgba::<u8>([
        (background.red() * 255.0) as u8,
        (background.green() * 255.0) as u8,
        (background.blue() * 255.0) as u8,
        255,
    ]);
    let foreground = Rgba::<u8>([
        (foreground.red() * 255.0) as u8,
        (foreground.green() * 255.0) as u8,
        (foreground.blue() * 255.0) as u8,
        255,
    ]);
    for x in 0..config.width {
        for y in 0..config.height {
            unsafe {
                image.unsafe_put_pixel(x, y, background);
            }
        }
    }

    let font = FontRef::try_from_slice(config.font_data).map_err(|_| "Failed to load font")?;
    let scale = PxScale::from(config.font_scale);
    let text_size = text_size(scale, &font, text);

    draw_text_mut(
        &mut image,
        foreground,
        ((config.width as i32 - text_size.0 as i32) / 2) as i32,
        (config.height as i32 - text_size.1 as i32 - 6) as i32,
        scale,
        &font,
        text,
    );

    Ok(image)
}

fn render_svg(
    svg_data: &[u8],
    config: &RenderConfig,
    background: Color,
    foreground: Color,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let opt = usvg::Options::default();
    let tree = Tree::from_data(svg_data, &opt)?;
    let mut raw_pixmap =
        Pixmap::new(config.width, config.height).ok_or("Failed to create pixmap")?;
    raw_pixmap.fill(Color::from_rgba8(0, 0, 0, 0));

    let scale = config.width as f32 / 40.0;
    let transform = Transform::from_scale(scale, scale).pre_translate(8.0, 6.0);
    resvg::render(&tree, transform, &mut raw_pixmap.as_mut());

    let mut pixmap = Pixmap::new(config.width, config.height).ok_or("Failed to create pixmap")?;
    pixmap.fill(background);
    let pixels_mut = pixmap.pixels_mut();
    for x in 0..config.width {
        for y in 0..config.height {
            let pixel = raw_pixmap.pixel(x, y).ok_or("Failed to get pixel")?;
            if pixel.alpha() > 0 {
                let foreground_red = (foreground.red() * pixel.alpha() as f32) / 255.0;
                let foreground_green = (foreground.green() * pixel.alpha() as f32) / 255.0;
                let foreground_blue = (foreground.blue() * pixel.alpha() as f32) / 255.0;
                let background_red = background.red() * (1.0 - pixel.alpha() as f32 / 255.0);
                let background_green = background.green() * (1.0 - pixel.alpha() as f32 / 255.0);
                let background_blue = background.blue() * (1.0 - pixel.alpha() as f32 / 255.0);
                let final_red = foreground_red + background_red;
                let final_green = foreground_green + background_green;
                let final_blue = foreground_blue + background_blue;
                pixels_mut[x as usize + y as usize * config.width as usize] =
                    PremultipliedColorU8::from_rgba(
                        (final_red * 255.0) as u8,
                        (final_green * 255.0) as u8,
                        (final_blue * 255.0) as u8,
                        255,
                    )
                    .ok_or(format!(
                    "Failed to create premultiplied color: {}, {}, {}, {}",
                    foreground.red(),
                    foreground.green(),
                    foreground.blue(),
                        pixel.alpha()
                ))?;
            }
        }
    }

    let image_rgba8 = DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(config.width, config.height, pixmap.take())
            .ok_or("Failed to create RGBA image")?,
    );

    Ok(image_rgba8)
}

fn render_svg_with_text(
    svg_data: &[u8],
    text: &str,
    foreground: Color,
    background: Color,
    config: &RenderConfig,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = render_svg(svg_data, config, background, foreground)?;

    let font = FontRef::try_from_slice(config.font_data).map_err(|_| "Failed to load font")?;
    let scale = PxScale::from(config.font_scale);
    let text_size = text_size(scale, &font, text);

    draw_text_mut(
        &mut img,
        Rgba([
            (foreground.red() * 255.0) as u8,
            (foreground.green() * 255.0) as u8,
            (foreground.blue() * 255.0) as u8,
            255,
        ]),
        ((config.width as i32 - text_size.0 as i32) / 2) as i32,
        (config.height as i32 - text_size.1 as i32 - 6) as i32,
        scale,
        &font,
        text,
    );

    Ok(img)
}

fn render_gradient(
    start_color: Rgba<u8>,
    end_color: Rgba<u8>,
    config: &RenderConfig,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut image = DynamicImage::new_rgba8(config.width, config.height);

    for y in 0..config.height {
        for x in 0..config.width {
            let r = interpolate(start_color[0], end_color[0], x, y, config);
            let g = interpolate(start_color[1], end_color[1], x, y, config);
            let b = interpolate(start_color[2], end_color[2], x, y, config);
            image.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    Ok(image)
}

fn interpolate(start: u8, end: u8, x: u32, y: u32, config: &RenderConfig) -> u8 {
    let t = (x as f32 / config.width as f32 + y as f32 / config.height as f32) / 2.0;
    (start as f32 * (1.0 - t) + end as f32 * t) as u8
}
