use std::io::{self, Write};
use terminal_utils as termutils;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

mod terminal_utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut should_quit = false;
    let terminal_size = termion::terminal_size().unwrap();

    loop {
        if let Err(e) = refresh_screen(&should_quit) {
            die(e);
        }

        if should_quit {
            break;
        } else {
            draw_rows(terminal_size);
            termutils::set_cursor_position(0, 0);
        }

        if let Err(e) = process_keypress(&mut should_quit) {
            die(e);
        }
    }
}

fn refresh_screen(should_quit: &bool) -> Result<(), io::Error> {
    termutils::hide_cursor();
    termutils::set_cursor_position(0, 0);

    if *should_quit {
        termutils::clear();
        println!("exited");
    }

    termutils::show_cursor();
    io::stdout().flush()
}

fn process_keypress(should_quit: &mut bool) -> Result<(), io::Error> {
    let pressed_key = read_key()?;

    match pressed_key {
        Key::Char(c) => {
            if c.is_control() {
                println!("{:?}\r", c as u8);
            } else {
                println!("{:?} ({})\r", c as u8, c);
            }
        }
        Key::Ctrl('q') => *should_quit = true,
        _ => println!("{:?}\r", pressed_key),
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

fn draw_rows(terminal_size: (u16, u16)) {
    for row in 0..terminal_size.1 - 1 {
        termutils::clear_line();

        if row == terminal_size.1 / 3 {
            draw_welcome_message(terminal_size);
        } else {
            println!("~\r");
        }
    }
}

fn draw_welcome_message(terminal_size: (u16, u16)) {
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
