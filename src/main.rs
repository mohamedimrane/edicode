use std::io::{self, Read};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => ,
            Err(e) => die(e),
        }
    }
}

fn process_keypress() -> Result<(), std::io::Error> {
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

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
