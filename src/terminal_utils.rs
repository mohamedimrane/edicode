use crate::cursor::Position;

pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn set_cursor_position(pos: &Position) {
    print!(
        "{}",
        termion::cursor::Goto(
            pos.x.saturating_add(1) as u16,
            pos.y.saturating_add(1) as u16
        )
    );
}

pub fn clear() {
    print!("{}", termion::clear::All);
}

pub fn clear_line() {
    print!("{}", termion::clear::CurrentLine);
}

pub fn set_bg_color(color: termion::color::Rgb) {
    print!("{}", termion::color::Bg(color));
}

pub fn reset_bg_color() {
    print!("{}", termion::color::Bg(termion::color::Reset));
}

pub fn set_fg_color(color: termion::color::Rgb) {
    print!("{}", termion::color::Fg(color));
}

pub fn reset_fg_color() {
    print!("{}", termion::color::Fg(termion::color::Reset));
}
