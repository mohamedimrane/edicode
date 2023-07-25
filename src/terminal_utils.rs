use crate::cursor::CursorPosition;

pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn set_cursor_position(pos: &CursorPosition) {
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
