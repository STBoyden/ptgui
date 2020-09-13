use crate::gui_component::*;
use crate::prelude::*;
use raylib::prelude::*;
use std::cell::Cell;

/// The default `struct` to handle the GUI system implemented by the `ptgui` crate.
pub struct GuiHandler<T> {
    can_draw: Cell<bool>,
    clear_colour: Colour,
    component_fixed_width: bool,
    components: Vec<Box<dyn InternalDrawable>>,
    button_action: Action<T>,
    actions: Vec<String>,
    additional_draws: Vec<Box<dyn Drawable>>,
    has_set_button_action: bool,
}

impl<T> GuiHandler<T> {
    /// Creates a new `GuiHandler<T>`.
    pub fn new(clear_colour: Colour) -> GuiHandler<T> {
        GuiHandler {
            can_draw: Cell::new(true),
            clear_colour,
            component_fixed_width: false,
            components: Vec::new(),
            button_action: |_, _| {},
            actions: Vec::new(),
            additional_draws: Vec::new(),
            has_set_button_action: false,
        }
    }

    fn components_fix_widths(&mut self) {
        let mut widest = -1;

        for component in self.components.iter() {
            let width = component.get_dimensions().0;
            if width > widest {
                widest = width;
            }
        }

        for component in self.components.iter_mut() {
            component.resize((widest, component.get_dimensions().1));
        }
    }

    /// Clears the external draw call vector.
    pub fn clear_external_draws(&mut self) -> &mut Self {
        self.additional_draws.clear();

        self
    }

    /// Adds an external draw call to be executed before the GuiHandler itself is drawn. This fixes
    /// an issue where things that would be drawn external of the GuiHandler have to be drawn over
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
    /// use ptgui::prelude::*;
    /// fn main() {
    ///     // ...
    ///     let mut g_handler = GuiHandler::new(Colour::WHITE);
    ///     g_handler.set_button_action_function(change_state);
    /// }
    ///
    /// ```
    pub fn set_button_action_function(&mut self, function: Action<T>) -> &mut Self {
        self.has_set_button_action = true;
        self.button_action = function;

        self
    }

    /// Makes it so that when components are drawn, that they are all drawn at the same width so that
    /// they are uniform.
    pub fn set_component_fix_widths(&mut self, value: bool) -> &mut Self {
        self.component_fixed_width = value;

        self
    }

    /// Adds a button to the `GuiHandler` with a given `position`.
    pub fn add_button_with_position(
        &mut self,
        text: &str,
        action: &str,
        position: Point,
    ) -> &mut Self {
        self.components
            .push(Box::new(Button::new(text, action, 20, position)));

        self
    }

    /// Adds a button to the `GuiHandler` with automatic positioning. It's automatic position is
    /// determined by whether or not there are buttons already added. For example, if no buttons
    /// are present then the first button is placed at (0, 0). If a button already exists then
    /// the buttons created afterwards are placed n+50 pixels below the first button.
    pub fn add_button(&mut self, text: &str, action: &str) -> &mut Self {
        if !self.components.is_empty() {
            self.components.push(Box::new(Button::new(
                text,
                action,
                20,
                (
                    self.components[self.components.len() - 1].get_position().0,
                    self.components[self.components.len() - 1].get_position().1
                        + self.components[0].get_dimensions().1,
                ),
            )));
        } else {
            self.components
                .push(Box::new(Button::new(text, action, 20, (0, 0))));
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

    /// Adds a `Slider` to the `GuiHandler` with a given `position`.
    pub fn add_slider_with_position(
        &mut self,
        min: i32,
        max: i32,
        initial_value: f32,
        position: Point,
    ) -> &mut Self {
        self.components.push(Box::new(Slider::new(
            min,
            max,
            initial_value,
            position,
            100,
        )));

        self
    }

    /// Adds a slider to the `GuiHandler` with automatic positioning. It's automatic position is
    /// determined by whether or not there are sliders already added. For example, if no sliders
    /// are present then the first slider is placed at (0, 0). If a slider already exists then
    /// the sliders created afterwards are placed n+30 pixels below the first slider.
    pub fn add_slider(&mut self, min: i32, max: i32, initial_value: f32) -> &mut Self {
        if !self.components.is_empty() {
            self.components.push(Box::new(Slider::new(
                min,
                max,
                initial_value,
                (
                    self.components[self.components.len() - 1].get_position().0,
                    self.components[self.components.len() - 1].get_position().1
                        + self.components[self.components.len() - 1]
                            .get_dimensions()
                            .1, // + 10,
                ),
                250,
            )));
        } else {
            self.components
                .push(Box::new(Slider::new(min, max, initial_value, (0, 0), 250)));
        }

        self
    }

    /// Gets the value of a specified `Slider` via an index, returning a `f32`.
    pub fn get_slider_value<'a>(&self, index: usize) -> Result<f32, String> {
        let mut sliders = vec![];

        self.components.iter().for_each(|c| {
            if c.get_type() == DrawableType::Slider {
                sliders.push(unsafe {
                    std::mem::transmute::<&Box<dyn InternalDrawable>, &Box<Slider>>(c)
                });
            }
        });

        if index > sliders.len() {
            return Err(
                "Cannot return the value of a Slider that is out of index range".to_string(),
            );
        }

        Ok(sliders[index].get_value())
    }

    /// Gets the value of a specified `Slider` via an index, returning an `i32`.
    pub fn get_slider_value_i32<'a>(&self, index: usize) -> Result<i32, String> {
        Ok(self.get_slider_value(index).ok().unwrap() as i32)
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

        let mut has_buttons = false;
        self.components.iter().for_each(|c| {
            if c.get_type() == DrawableType::Button {
                has_buttons = true
            }
        });

        if !self.has_set_button_action && has_buttons {
            return Err("Cannot draw. Actions function for buttons has not been set.");
        }

        if self.component_fixed_width {
            self.components_fix_widths();
        }

        draw_handler.clear_background(self.clear_colour);

        for drawable in self.additional_draws.iter_mut() {
            drawable.draw(&mut draw_handler);
        }

        for component in self.components.iter_mut() {
            // SAFETY: It is made sure that the type of the component is correct before it
            // is transmuted to the appropriate component type.
            //
            // TODO: Find a way to do this in safe Rust. Though I know this is safe, I would prefer
            // a safe Rust alternative.
            unsafe {
                match component.get_type() {
                    DrawableType::Button => {
                        let button = std::mem::transmute::<
                            &mut Box<dyn InternalDrawable>,
                            &mut Box<Button>,
                        >(component);

                        button.draw(&mut draw_handler);
                        self.actions.push(button.is_clicked(
                            mouse_position,
                            draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                        ));
                    }
                    DrawableType::Slider => {
                        let slider = std::mem::transmute::<
                            &mut Box<dyn InternalDrawable>,
                            &mut Box<Slider>,
                        >(component);

                        slider.draw(&mut draw_handler);
                        slider.is_clicked(
                            mouse_position,
                            draw_handler.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
                        );
                    }
                }
            }
        }

        // SAFETY: makes sure that the draw_handler is returned to the correct scope.
        Ok(unsafe {
            std::mem::transmute::<RaylibDrawHandle<'_>, RaylibDrawHandle<'a>>(draw_handler)
        })
    }
}
