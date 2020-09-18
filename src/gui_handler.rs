use crate::common::*;
use crate::gui_component::*;
use crate::prelude::*;
use raylib::prelude::*;

/// The default `struct` to handle the GUI system implemented by the `ptgui` crate.
pub struct GuiHandler<T> {
    actions: Vec<String>,
    additional_draws: Vec<Box<dyn Drawable>>,
    button_action: Action<T>,
    clear_colour: Colour,
    components_fixed_widths: bool,
    components: Vec<DrawableType>,
    has_set_button_action: bool,
}

impl<T> GuiHandler<T> {
    /// Creates a new `GuiHandler<T>`.
    pub fn new(clear_colour: Colour) -> Self {
        Self {
            actions: Vec::new(),
            additional_draws: Vec::new(),
            button_action: |_, _| {},
            clear_colour,
            components: Vec::new(),
            components_fixed_widths: false,
            has_set_button_action: false,
        }
    }

    fn components_fix_widths(&mut self) {
        let mut widest = -1;

        for component in self.components.iter() {
            let width = match component {
                DrawableType::Button(b) => b.dimensions.0,
                DrawableType::Slider(s) => s.dimensions.0,
            };

            if width > widest {
                widest = width;
            }
        }

        for component in self.components.iter_mut() {
            match component {
                DrawableType::Button(b) => b.resize((widest, b.dimensions.1)),
                DrawableType::Slider(s) => s.resize((widest, s.dimensions.1)),
            }
        }
    }

    /// Clears the external draw call vector.
    pub fn clear_external_draws(&mut self) -> &mut Self {
        self.additional_draws.clear();

        self
    }

    /// Adds an external draw call to be executed before the `GuiHandler<T>` itself is drawn. This fixes
    /// an issue where things that would be drawn external of the `GuiHandler<T>` have to be drawn over
    /// the GuiHandler.
    pub fn add_external_draw(&mut self, external_draw: Box<dyn Drawable>) -> &mut Self {
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
    /// let mut g_handler = GuiHandler::new(Colour::WHITE);
    /// g_handler.set_button_action_function(change_state);
    /// ```
    pub fn set_button_action_function(&mut self, function: Action<T>) -> &mut Self {
        self.has_set_button_action = true;
        self.button_action = function;

        self
    }

    /// Makes it so that when components are drawn, that they are all drawn at the same width so that
    /// they are uniform.
    pub fn set_components_fix_widths(&mut self, value: bool) -> &mut Self {
        self.components_fixed_widths = value;

        self
    }

    /// Adds a button to the `GuiHandler` with a given `position`.
    pub fn add_button_with_position(
        &mut self,
        text: &str,
        action: &str,
        position: Point,
    ) -> &mut Self {
        self.components.push(DrawableType::Button(Button::new(
            text, action, 20, position,
        )));

        self
    }

    /// Adds a `Button` to the `GuiHandler` with automatic positioning. It's automatic position is
    /// determined by whether or not there are components already added. For example, if no components
    /// are present then the first `Button` is placed at (0, 0). If a component already exists then
    /// the `Button`s created afterwards are placed n+50 pixels below the first component.
    pub fn add_button(&mut self, text: &str, action: &str) -> &mut Self {
        let first_dimensions = self.get_first_dimensions();
        let previous_position = self.get_previous_position();

        self.components.push(DrawableType::Button(Button::new(
            text,
            action,
            20,
            (
                previous_position.0,
                previous_position.1 + first_dimensions.1,
            ),
        )));

        self
    }

    /// Executes the actions of the buttons
    pub fn execute_actions(&mut self, state: &mut T) -> &mut Self {
        for action in self.actions.iter() {
            (self.button_action)(state, action.as_str());
        }

        self
    }

    /// Adds a `Slider` to the `GuiHandler` with a given `position`.
    pub fn add_slider_with_position(
        &mut self,
        min: i32,
        max: i32,
        initial_value: f32,
        position: Point,
    ) -> &mut Self {
        self.components.push(DrawableType::Slider(Slider::new(
            min,
            max,
            initial_value,
            position,
            100,
        )));

        self
    }

    /// Adds a `Slider` to the `GuiHandler` with automatic positioning. It's automatic position is
    /// determined by whether or not there are components already added. For example, if no components
    /// are present then the first `Slider` is placed at (0, 0). If a component already exists then
    /// the `Slider`s created afterwards are placed n+50 pixels below the first component.
    pub fn add_slider(&mut self, min: i32, max: i32, initial_value: f32) -> &mut Self {
        let first_dimensions = self.get_first_dimensions();
        let previous_position = self.get_previous_position();

        self.components.push(DrawableType::Slider(Slider::new(
            min,
            max,
            initial_value,
            (
                previous_position.0,
                previous_position.1 + first_dimensions.1,
            ),
            250,
        )));

        self
    }

    /// Gets the value of a specified `Slider` via an index, returning a `f32`.
    pub fn get_slider_value(&self, index: usize) -> Result<f32, String> {
        let mut sliders = vec![];
        for c in self.components.iter() {
            if let DrawableType::Slider(s) = c {
                sliders.push(s)
            }
        }

        if index > sliders.len() {
            return Err(
                "Cannot return the value of a Slider that is out of index range".to_string(),
            );
        }

        Ok(sliders[index].get_value())
    }

    /// Gets the value of a specified `Slider` via an index, returning an `i32`.
    pub fn get_slider_value_i32(&self, index: usize) -> Result<i32, String> {
        match self.get_slider_value(index) {
            Ok(v) => Ok(v as i32),
            Err(e) => Err(e),
        }
    }

    fn get_first_dimensions(&self) -> Dimensions {
        match self.components.get(0) {
            Some(DrawableType::Button(b)) => b.dimensions,
            Some(DrawableType::Slider(s)) => s.dimensions,
            Some(DrawableType::Dropdown(d)) => d.dimensions,
            None => (0, 50),
        }
    }

    fn get_previous_position(&self) -> Point {
        match self.components.last() {
            Some(DrawableType::Button(b)) => b.position,
            Some(DrawableType::Slider(s)) => s.position,
            Some(DrawableType::Dropdown(d)) => d.position,
            None => (0, 0),
        }
    }
    /// Draws the `GuiHandler` to the screen.
    pub fn draw<'a>(
        &mut self,
        rl_handler: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Result<RaylibDrawHandle<'a>, &str> {
        let mut draw_handler = rl_handler.begin_drawing(&rl_thread);

        self.actions.clear();

        let mouse_position = (draw_handler.get_mouse_x(), draw_handler.get_mouse_y());

        let buttons = self
            .components
            .iter()
            .filter(|c| match c {
                DrawableType::Button(_) => true,
                _ => false,
            })
            .count();

        if !self.has_set_button_action && !buttons.eq(&0) {
            return Err("Cannot draw. Actions function for buttons has not been set.");
        }

        if self.components_fixed_widths {
            self.components_fix_widths();
        }

        draw_handler.clear_background(self.clear_colour);

        for drawable in self.additional_draws.iter_mut() {
            drawable.draw(&mut draw_handler);
        }

        for component in self.components.iter_mut() {
            match component {
                DrawableType::Button(b) => {
                    b.draw(&mut draw_handler);
                    self.actions.push(b.is_clicked(
                        mouse_position,
                        draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                    ));
                }
                DrawableType::Slider(s) => {
                    s.draw(&mut draw_handler);
                    s.is_clicked(
                        mouse_position,
                        draw_handler.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
                    );
                }
            }
        }

        // SAFETY: makes sure that the draw_handler is returned to the correct scope.
        Ok(unsafe {
            std::mem::transmute::<RaylibDrawHandle<'_>, RaylibDrawHandle<'a>>(draw_handler)
        })
    }
}
