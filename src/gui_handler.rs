use crate::prelude::*;
use raylib::prelude::*;
use std::cell::Cell;

pub struct GuiHandler<'a> {
    can_draw: Cell<bool>,
    clear_colour: Colour,
    draw_handler: RaylibDrawHandle<'a>,
    button_fixed_width: bool,
    buttons: Vec<Button>,
}

impl GuiHandler<'_> {
    /// Creates a new `GuiHandler<'_>`.
    pub fn new(draw_handler: RaylibDrawHandle, clear_colour: Colour) -> GuiHandler<'_> {
        GuiHandler {
            can_draw: Cell::new(true),
            clear_colour,
            draw_handler,
            button_fixed_width: false,
            buttons: Vec::new(),
        }
    }

    fn button_fix_widths(&mut self) {
        let mut widest = -1;

        for button in self.buttons.iter() {
            let width = button.dimensions.0;
            if width > widest {
                widest = width;
            }
        }

        for button in self.buttons.iter_mut() {
            button.resize((widest, button.dimensions.1));
        }
    }

    /// Makes it so that when buttons are drawn, that they are all drawn at the same width so that
    /// they are uniform.
    pub fn set_button_fix_widths(&mut self, value: bool) -> &mut Self {
        self.button_fixed_width = value;

        self
    }

    /// Adds a button to the `GuiHandler`.
    pub fn add_button(&mut self, text: &str, position: Point) -> &mut Self {
        self.buttons.push(Button::new(text, 20, position));

        self
    }

    /// Draws the `GuiHandler` to the screen.
    pub fn draw(&mut self) -> Result<(), &str> {
        if !self.can_draw.get() {
            return Err("Cannot draw. Draw handler was released.");
        }

        if self.button_fixed_width {
            self.button_fix_widths();
        }

        self.draw_handler.clear_background(self.clear_colour);
        for button in self.buttons.iter_mut() {
            button.draw(&mut self.draw_handler);
        }

        Ok(())
    }

    /// Releases the `RaylibDrawHandle` from the `GuiHandler` so that other non-ptgui related
    /// things can be drawn to the screen afterwards.
    ///
    // TODO: Make this automatically happen after calling `draw`.
    pub fn release_draw_handle<'a>(self) -> RaylibDrawHandle<'a> {
        self.can_draw.set(false);
        // SAFETY: As can_draw is set to false, any subsequent `draw` call will return an Err
        // making this safe.
        unsafe {
            std::mem::transmute::<RaylibDrawHandle<'_>, RaylibDrawHandle<'a>>(self.draw_handler)
        }
    }
}
