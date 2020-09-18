use ptgui::prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut rl_handler, rl_thread) = raylib::init()
        .size(1280, 720)
        .title("Dropdown Test")
        .build();
    rl_handler.set_target_fps(60);

    let mut g_handler = GuiHandler::<()>::new(Colour::WHITE);
    g_handler
        .add_slider(69, 420, 69.0)
        .set_components_fix_widths(true)
        .add_dropdown("Test")
        .get_dropdowns_mut()
        .unwrap()
        .get_mut(0)
        .unwrap()
        .add_slider(0, 100, 50.0);

    while !rl_handler.window_should_close() {
        let mut draw_handler = g_handler.draw(&mut rl_handler, &rl_thread).unwrap();

        draw_handler.draw_fps(0, 0);
    }
}
