use crate::prelude::{
    is_inside, state_get_colour, Colour, Dimensions, GuiComponentBehaviour, Point, StateColour,
};
use raylib::prelude::text::measure_text;
use raylib::prelude::*;

pub struct Button {
    text: String,
    font_size: i32,
    position: Point,
    pub dimensions: Dimensions,
    colour: Colour,
}

impl Button {
    /// Create a new button, automatically figuring out width depending on the `text`
    /// and the given `font_size`.
    pub fn new(text: &str, font_size: i32, position: Point) -> Self {
        let dimensions = (measure_text(text, font_size) + 10, 50);
        Self {
            text: text.to_string(),
            font_size,
            position,
            dimensions,
            colour: state_get_colour(StateColour::Default),
        }
    }

    /// Create a new button with specific `dimensions`.
    pub fn new_with_dimensions(
        text: &str,
        font_size: i32,
        position: Point,
        dimensions: Dimensions,
    ) -> Self {
        Self {
            text: text.to_string(),
            font_size,
            position,
            dimensions,
            colour: state_get_colour(StateColour::Default),
        }
    }

    /// Set the text of a button, automatically resizing it's width to compensate
    /// for the change.
    pub fn set_text(&mut self, text: &str) -> &Self {
        let new_width = measure_text(text, self.font_size);
        self.resize((new_width, self.dimensions.1));

        &*self
    }

    /// Resizes the button to the given `new_dimensions`.
    pub fn resize(&mut self, new_dimensions: Dimensions) {
        self.dimensions = new_dimensions;
    }
}

impl GuiComponentBehaviour for Button {
    /// Draw `Button` to screen.
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        let mouse_position = (draw_handler.get_mouse_x(), draw_handler.get_mouse_y());
        self.is_hovered(mouse_position);

        draw_handler.draw_rectangle(
            self.position.0,
            self.position.1,
            self.dimensions.0 + 10,
            self.dimensions.1,
            self.colour,
        );

        draw_handler.draw_text(
            self.text.as_str(),
            self.position.0 + 10,
            self.position.1 + self.dimensions.1 - 30,
            self.font_size,
            state_get_colour(StateColour::Text),
        );
    }

    /// Checks whether cursor is hovering over button, changes colour and returns `true` or `false`
    /// depending on the result.
    fn is_hovered(&mut self, mouse_position: Point) -> bool {
        match is_inside(self.position, self.dimensions, mouse_position) {
            true => {
                self.colour = state_get_colour(StateColour::Hovered);
                true
            }
            false => {
                self.colour = state_get_colour(StateColour::Default);
                false
            }
        }
    }
}
