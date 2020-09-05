use raylib::prelude::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&mut self, draw_hander: &mut RaylibDrawHandle);
}
