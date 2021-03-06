use crate::{
    common::*,
    gui_component::GuiComponentBehaviour,
    prelude::{state_get_colour, Colour, Dimensions, Point, StateColour},
};
use raylib::prelude::{text::measure_text, *};
use std::iter::FromIterator;

#[derive(PartialEq)]
pub struct Label {
    colour: Colour,
    font_size: i32,
    pub dimensions: Dimensions,
    pub position: Point,
    text: String,
}

impl Label {
    /// Create a new button, automatically figuring out width depending on the
    /// `text` and the given `font_size`.
    pub fn new(text: &str, font_size: i32, position: Point) -> Self {
        let dimensions = (measure_text(text, font_size) + 10, 50);
        Self {
            text: text.to_string(),
            font_size,
            position,
            dimensions,
            colour: state_get_colour(StateColour::Active),
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
            colour: state_get_colour(StateColour::Active),
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

impl GuiComponentBehaviour<()> for Label {
    /// Draw `Label` to screen.
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

        draw_handler.draw_line_ex(
            Vector2::new(
                self.position.0 as f32,
                (self.position.1 + self.dimensions.1) as f32,
            ),
            Vector2::new(
                (self.position.0 + self.dimensions.0 + 10) as f32,
                (self.position.1 + self.dimensions.1) as f32,
            ),
            3.5,
            Colour::BLACK,
        );

        draw_handler.draw_text(
            self.text.as_str(),
            self.position.0 + 10,
            self.position.1 + self.dimensions.1 - 30,
            self.font_size,
            state_get_colour(StateColour::Text),
        );
    }

    /// Would check whether or not the mouse is hovering over the label, instead
    /// always return `false` as nothing should happen when the label is
    /// being hovered over.
    fn is_hovered(&mut self, _mouse_position: Point) -> bool { false }

    /// Would check whether or not the user is clicking on the label, however
    /// since the label requires no funcionality when being clicked, the
    /// function always implicitly returns `()`.
    fn is_clicked(&mut self, _mouse_position: Point, _is_clicked: bool) {}
}

impl FromIterator<DrawableType> for Vec<Label> {
    fn from_iter<T: IntoIterator<Item = DrawableType>>(iter: T) -> Self {
        let mut c = Vec::new();

        for i in iter {
            if let DrawableType::Label(s) = i {
                c.push(s)
            }
        }

        c
    }
}
