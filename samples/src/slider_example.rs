use ptgui::prelude::*;
use raylib::prelude::*;

fn main() {
    let (mut rl_handler, rl_thread) = raylib::init().size(1280, 720).title("Slider Test").build();
    rl_handler.set_target_fps(60);

    let mut g_handler = GuiHandler::<()>::new(Colour::WHITE);
    g_handler
        .add_slider_with_position(0, 255, 0.0, (100, 100))
        .add_slider(0, 10, 0.0)
        .add_slider_with_position(69, 420, 0.0, (500, 0))
        .add_slider(10, 20, 10.0)
        .set_components_fix_widths(true);

    while !rl_handler.window_should_close() {
        println!(
            "Slider 1: {}, Slider 2: {}",
            g_handler.get_slider_value(0).unwrap(),
            g_handler.get_slider_value(1).unwrap()
        );
        let mut draw_handler = g_handler.draw(&mut rl_handler, &rl_thread).unwrap();

        draw_handler.draw_fps(0, 0);
    }
}
