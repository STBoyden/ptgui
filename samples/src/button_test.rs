use ptgui::prelude::*;
use raylib::prelude::*;

#[derive(PartialEq, Debug)]
enum State {
    None,
    Quit,
    PrintWord,
}

fn main() {
    let (mut rl_handler, rl_thread) = raylib::init().size(1280, 720).title("Button Test").build();
    rl_handler.set_target_fps(60);
    let mut state = State::None;

    let mut g_handler = GuiHandler::new(Colour::WHITE);
    g_handler
        .add_button_with_position("Hello", "print_word", (100, 0))
        .add_button("Goodbye", "quit")
        .add_button("Hello again", "print_word")
        .add_button("Hello again again", "print_word")
        .add_button_with_position("Oop I'm over here now", "", (600, 100))
        .add_button("Wooop", "")
        .set_components_fix_widths(true)
        .set_button_action_function(|state, action| match action {
            "quit" => *state = State::Quit,
            "print_word" => *state = State::PrintWord,
            _ => {}
        });

    while !rl_handler.window_should_close() && state != State::Quit {
        let mut draw_handler = g_handler
            .execute_actions(&mut state)
            .draw(&mut rl_handler, &rl_thread)
            .unwrap();

        if state == State::PrintWord {
            println!("Hello");
            state = State::None;
        }

        draw_handler.draw_fps(0, 0);
    }
}
