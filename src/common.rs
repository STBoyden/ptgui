use crate::prelude::{Dimensions, Point};

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
