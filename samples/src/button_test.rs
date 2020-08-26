use ptgui::prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut rl_handler, rl_thread) = raylib::init().size(1280, 720).title("Button Test").build();

    while !rl_handler.window_should_close() {
        let draw_handler = rl_handler.begin_drawing(&rl_thread);

        let mut g_handler = GuiHandler::new(draw_handler, Color::WHITE);

        g_handler
            .add_button("Hello", (0, 10))
            .add_button("Goodbye", (0, 60))
            .add_button("Hello again", (0, 110))
            .add_button("Hello again again", (0, 160))
            .set_button_fix_widths(true)
            .draw()
            .unwrap();

        let mut draw_handler = g_handler.release_draw_handle();
        draw_handler.draw_rectangle(600, 10, 100, 100, Color::RED);
    }
}
