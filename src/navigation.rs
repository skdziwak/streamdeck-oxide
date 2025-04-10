//! Navigation traits and utilities for Stream Deck applications.
//!
//! This module provides traits and utilities for navigating between
//! different views in a Stream Deck application.

use crate::view::View;
use generic_array::ArrayLength;

type GetViewResult<W, H, C, N> = Result<Box<dyn View<W, H, C, N>>, Box<dyn std::error::Error>>;

/// A trait for navigation entries in a Stream Deck application.
///
/// This trait is implemented by types that represent different views
/// or screens in a Stream Deck application. It provides a method for
/// getting the view associated with a navigation entry.
pub trait NavigationEntry<W, H, C>: Default + Send + Sync + Clone + PartialEq + 'static
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
{
    /// Get the view associated with this navigation entry.
    ///
    /// This method returns a boxed view that can be used to render
    /// the buttons for this navigation entry.
    fn get_view(
        &self,
        context: C,
    ) -> impl std::future::Future<Output = GetViewResult<W, H, C, Self>>;
}

/// A helper trait for creating navigation entries.
///
/// This trait provides a method for creating a navigation entry
/// with a specific view.
pub trait IntoNavigationEntry<W, H, C, N>
where
    W: ArrayLength,
    H: ArrayLength,
    C: Send + Clone + Sync + 'static,
    N: NavigationEntry<W, H, C>,
{
    /// Convert this type into a navigation entry.
    fn into_navigation_entry(self) -> N;
}
