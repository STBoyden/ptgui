use crate::common::*;
use crate::gui_component::GuiComponentBehaviour;
use crate::prelude::{state_get_colour, Colour, Dimensions, Point, StateColour};
use raylib::prelude::*;
use std::iter::FromIterator;

#[derive(Copy, Clone, PartialEq)]
pub struct Slider {
    background_colour: Colour,
    max: i32,
    min: i32,
    pub dimensions: Dimensions,
    pub position: Point,
    slider_background_colour: Colour,
    slider_box_colour: Colour,
    slider_box_dimensions: Dimensions,
    slider_box_position: Point,
    slider_dimensions: Dimensions,
    slider_position: Point,
    slider_text_colour: Colour,
    value: f32,
}

impl Slider {
    /// Create a new `Slider` with a defined minimum and maximum value.
    pub fn new(min: i32, max: i32, initial_value: f32, position: Point, width: i32) -> Self {
        let mut initial_value = initial_value;

        if min > initial_value as i32 {
            initial_value = min as f32;
        } else if max < initial_value as i32 {
            initial_value = max as f32;
        }

        let mut s = Self {
            max,
            min,
            value: initial_value,
            position,
            dimensions: (width + 120, 50),
            slider_position: (position.0 + 10, position.1 + 7),
            slider_dimensions: (width, 35),
            slider_box_dimensions: (30, 35),
            slider_box_position: (position.0 + 10, position.1 + 7),
            slider_box_colour: state_get_colour(StateColour::Text),
            slider_text_colour: state_get_colour(StateColour::Text),
            slider_background_colour: Colour::LIGHTGRAY,
            background_colour: state_get_colour(StateColour::Default),
        };

        s.set_position_from_value();

        s
    }

    fn set_position_from_value(&mut self) {
        self.slider_box_position.0 = (((self.value - self.min as f32)
            / (self.max - self.min) as f32)
            * ((self.slider_position.0 + self.slider_dimensions.0) - self.slider_position.0) as f32
            + self.slider_position.0 as f32) as i32;
    }

    fn update_value(&mut self) {
        let difference = (self.slider_box_position.0 + self.slider_box_dimensions.0)
            - (self.slider_position.0 + self.slider_box_dimensions.0);

        self.value = (difference as f32
            / (self.slider_dimensions.0 - self.slider_box_dimensions.0) as f32)
            * (self.max - self.min) as f32
            + self.min as f32;
    }

    /// Returns the value of the current `Slider`.
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Resizes the slider to `new_dimensions` and resizes it's components proportionally.
    pub fn resize(&mut self, new_dimensions: Dimensions) {
        self.dimensions = new_dimensions;
        self.slider_dimensions = (new_dimensions.0 - 120, new_dimensions.1 - 15);
        self.slider_box_dimensions = (self.slider_box_dimensions.0, new_dimensions.1 - 15);
    }
}

impl GuiComponentBehaviour<()> for Slider {
    /// Draw `Slider` to screen.
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        let mouse_position = (draw_handler.get_mouse_x(), draw_handler.get_mouse_y());
        self.is_hovered(mouse_position);

        draw_handler.draw_rectangle(
            self.position.0,
            self.position.1,
            self.dimensions.0,
            self.dimensions.1,
            self.background_colour,
        );

        draw_handler.draw_text(
            format!("{:#.2}", self.value).as_str(),
            (self.position.0 + self.dimensions.0) - 105,
            self.position.1 + 10,
            32,
            self.slider_text_colour,
        );

        draw_handler.draw_rectangle(
            self.slider_position.0,
            self.slider_position.1,
            self.slider_dimensions.0,
            self.slider_dimensions.1,
            self.slider_background_colour,
        );

        draw_handler.draw_rectangle(
            self.slider_box_position.0,
            self.slider_box_position.1,
            self.slider_box_dimensions.0,
            self.slider_box_dimensions.1,
            self.slider_box_colour,
        );
    }

    /// Checks if the `Slider` is being hovered over.
    fn is_hovered(&mut self, mouse_position: Point) -> bool {
        if is_inside(
            self.slider_box_position,
            self.slider_box_dimensions,
            mouse_position,
        ) {
            self.slider_box_colour = state_get_colour(StateColour::Hovered);
            true
        } else {
            self.slider_box_colour = state_get_colour(StateColour::Text);
            false
        }
    }

    /// Checks if the `Slider` is clicked.
    fn is_clicked(&mut self, mouse_position: Point, is_clicked: bool) {
        if (is_inside(
            self.slider_box_position,
            self.slider_box_dimensions,
            mouse_position,
        ) || (is_inside(self.position, self.dimensions, mouse_position)))
            && is_clicked
        {
            self.slider_box_position = (
                mouse_position.0 - self.slider_box_dimensions.0 / 2,
                self.slider_box_position.1,
            );

            if (self.slider_box_position.0 + self.slider_box_dimensions.0)
                > self.slider_position.0 + self.slider_dimensions.0
            {
                self.slider_box_position.0 = (self.slider_position.0 + self.slider_dimensions.0)
                    - self.slider_box_dimensions.0;
            } else if self.slider_box_position.0 < self.slider_position.0 {
                self.slider_box_position.0 = self.slider_position.0;
            }

            self.update_value();
        }
    }
}

impl FromIterator<DrawableType> for Vec<Slider> {
    fn from_iter<T: IntoIterator<Item = DrawableType>>(iter: T) -> Self {
        let mut c = Vec::new();

        for i in iter {
            if let DrawableType::Slider(s) = i {
                c.push(s)
            }
        }

        c
    }
}
