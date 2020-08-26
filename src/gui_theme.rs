use crate::prelude::Colour;

pub enum StateColour {
    Hovered,
    Active,
    Default,
    Text,
}

pub fn state_get_colour(state: StateColour) -> Colour {
    match state {
        StateColour::Hovered | StateColour::Active => Colour::DARKGRAY,
        StateColour::Default => Colour::GRAY,
        StateColour::Text => Colour::RAYWHITE,
    }
}
