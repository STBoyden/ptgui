use crate::common::*;
use crate::gui_component::*;
use crate::prelude::*;
use raylib::prelude::*;

#[derive(PartialEq)]
pub struct Dropdown {
    background_colour: Colour,
    components: Vec<DrawableType>,
    components_fixed_widths: bool,
    font_size: i32,
    pub actions: Vec<String>,
    pub dimensions: Dimensions,
    pub position: Point,
    show: bool,
    text: String,
    text_colour: Colour,
}

impl Dropdown {
    /// Create a new `Dropdown`, automatically figuring out width depending on the `text` and the
    /// given `font_size`.
    pub fn new(text: &str, font_size: i32, position: Point) -> Self {
        let dimensions = (measure_text(text, font_size) + 10, 50);
        Self {
            actions: Vec::new(),
            background_colour: state_get_colour(StateColour::Default),
            components: Vec::new(),
            components_fixed_widths: false,
            dimensions,
            font_size,
            position,
            show: false,
            text: text.to_string(),
            text_colour: state_get_colour(StateColour::Text),
        }
    }

    /// Add a new `DrawableType` component to the list of components to be drawn.
    pub fn add_component(&mut self, component: DrawableType) -> &mut Self {
        self.components.push(component);

        self
    }

    fn get_previous_position(&self) -> Point {
        match self.components.last() {
            Some(DrawableType::Button(b)) => b.position,
            Some(DrawableType::Slider(s)) => s.position,
            Some(DrawableType::Dropdown(d)) => d.position,
            None => self.position,
        }
    }

    /// Makes it so that when components are drawn, that they are all drawn at the same width so that
    /// they are uniform.
    pub fn set_components_fix_widths(&mut self, value: bool) -> &mut Self {
        self.components_fixed_widths = value;

        self
    }

    fn components_fix_widths(&mut self) {
        let mut widest = -1;

        for component in self.components.iter() {
            let width = match component {
                DrawableType::Button(b) => b.dimensions.0,
                DrawableType::Slider(s) => s.dimensions.0,
                DrawableType::Dropdown(d) => d.dimensions.0,
            };

            if width > widest {
                widest = width;
            }
        }

        for component in self.components.iter_mut() {
            match component {
                DrawableType::Button(b) => b.resize((widest, b.dimensions.1)),
                DrawableType::Slider(s) => s.resize((widest, s.dimensions.1)),
                DrawableType::Dropdown(d) => d.resize((widest, d.dimensions.1)),
            }
        }
    }

    /// Resizes the `Dropdown` to given `new_dimensions`.
    pub fn resize(&mut self, new_dimensions: Dimensions) {
        self.dimensions = (new_dimensions.0 - 10, new_dimensions.1);
        let new_x_pos = self.position.0 + 10 + self.dimensions.0;

        for component in self.components.iter_mut() {
            match component {
                DrawableType::Button(b) => {
                    if b.position.0 != new_x_pos {
                        b.position.0 = new_x_pos;
                    }
                }
                DrawableType::Slider(s) => {
                    if s.position.0 != new_x_pos {
                        s.move_x(new_x_pos);
                    }
                }
                DrawableType::Dropdown(d) => {
                    if d.position.0 != new_x_pos {
                        d.position.0 = new_x_pos;
                    }
                }
            }
        }
    }

    /// Adds a `Button` to the `Dropdown` with automatic positioning. It's automatic position is
    /// determined by whether or not there are components already added. For example, if no components
    /// are present then the first `Button` is placed at (0, 0). If a component already exists then
    /// the `Button`s created afterwards are placed n+50 pixels below the first component.
    pub fn add_button(&mut self, text: &str, action: &str) -> &mut Self {
        let previous_position = self.get_previous_position();
        let new_x = self.position.0 + self.dimensions.0;

        self.components.push(DrawableType::Button(Button::new(
            text,
            action,
            20,
            (
                previous_position.0 + new_x,
                previous_position.1
                    + match self.components.first() {
                        Some(c) => match c {
                            DrawableType::Button(b) => b.dimensions.1,
                            DrawableType::Slider(s) => s.dimensions.1,
                            DrawableType::Dropdown(d) => d.dimensions.1,
                        },
                        None => 0,
                    },
            ),
        )));

        self
    }

    /// Adds a `Slider` to the `Dropdown` with automatic positioning. It's automatic position is
    /// determined by whether or not there are components already added. For example, if no components
    /// are present then the first `Slider` is placed at (0, 0). If a component already exists then
    /// the `Slider`s created afterwards are placed n+50 pixels below the first component.
    pub fn add_slider(&mut self, min: i32, max: i32, initial_value: f32) -> &mut Self {
        let previous_position = self.get_previous_position();
        let new_x = self.position.0 + self.dimensions.0;

        self.components.push(DrawableType::Slider(Slider::new(
            min,
            max,
            initial_value,
            (
                previous_position.0 + new_x,
                previous_position.1
                    + match self.components.first() {
                        Some(c) => match c {
                            DrawableType::Button(b) => b.dimensions.1,
                            DrawableType::Slider(s) => s.dimensions.1,
                            DrawableType::Dropdown(d) => d.dimensions.1,
                        },
                        None => 0,
                    },
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

    /// Adds a `Dropdown` to the `Dropdown` with automatic positioning. It's automatic position is
    /// determined by whether or not there are components already added. For example, if no components
    /// are present then the first `Dropdown` is placed at (0, 0). If a component already exists then
    /// the `Dropdown`s created afterwards are placed n+50 pixels below the first component.
    pub fn add_dropdown(&mut self, text: &str) -> &mut Self {
        let previous_position = self.get_previous_position();
        let new_x = self.position.0 + self.dimensions.0;

        self.components.push(DrawableType::Dropdown(Dropdown::new(
            text,
            20,
            (
                previous_position.0 + new_x,
                previous_position.1
                    + match self.components.first() {
                        Some(c) => match c {
                            DrawableType::Button(b) => b.dimensions.1,
                            DrawableType::Slider(s) => s.dimensions.1,
                            DrawableType::Dropdown(d) => d.dimensions.1,
                        },
                        None => 0,
                    },
            ),
        )));

        self
    }

    /// Gets a vector of mutable `Dropdown` references in the components vector,
    pub fn get_dropdowns_mut(&mut self) -> Result<Vec<&mut Dropdown>, String> {
        let mut dropdown = vec![];
        for c in self.components.iter_mut() {
            if let DrawableType::Dropdown(d) = c {
                dropdown.push(d)
            }
        }

        Ok(dropdown)
    }

    /// Gets a vector of `Dropdown` references in the components vector,
    pub fn get_dropdowns(&mut self) -> Result<Vec<&Dropdown>, String> {
        let mut dropdown = vec![];
        for c in self.components.iter() {
            if let DrawableType::Dropdown(d) = c {
                dropdown.push(d)
            }
        }

        Ok(dropdown)
    }
}

impl GuiComponentBehaviour<()> for Dropdown {
    /// Draw `Dropdown` to screen.
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        let mouse_position = (draw_handler.get_mouse_x(), draw_handler.get_mouse_y());
        self.is_hovered(mouse_position);

        self.actions.clear();

        draw_handler.draw_rectangle(
            self.position.0,
            self.position.1,
            self.dimensions.0 + 10,
            self.dimensions.1,
            self.background_colour,
        );

        draw_handler.draw_text(
            self.text.as_str(),
            self.position.0 + 10,
            self.position.1 + self.dimensions.1 - 30,
            self.font_size,
            self.text_colour,
        );

        if self.components_fixed_widths {
            self.components_fix_widths();
        }

        if self.show {
            for component in self.components.iter_mut() {
                match component {
                    DrawableType::Button(b) => {
                        b.draw(draw_handler);
                        self.actions.push(b.is_clicked(
                            mouse_position,
                            draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                        ));
                    }
                    DrawableType::Slider(s) => {
                        s.draw(draw_handler);
                        s.is_clicked(
                            mouse_position,
                            draw_handler.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
                        );
                    }
                    DrawableType::Dropdown(d) => {
                        d.draw(draw_handler);
                        d.is_clicked(
                            mouse_position,
                            draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                        );
                    }
                }
            }
        }
    }

    /// Checks whether or not the cursor is hovering over the `Dropdown` and returns `true` or
    /// `false`.
    fn is_hovered(&mut self, mouse_position: Point) -> bool {
        if is_inside(self.position, self.dimensions, mouse_position) {
            self.background_colour = state_get_colour(StateColour::Hovered);
            true
        } else {
            if !self.show {
                self.background_colour = state_get_colour(StateColour::Default);
            }
            false
        }
    }

    /// Checks whether or not the user is clicking on the `Dropdown`.
    fn is_clicked(&mut self, mouse_position: Point, is_clicked: bool) {
        if is_inside(self.position, self.dimensions, mouse_position) && is_clicked {
            self.background_colour = state_get_colour(StateColour::Active);
            self.show = !self.show;
        }
    }
}
