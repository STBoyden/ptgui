use crate::{
    common::*,
    gui_component::GuiComponentBehaviour,
    prelude::{state_get_colour, Colour, Dimensions, Point, StateColour},
};
use raylib::prelude::{text::measure_text, *};
use std::iter::FromIterator;

#[derive(PartialEq)]
pub struct Button {
    action_string: String,
    colour: Colour,
    font_size: i32,
    pub dimensions: Dimensions,
    pub position: Point,
    text: String,
}

impl Button {
    /// Create a new button, automatically figuring out width depending on the
    /// `text` and the given `font_size`.
    pub fn new(text: &str, action_string: &str, font_size: i32, position: Point) -> Self {
        let dimensions = (measure_text(text, font_size) + 10, 50);
        Self {
            text: text.to_string(),
            action_string: action_string.to_string(),
            font_size,
            position,
            dimensions,
            colour: state_get_colour(StateColour::Default),
        }
    }

    /// Create a new button with specific `dimensions`.
    pub fn new_with_dimensions(
        text: &str,
        action_string: &str,
        font_size: i32,
        position: Point,
        dimensions: Dimensions,
    ) -> Self {
        Self {
            text: text.to_string(),
            action_string: action_string.to_string(),
            font_size,
            position,
            dimensions,
            colour: state_get_colour(StateColour::Default),
        }
    }

    /// Set the text of a button, automatically resizing it's width to
    /// compensate for the change.
    pub fn set_text(&mut self, text: &str) -> &Self {
        let new_width = measure_text(text, self.font_size);
        self.resize((new_width, self.dimensions.1));

        &*self
    }

    /// Resizes the button to the given `new_dimensions`.
    pub fn resize(&mut self, new_dimensions: Dimensions) { self.dimensions = new_dimensions; }
}

impl GuiComponentBehaviour<String> for Button {
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

    /// Checks whether cursor is hovering over button, changes colour and
    /// returns `true` or `false` depending on the result.
    fn is_hovered(&mut self, mouse_position: Point) -> bool {
        if is_inside(self.position, self.dimensions, mouse_position) {
            self.colour = state_get_colour(StateColour::Hovered);
            true
        } else {
            self.colour = state_get_colour(StateColour::Default);
            false
        }
    }

    /// Checks whether or not the user is clicking on the button.
    fn is_clicked(&mut self, mouse_position: Point, is_clicked: bool) -> String {
        if is_inside(self.position, self.dimensions, mouse_position) && is_clicked {
            self.action_string.to_string()
        } else {
            "".to_string()
        }
    }
}

impl FromIterator<DrawableType> for Vec<Button> {
    fn from_iter<T: IntoIterator<Item = DrawableType>>(iter: T) -> Self {
        let mut c = Vec::new();

        for i in iter {
            if let DrawableType::Button(s) = i {
                c.push(s)
            }
        }

        c
    }
}
