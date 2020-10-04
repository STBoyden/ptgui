use crate::prelude::*;

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

/// Allows for the various components in `ptgui` to be contained within a single collection rather
/// than having seperate collections for each component that can be drawn by the `GuiHandler<T>`.
#[derive(PartialEq)]
pub enum DrawableType {
    Button(Button),
    Slider(Slider),
    Dropdown(Dropdown),
}

impl DrawableType {
    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        match self {
            DrawableType::Button(b) => b.draw(draw_handler),
            DrawableType::Slider(s) => s.draw(draw_handler),
            DrawableType::Dropdown(d) => d.draw(draw_handler),
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
            }
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            DrawableType::Button(b) => b.position,
            DrawableType::Slider(s) => s.position,
            DrawableType::Dropdown(d) => d.position,
        }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        match self {
            DrawableType::Button(b) => b.dimensions,
            DrawableType::Slider(s) => s.dimensions,
            DrawableType::Dropdown(d) => d.dimensions,
        }
    }

    pub fn resize(&mut self, new_dimensions: Dimensions) {
        match self {
            DrawableType::Button(b) => b.resize(new_dimensions),
            DrawableType::Slider(s) => s.resize(new_dimensions),
            DrawableType::Dropdown(d) => d.resize(new_dimensions),
        }
    }
}
