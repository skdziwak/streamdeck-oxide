//! customizable view implementation for the view system.
//!
//! This module provides a customizable view implementation for the view system.
//! customizable views allow for programmatic creation of views with custom buttons.

use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use generic_array::{sequence::GenericSequence, GenericArray, ArrayLength};
use tokio::sync::mpsc;

use crate::navigation::NavigationEntry;

use super::{button::Button, button::ButtonState, matrix::ButtonMatrix, View};

type Matrix<W, H, C, N> = GenericArray<GenericArray<Option<CustomizableViewButton<W, H, C, N>>, W>, H>;

/// A customizable view.
///
/// This struct represents a customizable view in the view system.
/// It allows for programmatic creation of views with custom buttons.
pub struct CustomizableView<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    /// The matrix of buttons.
    pub(crate) matrix: Matrix<W, H, C, N>,
    /// Phantom data for the navigation type.
    pub(crate) _marker: PhantomData<N>,
}

/// A button in a customizable view.
///
/// This enum represents a button in a customizable view.
/// It can be either a navigation button or a custom button.
pub enum CustomizableViewButton<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    /// A navigation button.
    ///
    /// This button navigates to a different view when clicked.
    Navigation {
        /// The navigation entry to navigate to.
        navigation: N,
        /// The button to display.
        button: Button,
        /// Phantom data for the width and height.
        _marker: PhantomData<fn() -> (W, H)>
    },
    /// A custom button.
    ///
    /// This button has custom behavior when clicked.
    Button(Box<dyn CustomButton<C>>),
}

/// A trait for custom buttons.
///
/// This trait is implemented by types that represent custom buttons
/// in a customizable view. It provides methods for getting the button state,
/// fetching state, and handling clicks.
#[async_trait::async_trait]
pub trait CustomButton<C>: Send + Sync + 'static
where
    C: Send + Clone + Sync + 'static,
{
    /// Get the button state.
    ///
    /// This method returns the current state of the button.
    fn get_state(&self) -> Button;

    /// Fetch state for the button.
    ///
    /// This method fetches the state for the button.
    /// It takes the application context.
    async fn fetch(&self, context: &C) -> Result<(), Box<dyn std::error::Error>>;

    /// Handle a button click.
    ///
    /// This method is called when the button is clicked.
    /// It takes the application context.
    async fn click(&self, context: &C) -> Result<(), Box<dyn std::error::Error>>;
}

/// A future that returns a boolean.
pub type FetchFuture =
    Pin<Box<dyn Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send + Sync>>;

/// A function that returns a fetch future.
pub type FetchFunction<C> = Arc<Box<dyn Fn(&C) -> FetchFuture + Send + Sync>>;

/// A future that returns a unit.
pub type ClickFuture =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync>>;

/// A function that returns a click future.
pub type ClickAction<C> = Arc<Box<dyn Fn(&C) -> ClickFuture + Send + Sync>>;

/// A future that returns a unit.
pub type PushFuture =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync>>;

/// A function that returns a push future.
pub type PushFunction<C> = Arc<Box<dyn Fn(&C, bool) -> PushFuture + Send + Sync>>;

/// A toggle button.
///
/// This struct represents a toggle button in a customizable view.
/// It has two states: active and inactive.
pub struct ToggleButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    /// The function to fetch the active state.
    pub(crate) fetch_active: FetchFunction<C>,
    /// The function to push the active state.
    pub(crate) push_active: PushFunction<C>,
    /// The button to display when inactive.
    pub(crate) button: Button,
    /// The button to display when active.
    pub(crate) active_button: Button,
    /// The current active state.
    pub(crate) active: AtomicBool,
}

/// A click button.
///
/// This struct represents a click button in a customizable view.
/// It has a single action that is performed when clicked.
pub struct ClickButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    /// The function to call when clicked.
    pub(crate) push_click: ClickAction<C>,
    /// The button to display.
    pub(crate) button: Button,
}

impl<C> ClickButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    /// Create a new click button.
    ///
    /// This method creates a new click button with the given text,
    /// icon, and action.
    pub fn new<A, F, S>(text: S, icon: Option<&'static str>, action: A) -> Self
    where
        F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static,
        A: Fn(C) -> F + Send + Sync + Clone + 'static,
        S: Into<String>
    {
        ClickButton {
            push_click: Arc::new(Box::new(move |ctx| {
                let action = action.clone();
                let ctx = ctx.clone();
                Box::pin(async move { action(ctx).await })
            })),
            button: Button {
                text: text.into(),
                icon,
                state: ButtonState::Default,
            },
        }
    }
}

impl<C> ToggleButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    /// Create a new toggle button.
    ///
    /// This method creates a new toggle button with the given text,
    /// icon, fetch function, and push function.
    pub fn new<FF, PF, F, P, S>(
        text: S,
        icon: Option<&'static str>,
        fetch_active: F,
        push_active: P,
    ) -> Self
    where
        FF: Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send + Sync + 'static,
        PF: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static,
        F: Fn(C) -> FF + Send + Sync + Clone + 'static,
        P: Fn(C, bool) -> PF + Send + Sync + Clone + 'static,
        S: Into<String>
    {
        let text = text.into();
        ToggleButton {
            fetch_active: Arc::new(Box::new(move |ctx| {
                let fetch_active = fetch_active.clone();
                let ctx = ctx.clone();
                Box::pin(async move { fetch_active(ctx).await })
            })),
            push_active: Arc::new(Box::new(move |ctx, x| {
                let push_active = push_active.clone();
                let ctx = ctx.clone();
                Box::pin(async move { push_active(ctx, x).await })
            })),
            button: Button {
                text: text.clone(),
                icon,
                state: ButtonState::Default,
            },
            active_button: Button {
                text,
                icon,
                state: ButtonState::Active,
            },
            active: AtomicBool::new(false),
        }
    }

    /// Set the active button.
    ///
    /// This method sets the button to display when active.
    pub fn when_active<S: Into<String>>(self, text: S, icon: Option<&'static str>) -> Self {
        ToggleButton {
            active_button: Button {
                text: text.into(),
                icon,
                state: ButtonState::Active,
            },
            ..self
        }
    }
}

#[async_trait::async_trait]
impl<C> CustomButton<C> for ToggleButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    fn get_state(&self) -> Button {
        let current_state = self.active.load(Ordering::SeqCst);
        match current_state {
            true => self.active_button.clone(),
            false => self.button.clone(),
        }
    }

    async fn fetch(&self, context: &C) -> Result<(), Box<dyn std::error::Error>> {
        let new_state = (self.fetch_active)(context).await;
        self.active.store(new_state?, Ordering::SeqCst);
        Ok(())
    }

    async fn click(&self, context: &C) -> Result<(), Box<dyn std::error::Error>> {
        let current_state = self.active.load(Ordering::SeqCst);
        (self.push_active)(context, !current_state).await?;
        self.active.store(!current_state, Ordering::SeqCst);
        Ok(())
    }
}

#[async_trait::async_trait]
impl<C> CustomButton<C> for ClickButton<C>
where
    C: Send + Clone + Sync + 'static,
{
    fn get_state(&self) -> Button {
        self.button.clone()
    }

    async fn fetch(&self, _: &C) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn click(&self, context: &C) -> Result<(), Box<dyn std::error::Error>> {
        (self.push_click)(context).await?;
        Ok(())
    }
}

impl<W, H, C, N> Default for CustomizableView<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    fn default() -> Self {
        CustomizableView::new()
    }
}

impl<W, H, C, N> CustomizableView<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    /// Create a new customizable view.
    pub fn new() -> Self {
        CustomizableView {
            matrix: GenericArray::generate(|_| GenericArray::generate(|_| None)),
            _marker: PhantomData,
        }
    }

    /// Set a button at the given coordinates.
    ///
    /// This method sets a custom button at the given coordinates.
    pub fn set_button(
        &mut self,
        x: usize,
        y: usize,
        button: impl CustomButton<C>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if x < W::to_usize() && y < H::to_usize() {
            self.matrix[y][x] = Some(CustomizableViewButton::Button(Box::new(button)));
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Row or column out of bounds",
            )))
        }
    }

    /// Set a navigation button at the given coordinates.
    ///
    /// This method sets a navigation button at the given coordinates.
    pub fn set_navigation<S: Into<String>>(
        &mut self,
        x: usize,
        y: usize,
        navigation: N,
        text: S,
        icon: Option<&'static str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if x < W::to_usize() && y < H::to_usize() {
            self.matrix[y][x] = Some(CustomizableViewButton::Navigation {
                navigation,
                button: Button {
                    text: text.into(),
                    icon,
                    state: ButtonState::Default,
                },
                _marker: PhantomData,
            });
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Row or column out of bounds",
            )))
        }
    }

    /// Remove a button at the given coordinates.
    ///
    /// This method removes the button at the given coordinates.
    pub fn remove_button(&mut self, x: usize, y: usize) -> Result<(), Box<dyn std::error::Error>> {
        if x < W::to_usize() && y < H::to_usize() {
            self.matrix[y][x] = None;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Row or column out of bounds",
            )))
        }
    }
}

#[async_trait::async_trait]
impl<W, H, C, N> View<W, H, C, N> for CustomizableView<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    async fn render(&self) -> Result<ButtonMatrix<W, H>, Box<dyn std::error::Error>> {
        let mut button_matrix = ButtonMatrix::new();
        for x in 0..W::to_usize() {
            for y in 0..H::to_usize() {
                if let Some(button) = &self.matrix[y][x] {
                    let state = match button {
                        CustomizableViewButton::Navigation { button, .. } => button,
                        CustomizableViewButton::Button(button) => &button.get_state(),
                    };
                    button_matrix.set_button(x, y, state.clone())?;
                }
            }
        }
        Ok(button_matrix)
    }
    
    async fn on_click(
        &self,
        context: &C,
        index: u8,
        navigation: Arc<mpsc::Sender<N>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if (index as usize) < W::to_usize() * H::to_usize() {
            let x = index % W::to_u8();
            let y = index / W::to_u8();
            if let Some(button) = &self.matrix[y as usize][x as usize] {
                match button {
                    CustomizableViewButton::Navigation { navigation: nav, .. } => {
                        navigation.send(nav.clone()).await?;
                    }
                    CustomizableViewButton::Button(button) => {
                        button.click(context).await?;
                    }
                }
            }
            Ok(())
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Button index out of bounds",
            )));
        }
    }

    async fn fetch_all(&self, context: &C) -> Result<(), Box<dyn std::error::Error>> {
        for x in 0..W::to_usize() {
            for y in 0..H::to_usize() {
                if let Some(button) = &self.matrix[y][x] {
                    match button {
                        CustomizableViewButton::Navigation { .. } => {}
                        CustomizableViewButton::Button(button) => {
                            button.fetch(context).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
