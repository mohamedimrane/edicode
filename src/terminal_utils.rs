pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn set_cursor_position(x: u16, y: u16) {
    print!("{}", termion::cursor::Goto(x + 1, y + 1));
}

pub fn clear() {
    print!("{}", termion::clear::All);
}

pub fn clear_line() {
    print!("{}", termion::clear::CurrentLine);
}
