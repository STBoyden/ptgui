use raylib::prelude::Color;

pub type Point = (i32, i32);
pub type Dimensions = (i32, i32);
pub type Colour = Color;
pub type Action<T> = fn(&mut T, &str);
