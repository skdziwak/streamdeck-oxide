//! Button matrix for the view system.
//!
//! This module provides a matrix of buttons for the view system.

use generic_array::{sequence::GenericSequence, ArrayLength, GenericArray};
use std::marker::PhantomData;

use super::button::Button;

/// A matrix of buttons.
///
/// This struct represents a matrix of buttons in the view system.
/// It is parameterized by the width and height of the matrix.
pub struct ButtonMatrix<W, H>
where
    W: ArrayLength,
    H: ArrayLength,
{
    /// The buttons in the matrix.
    pub(crate) buttons: GenericArray<GenericArray<Button, W>, H>,
    /// Phantom data for the width.
    pub(crate) _width: PhantomData<W>,
    /// Phantom data for the height.
    pub(crate) _height: PhantomData<H>,
}

impl<W, H> Default for ButtonMatrix<W, H>
where
    W: ArrayLength,
    H: ArrayLength,
{
    fn default() -> Self {
        ButtonMatrix {
            buttons: GenericArray::generate(|_| GenericArray::generate(|_| Button::default())),
            _width: PhantomData,
            _height: PhantomData,
        }
    }
}

impl<W, H> ButtonMatrix<W, H>
where
    W: ArrayLength,
    H: ArrayLength,
{
    /// Create a new button matrix.
    pub fn new() -> Self {
        ButtonMatrix::default()
    }

    /// Get a button by index.
    ///
    /// This method returns a reference to the button at the given index.
    /// The index is calculated as `y * width + x`.
    pub fn get_button_by_index(&self, index: usize) -> Option<&Button> {
        if index < W::to_usize() * H::to_usize() {
            let x = index % W::to_usize();
            let y = index / W::to_usize();
            Some(&self.buttons[y][x])
        } else {
            None
        }
    }

    /// Get a button by coordinates.
    ///
    /// This method returns a reference to the button at the given coordinates.
    pub fn get_button(&self, x: usize, y: usize) -> Option<&Button> {
        if x < W::to_usize() && y < H::to_usize() {
            Some(&self.buttons[y][x])
        } else {
            None
        }
    }

    /// Set a button by coordinates.
    ///
    /// This method sets the button at the given coordinates.
    pub fn set_button(
        &mut self,
        x: usize,
        y: usize,
        button: Button,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if x < W::to_usize() && y < H::to_usize() {
            self.buttons[y][x] = button;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Button index out of bounds",
            )))
        }
    }

    /// Set a button by index.
    ///
    /// This method sets the button at the given index.
    /// The index is calculated as `y * width + x`.
    pub fn set_button_by_index(
        &mut self,
        index: usize,
        button: Button,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if index < W::to_usize() * H::to_usize() {
            let x = index % W::to_usize();
            let y = index / W::to_usize();
            self.buttons[y][x] = button;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Button index out of bounds",
            )))
        }
    }

    /// Get the width of the matrix.
    pub fn width(&self) -> usize {
        W::to_usize()
    }

    /// Get the height of the matrix.
    pub fn height(&self) -> usize {
        H::to_usize()
    }

    /// Get the total number of buttons in the matrix.
    pub fn size(&self) -> usize {
        W::to_usize() * H::to_usize()
    }
}
