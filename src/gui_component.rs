use crate::prelude::Point;
use raylib::prelude::RaylibDrawHandle;

pub trait GuiComponentBehaviour {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle);
    fn is_hovered(&mut self, mouse_position: Point) -> bool;
}
