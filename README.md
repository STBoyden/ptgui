# PTGUI - A GUI library for Project Triangle.

This is a library designed for use with my [Project Triangle repository](https://github.com/STBoyden/project-triangle).


This library uses Raylib-rs to draw buttons and other GUI elements to the screen. It is also not inteded as an alternative
to `rgui` as `rgui` is intended to be used in the same places you would use ImGUI whereas this library would be used in
a Raylib Rust-based games.

## Examples (`samples/`)
Button sample: `cargo run --release --bin button-sample`
Button sample with external draws: `cargo run --release --bin button-sample-additional-draw`
Slider sample: `cargo run --release --bin slider-sample`
