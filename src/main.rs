#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
use editor::Editor;

mod editor;
#[allow(arithmetic_overflow)]
fn main() {
    Editor::default().run();
}
