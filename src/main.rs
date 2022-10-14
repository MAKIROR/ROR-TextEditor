#![warn(clippy::all, clippy::pedantic)]

mod rorlib;
mod terminal;

use std::io::{self, stdout};
use rorlib::Editor;
pub use terminal::Terminal;
pub use rorlib::Position;

fn main() {
    Editor::default().run();
}
