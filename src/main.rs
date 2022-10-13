#![warn(clippy::all, clippy::pedantic)]

mod rorlib;

use std::io::{self, stdout};
use rorlib::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
