#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

#[path = "modules/rormod.rs"]
mod rormod;

#[path = "modules/terminal.rs"]
mod terminal;

#[path = "modules/document.rs"]
mod document;

#[path = "modules/row.rs"]
mod row;

#[path = "modules/highlighting.rs"]
mod highlighting;

#[path = "modules/filetype.rs"]
mod filetype;

use rormod::Editor;
pub use rormod::SearchDirection;
pub use terminal::Terminal;
pub use rormod::Position;
pub use document::Document;
pub use row::Row;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;

fn main() {
    Editor::default().run();
}
