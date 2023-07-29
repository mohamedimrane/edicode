use crate::cursor::Position;
use termion::{clear::*, color::*, cursor::*};

pub fn hide_cursor() {
    print!("{}", Hide);
}

pub fn show_cursor() {
    print!("{}", Show);
}

pub fn set_cursor_position(pos: &Position) {
    print!(
        "{}",
        Goto(
            pos.x.saturating_add(1) as u16,
            pos.y.saturating_add(1) as u16
        )
    );
}

pub fn clear() {
    print!("{}", All);
}

pub fn clear_line() {
    print!("{}", CurrentLine);
}

pub fn set_bg_color(color: Rgb) {
    print!("{}", Bg(color));
}

pub fn reset_bg_color() {
    print!("{}", Bg(Reset));
}

pub fn set_fg_color(color: Rgb) {
    print!("{}", Fg(color));
}

pub fn reset_fg_color() {
    print!("{}", Fg(Reset));
}

pub fn color_bg(string: impl std::fmt::Display, color: impl Color) -> String {
    format!("{}{}{}", Bg(color), string, Bg(Reset))
}

pub fn color_fg(string: impl std::fmt::Display, color: impl Color) -> String {
    format!("{}{}{}", Fg(color), string, Fg(Reset))
}
