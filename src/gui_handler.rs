use crate::prelude::*;
use raylib::prelude::*;
use std::cell::Cell;

type Action<T> = dyn Fn(&mut T, &str) -> ();
type DrawAction = dyn FnMut(&mut RaylibDrawHandle) -> ();

/// The default `struct` to handle the GUI system implemented by the `ptgui` crate.
pub struct GuiHandler<T> {
    can_draw: Cell<bool>,
    clear_colour: Colour,
    button_fixed_width: bool,
    buttons: Vec<Button>,
    button_action: Box<Action<T>>,
    actions: Vec<String>,
    additional_draws: Vec<Box<DrawAction>>,
    has_set_button_action: bool,
}

impl<T> GuiHandler<T> {
    /// Creates a new `GuiHandler<T>`.
    pub fn new(clear_colour: Colour) -> GuiHandler<T> {
        GuiHandler {
            can_draw: Cell::new(true),
            clear_colour,
            button_fixed_width: false,
            buttons: Vec::new(),
            button_action: Box::new(|_, _| {}),
            actions: Vec::new(),
            additional_draws: Vec::new(),
            has_set_button_action: false,
        }
    }

    fn button_fix_widths(&mut self) {
        let mut widest = -1;

        for button in self.buttons.iter() {
            let width = button.dimensions.0;
            if width > widest {
                widest = width;
            }
        }

        for button in self.buttons.iter_mut() {
            button.resize((widest, button.dimensions.1));
        }
    }

    /// Adds an external draw call to be executed before the GuiHandler itself is drawn. This fixes
    /// an issue where things that would be drawn external of the GuiHandler have to be drawn over
    /// the GuiHandler.
    pub fn add_external_draw(&mut self, external_draw: Box<DrawAction>) -> &mut Self {
        self.additional_draws.push(external_draw);

        self
    }

    /// Sets the function that will be called everytime any button is pressed.
    ///
    /// Example function:
    /// ```
    /// fn change_state(state: &mut GameStates, state_str: &str) {
    ///     match state_str {
    ///         "menu" => *state = GameStates::Menu,
    ///         "paused" => *state = GameStates::Paused,
    ///         "quit" => *state = GameStates::Quitting,
    ///         "play" => *state = GameStates::Playing,
    ///         "play_reset" => *state = GameStates::Resetting,
    ///         _ => {}
    ///     }
    /// }
    ///
    /// ```
    /// Your code:
    /// ```
    /// use ptgui::prelude::*;
    ///
    /// let mut g_handler = GuiHandler::new(Colour::WHITE);
    /// g_handler.set_button_action_function(change_state);
    /// ```
    pub fn set_button_action_function(&mut self, function: Box<Action<T>>) -> &mut Self {
        self.has_set_button_action = true;
        self.button_action = function;

        self
    }

    /// Makes it so that when buttons are drawn, that they are all drawn at the same width so that
    /// they are uniform.
    pub fn set_button_fix_widths(&mut self, value: bool) -> &mut Self {
        self.button_fixed_width = value;

        self
    }

    /// Adds a button to the `GuiHandler` with a given `position`.
    pub fn add_button_with_position(
        &mut self,
        text: &str,
        action: &str,
        position: Point,
    ) -> &mut Self {
        self.buttons.push(Button::new(text, action, 20, position));

        self
    }

    /// Adds a button to the `GuiHandler` with automatic positioning. It's automatic position is
    /// determined by whether or not there are buttons already added. For example, if no buttons
    /// are present then the first button is placed at (0, 0). If a button already exists then
    /// the buttons created afterwards are placed n+50 pixels below the first button.
    pub fn add_button(&mut self, text: &str, action: &str) -> &mut Self {
        if !self.buttons.is_empty() {
            self.buttons.push(Button::new(
                text,
                action,
                20,
                (
                    self.buttons[self.buttons.len() - 1].position.0,
                    self.buttons[self.buttons.len() - 1].position.1 + self.buttons[0].dimensions.1,
                ),
            ));
        } else {
            self.buttons.push(Button::new(text, action, 20, (0, 0)));
        }

        self
    }

    /// Executes the actions of the buttons
    pub fn execute_actions(&mut self, state: &mut T) -> &mut Self {
        for action in self.actions.iter() {
            (self.button_action)(state, action.as_str());
        }

        self
    }

    /// Draws the `GuiHandler` to the screen.
    pub fn draw<'a>(
        &mut self,
        rl_handler: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Result<RaylibDrawHandle<'a>, &str> {
        let mut draw_handler = rl_handler.begin_drawing(&rl_thread);
        if !self.actions.is_empty() {
            self.actions = vec![];
        }

        let mouse_position = (draw_handler.get_mouse_x(), draw_handler.get_mouse_y());

        if !self.can_draw.get() {
            return Err("Cannot draw. Draw handler was released.");
        }

        if !self.has_set_button_action {
            return Err("Cannot draw. Actions function for buttons has not been set.");
        }

        if self.button_fixed_width {
            self.button_fix_widths();
        }

        draw_handler.clear_background(self.clear_colour);

        for draw_action in self.additional_draws.iter_mut() {
            (draw_action)(&mut draw_handler);
        }

        for button in self.buttons.iter_mut() {
            button.draw(&mut draw_handler);
            self.actions.push(button.is_clicked(
                mouse_position,
                draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
            ));
        }

        // SAFETY: makes sure that the draw_handler is returned to the correct scope.
        Ok(unsafe {
            std::mem::transmute::<RaylibDrawHandle<'_>, RaylibDrawHandle<'a>>(draw_handler)
        })
    }
}
