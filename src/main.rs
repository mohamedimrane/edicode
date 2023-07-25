use std::io::{self, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

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
            println!("{}", termion::cursor::Goto(1, 1));
        }

        if let Err(e) = process_keypress(&mut should_quit) {
            die(e);
        }
    }
}

fn refresh_screen(should_quit: &bool) -> Result<(), io::Error> {
    print!("{}{}", termion::cursor::Hide, termion::cursor::Goto(1, 1));

    if *should_quit {
        print!("{}", termion::clear::All);
        println!("exited");
    }

    print!("{}", termion::cursor::Show);
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
        print!("{}", termion::clear::CurrentLine);

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
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
