use crate::{gui_component::*, prelude::*};
use raylib::prelude::*;

pub fn is_inside(position: Point, dimensions: Dimensions, mouse_position: Point) -> bool {
    let rect_points = [
        position,
        (position.0 + dimensions.0, position.1),
        (position.0, position.1 + dimensions.1),
        (position.0 + dimensions.0, position.1 + dimensions.1),
    ];

    (mouse_position.0 > rect_points[0].0 && mouse_position.0 > rect_points[2].0)
        && (mouse_position.0 < rect_points[1].0 && mouse_position.0 < rect_points[3].0)
        && (mouse_position.1 > rect_points[0].1 && mouse_position.1 > rect_points[1].1)
        && (mouse_position.1 < rect_points[2].1 && mouse_position.1 < rect_points[3].1)
}

/// Allows for the various components in `ptgui` to be contained within a single
/// collection rather than having seperate collections for each component that
/// can be drawn by the `GuiHandler<T>`.
#[derive(PartialEq)]
pub enum DrawableType {
    Button(Button),
    Slider(Slider),
    Dropdown(Dropdown),
    Label(Label),
}

impl DrawableType {
    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        match self {
            DrawableType::Button(b) => b.draw(draw_handler),
            DrawableType::Slider(s) => s.draw(draw_handler),
            DrawableType::Dropdown(d) => d.draw(draw_handler),
            DrawableType::Label(l) => l.draw(draw_handler),
        }
    }

    pub fn is_clicked(
        &mut self,
        mouse_position: Point,
        actions: &mut Vec<String>,
        draw_handler: &RaylibDrawHandle,
    ) {
        match self {
            DrawableType::Button(b) => actions.push(b.is_clicked(
                mouse_position,
                draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
            )),
            DrawableType::Slider(s) => s.is_clicked(
                mouse_position,
                draw_handler.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON),
            ),
            DrawableType::Dropdown(d) => {
                d.is_clicked(
                    mouse_position,
                    draw_handler.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON),
                );
                actions.append(&mut d.actions)
            },
            _ => (),
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            DrawableType::Button(b) => b.position,
            DrawableType::Slider(s) => s.position,
            DrawableType::Dropdown(d) => d.position,
            DrawableType::Label(l) => l.position,
        }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        match self {
            DrawableType::Button(b) => b.dimensions,
            DrawableType::Slider(s) => s.dimensions,
            DrawableType::Dropdown(d) => d.dimensions,
            DrawableType::Label(l) => l.dimensions,
        }
    }

    pub fn resize(&mut self, new_dimensions: Dimensions) {
        match self {
            DrawableType::Button(b) => b.resize(new_dimensions),
            DrawableType::Slider(s) => s.resize(new_dimensions),
            DrawableType::Dropdown(d) => d.resize(new_dimensions),
            DrawableType::Label(l) => l.resize(new_dimensions),
        }
    }

    pub fn move_x(&mut self, new_x_pos: i32) {
        match self {
            DrawableType::Button(b) =>
                if b.position.0 != new_x_pos {
                    b.position.0 = new_x_pos;
                },
            DrawableType::Slider(s) =>
                if s.position.0 != new_x_pos {
                    s.move_x(new_x_pos);
                },
            DrawableType::Dropdown(d) =>
                if d.position.0 != new_x_pos {
                    d.position.0 = new_x_pos;
                },
            DrawableType::Label(l) =>
                if l.position.0 != new_x_pos {
                    l.position.0 = new_x_pos;
                },
        }
    }
}
