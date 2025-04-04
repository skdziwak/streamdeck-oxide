//! Error types for the Stream Deck library.
//!
//! This module provides custom error types for the Stream Deck library.

use std::fmt;
use std::error::Error as StdError;

/// Errors that can occur when using the Stream Deck library.
#[derive(Debug)]
pub enum Error {
    /// The requested device was not found.
    DeviceNotFound,
    /// An error occurred while communicating with the device.
    DeviceError(String),
    /// An error occurred while rendering a button.
    RenderError(String),
    /// The button index is out of bounds.
    ButtonIndexOutOfBounds(usize),
    /// An error occurred in the underlying Stream Deck library.
    ElgatoError(elgato_streamdeck::StreamDeckError),
    /// An error occurred in the image processing library.
    ImageError(String),
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// A custom error with a message.
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DeviceNotFound => write!(f, "Stream Deck device not found"),
            Error::DeviceError(msg) => write!(f, "Device error: {}", msg),
            Error::RenderError(msg) => write!(f, "Render error: {}", msg),
            Error::ButtonIndexOutOfBounds(index) => {
                write!(f, "Button index {} is out of bounds", index)
            }
            Error::ElgatoError(err) => write!(f, "Elgato Stream Deck error: {}", err),
            Error::ImageError(msg) => write!(f, "Image error: {}", msg),
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            Error::ElgatoError(_) => None, // StreamDeckError doesn't implement StdError
            _ => None,
        }
    }
}

impl From<elgato_streamdeck::StreamDeckError> for Error {
    fn from(err: elgato_streamdeck::StreamDeckError) -> Self {
        Error::ElgatoError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Custom(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Custom(msg.to_string())
    }
}

/// A specialized Result type for Stream Deck operations.
pub type Result<T> = std::result::Result<T, Error>;