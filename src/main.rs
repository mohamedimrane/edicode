use editor::Editor;
use std::io;
use termion::raw::IntoRawMode;

mod cursor;
mod editor;
mod file;
mod terminal_utils;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    Editor::default().run();
}
