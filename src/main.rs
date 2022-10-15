#![warn(clippy::all, clippy::pedantic)]

#[path = "modules/rorlib.rs"]
mod rorlib;

#[path = "modules/terminal.rs"]
mod terminal;

#[path = "modules/document.rs"]
mod document;

#[path = "modules/row.rs"]
mod row;

use std::io::{self, stdout};
use rorlib::Editor;
pub use terminal::Terminal;
pub use rorlib::Position;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run();
}
