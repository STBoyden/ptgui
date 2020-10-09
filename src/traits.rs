use raylib::prelude::RaylibDrawHandle;

/// Allows for external objects to be drawn before any GUI elements are drawn,
/// effectively making a new layer for GUI.
pub trait Drawable {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle);
}
