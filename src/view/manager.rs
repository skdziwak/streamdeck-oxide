//! Display manager for the view system.
//!
//! This module provides a display manager for the view system.

use std::{marker::PhantomData, sync::Arc};

use elgato_streamdeck::AsyncStreamDeck;
use generic_array::ArrayLength;
use tokio::sync::{mpsc, RwLock};

use crate::{
    button::{render_button, RenderConfig},
    navigation::NavigationEntry,
    theme::Theme,
};

use super::{button::ButtonState, matrix::ButtonMatrix, View};

/// A display manager for the view system.
///
/// This struct manages the display of views on the Stream Deck.
/// It handles rendering, navigation, and event processing.
pub struct DisplayManager<N: NavigationEntry<W, H, C>, W, H, C>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
{
    /// The render configuration.
    pub(crate) config: RenderConfig,
    /// The theme.
    pub(crate) theme: Theme,
    /// The Stream Deck.
    pub(crate) deck: Arc<AsyncStreamDeck>,
    /// The current view.
    pub(crate) view: RwLock<Box<dyn View<W, H, C, N>>>,
    /// Phantom data for the navigation type.
    pub(crate) _navigation: PhantomData<N>,
    /// Phantom data for the width.
    pub(crate) _width: PhantomData<W>,
    /// Phantom data for the height.
    pub(crate) _height: PhantomData<H>,
    /// The sender for navigation events.
    pub(crate) navigation_sender: Arc<mpsc::Sender<N>>,
    /// The application context.
    pub(crate) context: C,
    /// Current navigation entry
    pub(crate) current_navigation: RwLock<N>,
}

impl<N: NavigationEntry<W, H, C>, W, H, C> DisplayManager<N, W, H, C>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
{
    /// Create a new display manager.
    ///
    /// This method creates a new display manager with the given
    /// Stream Deck, render configuration, theme, and context.
    pub async fn new(
        deck: Arc<AsyncStreamDeck>,
        config: RenderConfig,
        theme: Theme,
        context: C,
    ) -> Result<(Self, mpsc::Receiver<N>), Box<dyn std::error::Error>> {
        let (sender, receiver) = mpsc::channel(1);
        let sender = Arc::new(sender);
        Ok((
            Self {
                config,
                theme,
                deck,
                view: RwLock::new(N::default().get_view(context.clone()).await?),
                _navigation: PhantomData,
                _width: PhantomData,
                _height: PhantomData,
                navigation_sender: sender.clone(),
                context,
                current_navigation: RwLock::new(N::default()),
            },
            receiver,
        ))
    }

    /// Navigate to a new view.
    ///
    /// This method navigates to the view associated with the given
    /// navigation entry.
    pub async fn navigate_to(&self, navigation_entry: N) -> Result<(), Box<dyn std::error::Error>> {
        let mut view = self.view.write().await;
        let mut current_navigation = self.current_navigation.write().await;
        *view = navigation_entry.get_view(self.context.clone()).await?;
        *current_navigation = navigation_entry.clone();
        Ok(())
    }

    /// Get current navigation entry.
    ///
    /// This method returns the current navigation entry.
    pub async fn get_current_navigation(
        &self,
    ) -> Result<N, Box<dyn std::error::Error>> {
        let current_navigation = self.current_navigation.read().await;
        Ok(current_navigation.clone())
    }

    /// Render the current view.
    ///
    /// This method renders the current view to the Stream Deck.
    pub async fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let view = self.view.read().await;
        let button_matrix = view.render().await?;
        self.render_matrix(&button_matrix).await?;
        Ok(())
    }

    /// Fetch state for all buttons in the current view.
    ///
    /// This method fetches the state for all buttons in the current view.
    pub async fn fetch_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let view = self.view.read().await;
        let result = view.fetch_all(&self.context).await;
        if let Err(e) = result {
            eprintln!("Error fetching view state: {}", e);
        }
        Ok(())
    }

    /// Render a button matrix to the Stream Deck.
    ///
    /// This method renders the given button matrix to the Stream Deck.
    async fn render_matrix(
        &self,
        button_matrix: &ButtonMatrix<W, H>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for x in 0..W::to_usize() {
            for y in 0..H::to_usize() {
                let button = &button_matrix.buttons[y][x];
                let button_index = (y * W::to_usize() + x) as u8;
                let background_color = match button.state {
                    ButtonState::Default => self.theme.background,
                    ButtonState::Active => self.theme.active_background,
                    ButtonState::Inactive => self.theme.inactive_background,
                    ButtonState::Error => self.theme.error_background,
                    ButtonState::Pressed => self.theme.pressed_background,
                };
                let foreground_color = match button.state {
                    ButtonState::Default => self.theme.foreground_color,
                    ButtonState::Active => self.theme.active_foreground_color,
                    ButtonState::Inactive => self.theme.foreground_color,
                    ButtonState::Error => self.theme.foreground_color,
                    ButtonState::Pressed => self.theme.active_foreground_color,
                };
                let raw_button = match button.icon {
                    Some(icon) => crate::button::Button::IconWithText {
                        svg_data: icon,
                        text: button.text.to_string(),
                        background: background_color,
                        foreground: foreground_color,
                    },
                    None => crate::button::Button::Text {
                        text: button.text.to_string(),
                        background: background_color,
                        foreground: foreground_color,
                    },
                };
                let image = render_button(&raw_button, &self.config)?;
                self.deck.set_button_image(button_index, image).await?;
            }
            self.deck.flush().await?;
        }
        Ok(())
    }

    /// Handle a button press.
    ///
    /// This method is called when a button is pressed. It updates
    /// the button state to pressed.
    pub async fn on_press(&self, button: u8) -> Result<(), Box<dyn std::error::Error>> {
        let view = self.view.read().await;
        let mut button_matrix = view.render().await?;
        let button_index = button as usize;
        let button = button_matrix
            .get_button_by_index(button_index)
            .ok_or("Button not found")?;
        let new_button = button.updated_state(ButtonState::Pressed);
        button_matrix.set_button_by_index(button_index, new_button)?;
        self.render_matrix(&button_matrix).await?;
        Ok(())
    }

    /// Handle a button release.
    ///
    /// This method is called when a button is released. It calls
    /// the on_click method of the current view.
    pub async fn on_release(&self, button: u8) -> Result<(), Box<dyn std::error::Error>> {
        let view = self.view.read().await;
        let result = view
            .on_click(&self.context, button, self.navigation_sender.clone())
            .await;
        if let Err(e) = result {
            eprintln!("Error handling button click: {}", e);
        }
        self.render().await?;
        Ok(())
    }
}
