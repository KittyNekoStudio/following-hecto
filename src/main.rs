#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
use editor::Editor;

mod editor;
fn main() {
    Editor::default().run();
}
