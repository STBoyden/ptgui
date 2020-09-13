use crate::prelude::types::*;
use raylib::prelude::RaylibDrawHandle;

pub trait InternalDrawable {
    fn get_position(&self) -> Point;
    fn get_dimensions(&self) -> Dimensions;
    fn get_type(&self) -> DrawableType;
    fn resize(&mut self, new_dimensions: Dimensions);
}

#[derive(PartialEq)]
pub enum DrawableType {
    Button,
    Slider,
}

/// Allows for external objects to be drawn before any GUI elements are drawn, effectively making
/// a new layer for GUI.
pub trait Drawable {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle);
}
