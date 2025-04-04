//! View system for Stream Deck applications.
//!
//! This module provides types and traits for creating and managing
//! views in a Stream Deck application.

mod button;
mod matrix;
mod manager;
pub mod customizable;

// Re-export public items
pub use self::button::{Button, ButtonState};
pub use self::matrix::ButtonMatrix;
pub use self::manager::DisplayManager;

use std::sync::Arc;
use tokio::sync::mpsc;
use generic_array::ArrayLength;

use crate::navigation::NavigationEntry;

/// A trait for views in a Stream Deck application.
///
/// This trait is implemented by types that represent different views
/// or screens in a Stream Deck application. It provides methods for
/// rendering the view, handling button clicks, and fetching state.
#[async_trait::async_trait]
pub trait View<W, H, C, N>: Send + Sync + 'static
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    /// Render the view to a button matrix.
    ///
    /// This method returns a button matrix that can be used to render
    /// the buttons for this view.
    async fn render(&self) -> Result<ButtonMatrix<W, H>, Box<dyn std::error::Error>>
    where
        W: ArrayLength,
        H: ArrayLength;

    /// Handle a button click.
    ///
    /// This method is called when a button is clicked. It takes the
    /// application context, the button index, and a sender for navigation
    /// events.
    async fn on_click(
        &self,
        context: &C,
        index: u8,
        navigation: Arc<mpsc::Sender<N>>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Fetch state for all buttons in the view.
    ///
    /// This method is called to fetch the state for all buttons in the view.
    /// It takes the application context.
    async fn fetch_all(&self, _context: &C) -> Result<(), Box<dyn std::error::Error>>;
}

/// A trait for view state.
///
/// This trait is implemented by types that represent the state of a view.
/// It provides a method for creating a new instance of the state.
pub trait ViewState: Send + Sync + 'static {
    /// Create a new instance of the view state.
    fn new() -> Self;
}
