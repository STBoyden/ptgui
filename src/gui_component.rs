use crate::prelude::Point;
use raylib::prelude::RaylibDrawHandle;

pub trait GuiComponentBehaviour<T> {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle);
    fn is_hovered(&mut self, mouse_position: Point) -> bool;
    fn is_clicked(&mut self, mouse_position: Point, is_clicked: bool) -> T;
}
