use std::io::{self, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut should_quit = false;

    loop {
        if let Err(e) = refresh_screen(&should_quit) {
            die(e);
        }

        if should_quit {
            break;
        }

        if let Err(e) = process_keypress(&mut should_quit) {
            die(e);
        }
    }
}

fn refresh_screen(should_quit: &bool) -> Result<(), io::Error> {
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    if *should_quit {
        println!("exited");
    }

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

fn die(e: io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
