use crate::{
    cursor::Position,
    file::{File, Row},
    terminal_utils as termutils,
};
use std::io::{self, Write};
use termion::{event::Key, input::TermRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    file: File,
    terminal_size: (u16, u16),
    cursor_position: Position,
    should_quit: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            file: {
                let args: Vec<String> = std::env::args().collect();

                if args.len() > 1 {
                    File::open(&args[1]).unwrap_or_default()
                } else {
                    File::default()
                }
            },
            terminal_size: termion::terminal_size().unwrap(),
            cursor_position: Position::default(),
            should_quit: false,
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }

            if self.should_quit {
                break;
            }

            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        termutils::hide_cursor();
        termutils::set_cursor_position(&Position::default());

        if self.should_quit {
            termutils::clear();
            println!("exited");
        } else {
            self.draw_rows();
            termutils::set_cursor_position(&self.cursor_position);
        }

        termutils::show_cursor();
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Editor::read_key()?;

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
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

    fn move_cursor(&mut self, pressed_key: Key) {
        match pressed_key {
            Key::Up => self.cursor_position.y = self.cursor_position.y.saturating_sub(1),
            Key::Down => {
                if self.cursor_position.y < self.terminal_size.1 as usize {
                    self.cursor_position.y = self.cursor_position.y.saturating_add(1)
                }
            }
            Key::Left => self.cursor_position.x = self.cursor_position.x.saturating_sub(1),
            Key::Right => {
                if self.cursor_position.x < self.terminal_size.0 as usize {
                    self.cursor_position.x = self.cursor_position.x.saturating_add(1)
                }
            }
            _ => unreachable!(),
        };
    }

    fn draw_row(&self, row: &Row) {
        println!("{}\r", row.render(0, self.terminal_size.0 as usize));
    }

    fn draw_rows(&self) {
        for terminal_row in 0..self.terminal_size.1 - 1 {
            termutils::clear_line();

            if let Some(row) = self.file.row(terminal_row as usize) {
                self.draw_row(row);
            } else if self.file.is_empty() && terminal_row == self.terminal_size.1 / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Edicode -- version {}", VERSION);
        let width = self.terminal_size.0 as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

fn die(e: io::Error) {
    termutils::clear();
    panic!("{}", e);
}
