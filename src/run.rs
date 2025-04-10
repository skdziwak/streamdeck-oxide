use std::{marker::PhantomData, sync::Arc};

use elgato_streamdeck::AsyncStreamDeck;

use crate::{
    button::RenderConfig, navigation::NavigationEntry, theme::Theme, view::DisplayManager,
};

/// Run a Stream Deck application with the specified configuration.
///
/// This function takes a theme, render configuration, Stream Deck instance,
/// and application context, and runs the main event loop.
pub async fn run<N, W, H, C>(
    theme: Theme,
    config: RenderConfig,
    deck: Arc<AsyncStreamDeck>,
    context: C,
) -> Result<(), Box<dyn std::error::Error>>
where
    W: generic_array::ArrayLength,
    H: generic_array::ArrayLength,
    C: Send + Sync + Clone + 'static,
    N: NavigationEntry<W, H, C>,
{
    let (display_manager, mut navigation_receiver) =
        DisplayManager::<N, W, H, C>::new(deck.clone(), config, theme, context).await?;

    display_manager.fetch_all().await?;
    display_manager.render().await?;

    let reader = deck.get_reader();
    loop {
        let events_future = reader.read(10.0);
        let navigation_future = navigation_receiver.recv();
        tokio::select! {
            events = events_future => {
                let events = events?;
                for event in events {
                    match event {
                        elgato_streamdeck::DeviceStateUpdate::ButtonDown(id) => {
                            display_manager.on_press(id).await?;
                        }
                        elgato_streamdeck::DeviceStateUpdate::ButtonUp(id) => {
                            display_manager.on_release(id).await?;
                        }
                        _ => {}
                    }
                }
            }
            Some(navigation) = navigation_future => {
                display_manager.navigate_to(navigation).await?;
                display_manager.fetch_all().await?;
                display_manager.render().await?;
            }
        }
    }
}

pub struct ExternalTrigger<N, W, H, C> {
    /// The navigation entry.
    pub(crate) navigation: N,
    /// Whether to force switching views.
    pub(crate) switch_view: bool,
    pub(crate) _marker: PhantomData<(W, H, C)>,
}

impl<N, W, H, C> ExternalTrigger<N, W, H, C> {
    /// Create a new external trigger.
    pub fn new(navigation: N, switch_view: bool) -> Self {
        Self {
            navigation,
            switch_view,
            _marker: PhantomData,
        }
    }
}

/// Run a Stream Deck application with the specified configuration and external triggers.
///
/// This function takes a theme, render configuration, Stream Deck instance,
/// aplication context, and external triggers, and runs the main event loop.
pub async fn run_with_external_triggers<N, W, H, C>(
    theme: Theme,
    config: RenderConfig,
    deck: Arc<AsyncStreamDeck>,
    context: C,
    mut receiver: tokio::sync::mpsc::Receiver<ExternalTrigger<N, W, H, C>>,
) -> Result<(), Box<dyn std::error::Error>>
where
    W: generic_array::ArrayLength,
    H: generic_array::ArrayLength,
    C: Send + Sync + Clone + 'static,
    N: NavigationEntry<W, H, C>,
{
    let (display_manager, mut navigation_receiver) =
        DisplayManager::<N, W, H, C>::new(deck.clone(), config, theme, context).await?;

    display_manager.fetch_all().await?;
    display_manager.render().await?;

    let reader = deck.get_reader();
    loop {
        let events_future = reader.read(10.0);
        let navigation_future = navigation_receiver.recv();
        let trigger_future = receiver.recv();
        tokio::select! {
            events = events_future => {
                let events = events?;
                for event in events {
                    match event {
                        elgato_streamdeck::DeviceStateUpdate::ButtonDown(id) => {
                            display_manager.on_press(id).await?;
                        }
                        elgato_streamdeck::DeviceStateUpdate::ButtonUp(id) => {
                            display_manager.on_release(id).await?;
                        }
                        _ => {}
                    }
                }
            }
            Some(navigation) = navigation_future => {
                display_manager.navigate_to(navigation).await?;
                display_manager.fetch_all().await?;
                display_manager.render().await?;
            }
            Some(trigger) = trigger_future => {
                if trigger.switch_view || trigger.navigation == display_manager.get_current_navigation().await? {
                    display_manager.navigate_to(trigger.navigation).await?;
                    display_manager.fetch_all().await?;
                    display_manager.render().await?;
                }
            }
        }
    }
}
