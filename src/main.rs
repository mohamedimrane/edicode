use std::io::{self, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();

    loop {
        if let Err(e) = refresh_screen() {
            die(e);
        }

        if let Err(e) = process_keypress() {
            die(e);
        }
    }
}

fn refresh_screen() -> Result<(), io::Error> {
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    io::stdout().flush()
}

fn process_keypress() -> Result<(), io::Error> {
    let pressed_key = read_key()?;

    match pressed_key {
        Key::Char(c) => {
            if c.is_control() {
                println!("{:?}\r", c as u8);
            } else {
                println!("{:?} ({})\r", c as u8, c);
            }
        }
        Key::Ctrl('q') => panic!("Exited editor"),
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

fn die(e: io::Error) {
    panic!("{}", e);
}
