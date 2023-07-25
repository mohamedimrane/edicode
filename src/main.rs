use cursor::CursorPosition;
use file::{File, Row};
use std::io::{self, Write};
use terminal_utils as termutils;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

mod cursor;
mod file;
mod terminal_utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut should_quit = false;
    let file = {
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            File::open(&args[1]).unwrap_or_default()
        } else {
            File::default()
        }
    };
    let terminal_size = termion::terminal_size().unwrap();
    let mut cursor_position = CursorPosition::default();

    loop {
        if let Err(e) = refresh_screen(&file, &cursor_position, &terminal_size, &should_quit) {
            die(e);
        }

        if should_quit {
            break;
        }

        if let Err(e) = process_keypress(&mut cursor_position, &terminal_size, &mut should_quit) {
            die(e);
        }
    }
}

fn refresh_screen(
    file: &File,
    cursor_position: &CursorPosition,
    terminal_size: &(u16, u16),
    should_quit: &bool,
) -> Result<(), io::Error> {
    termutils::hide_cursor();
    termutils::set_cursor_position(&CursorPosition::default());

    if *should_quit {
        termutils::clear();
        println!("exited");
    } else {
        draw_rows(file, terminal_size);
        termutils::set_cursor_position(cursor_position);
    }

    termutils::show_cursor();
    io::stdout().flush()
}

fn process_keypress(
    cursor_position: &mut CursorPosition,
    terminal_size: &(u16, u16),
    should_quit: &mut bool,
) -> Result<(), io::Error> {
    let pressed_key = read_key()?;

    match pressed_key {
        Key::Ctrl('q') => *should_quit = true,
        Key::Up | Key::Down | Key::Left | Key::Right => {
            move_cursor(pressed_key, cursor_position, terminal_size)
        }
        _ => (),
    };

    Ok(())
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn move_cursor(pressed_key: Key, cursor_position: &mut CursorPosition, terminal_size: &(u16, u16)) {
    match pressed_key {
        Key::Up => cursor_position.y = cursor_position.y.saturating_sub(1),
        Key::Down => {
            if cursor_position.y < terminal_size.1 as usize {
                cursor_position.y = cursor_position.y.saturating_add(1)
            }
        }
        Key::Left => cursor_position.x = cursor_position.x.saturating_sub(1),
        Key::Right => {
            if cursor_position.x < terminal_size.0 as usize {
                cursor_position.x = cursor_position.x.saturating_add(1)
            }
        }
        _ => unreachable!(),
    };
}

fn draw_row(row: &Row, terminal_size: &(u16, u16)) {
    println!("{}\r", row.render(0, terminal_size.0 as usize));
}

fn draw_rows(file: &File, terminal_size: &(u16, u16)) {
    for terminal_row in 0..terminal_size.1 - 1 {
        termutils::clear_line();

        if let Some(row) = file.row(terminal_row as usize) {
            draw_row(row, terminal_size);
        } else if file.is_empty() && terminal_row == terminal_size.1 / 3 {
            draw_welcome_message(terminal_size);
        } else {
            println!("~\r");
        }
    }
}

fn draw_welcome_message(terminal_size: &(u16, u16)) {
    let mut welcome_message = format!("Edicode -- version {}", VERSION);
    let width = terminal_size.0 as usize;
    let len = welcome_message.len();
    let padding = width.saturating_sub(len) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    welcome_message = format!("~{}{}", spaces, welcome_message);
    welcome_message.truncate(width);
    println!("{}\r", welcome_message);
}

fn die(e: io::Error) {
    termutils::clear();
    panic!("{}", e);
}
