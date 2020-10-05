use ptgui::prelude::*;
use raylib::prelude::*;

#[derive(PartialEq)]
enum State {
    Empty,
}

fn main() {
    let (mut rl_handler, rl_thread) = raylib::init().size(1280, 720).title("Label test").build();
    rl_handler.set_target_fps(60);

    let mut g_handler = GuiHandler::new(Colour::WHITE);
    g_handler
        .add_label("Main Menu")
        .add_button("Test button", "")
        .set_button_action_function(|s, a| match a {
            _ => *s = State::Empty,
        })
        .set_components_fix_widths(true);

    while !rl_handler.window_should_close() {
        let mut draw_handler = g_handler.draw(&mut rl_handler, &rl_thread).unwrap();

        draw_handler.draw_fps(0, 0);
    }
}
