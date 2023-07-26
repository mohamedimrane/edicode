use crate::{
    cursor::Position,
    file::{File, Row},
    terminal_utils as termutils,
};
use std::io::{self, Write};
use termion::{event::Key, input::TermRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BAR_BG_COLOR: termion::color::Rgb = termion::color::Rgb(0, 50, 100);
const STATUS_BAR_FG_COLOR: termion::color::Rgb = termion::color::Rgb(255, 255, 255);

#[derive(PartialEq, Eq)]
enum Mode {
    Normal,
    Edit,
}

pub struct Editor {
    file: File,
    terminal_size: (u16, u16),
    mode: Mode,
    cursor_position: Position,
    offset: Position,
    should_quit: bool,
}

impl Default for Editor {
    fn default() -> Self {
        let mut terminal_size = termion::terminal_size().unwrap();
        terminal_size.1 -= 2;

        Self {
            file: {
                let args: Vec<String> = std::env::args().collect();

                if args.len() > 1 {
                    File::open(&args[1]).unwrap_or_default()
                } else {
                    File::default()
                }
            },
            terminal_size,
            mode: Mode::Normal,
            cursor_position: Position::default(),
            offset: Position::default(),
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
            self.draw_status_bar();
            termutils::set_cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        termutils::show_cursor();
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Editor::read_key()?;

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('s') => self.file.save().expect("Could not save file"),
            Key::Esc => self.mode = Mode::Normal,
            Key::Char('i') if self.mode == Mode::Normal => self.mode = Mode::Edit,
            Key::Char('k') | Key::Char('j') | Key::Char('h') | Key::Char('l')
                if self.mode == Mode::Normal =>
            {
                self.move_cursor(pressed_key)
            }

            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            Key::Backspace if self.mode == Mode::Edit => {
                let x = self.cursor_position.x.saturating_sub(1);
                let y = self.cursor_position.y;

                if x == 0 {
                    self.cursor_position.y -= 1;
                    self.cursor_position.x = self.file.row(y - 1).unwrap().len();
                } else {
                    self.move_cursor(Key::Left);
                }
                self.file.delete(&Position { x, y });
            }
            Key::Char(c) if self.mode == Mode::Edit => {
                self.file.insert(c, &self.cursor_position);
                self.move_cursor(Key::Right);
            }
            _ => (),
        };

        self.scroll();

        Ok(())
    }

    fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn scroll(&mut self) {
        if self.cursor_position.y < self.offset.y {
            self.offset.y = self.cursor_position.y;
        } else if self.cursor_position.y
            >= self.offset.y.saturating_add(self.terminal_size.1 as usize)
        {
            self.offset.y = self
                .cursor_position
                .y
                .saturating_sub(self.terminal_size.1 as usize)
                .saturating_add(1);
        }

        if self.cursor_position.x < self.offset.x {
            self.offset.x = self.cursor_position.x;
        } else if self.cursor_position.x
            >= self.offset.x.saturating_add(self.terminal_size.0 as usize)
        {
            self.offset.x = self
                .cursor_position
                .x
                .saturating_sub(self.terminal_size.0 as usize)
                .saturating_add(1);
        }
    }

    fn move_cursor(&mut self, pressed_key: Key) {
        let x = &mut self.cursor_position.x;
        let y = &mut self.cursor_position.y;

        let height = self.file.len();
        let mut width = if let Some(row) = self.file.row(*y) {
            row.len()
        } else {
            0
        };

        match pressed_key {
            Key::Up | Key::Char('k') => *y = y.saturating_sub(1),
            Key::Down | Key::Char('j') => {
                if *y < height as usize {
                    *y = y.saturating_add(1)
                }
            }
            Key::Left | Key::Char('h') => {
                if *x > 0 {
                    *x -= 1;
                } else if *y > 0 {
                    *y -= 1;

                    if let Some(row) = self.file.row(*y) {
                        *x = row.len();
                    } else {
                        *x = 0;
                    }
                }
            }
            Key::Right | Key::Char('l') => {
                if *x < width as usize {
                    *x += 1;
                } else if *y < height {
                    *y += 1;
                    *x = 0;
                }
            }
            _ => unreachable!(),
        };

        width = if let Some(row) = self.file.row(*y) {
            row.len()
        } else {
            0
        };

        if *x > width {
            *x = width;
        }
    }

    fn draw_row(&self, row: &Row) {
        println!(
            "{}\r",
            row.render(self.offset.x, self.terminal_size.0 as usize + self.offset.x)
        );
    }

    fn draw_rows(&self) {
        for terminal_row in 0..self.terminal_size.1 {
            termutils::clear_line();

            if let Some(row) = self.file.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.file.is_empty() && terminal_row == self.terminal_size.1 / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal_size.0 as usize;
        let file_name = if let Some(name) = self.file.name.clone() {
            name
        } else {
            "[scratch]".to_string()
        };
        let current_pos = format!(
            "{}:{}",
            self.cursor_position.y + 1,
            self.cursor_position.x + 1
        );

        let mut status = String::new();

        // right side
        status.push_str(&file_name);

        // separator
        let len = file_name.len() + current_pos.len() + 2; // +1 is the space at the start of the status bar
        status.push_str(&" ".repeat(width - len));

        // left side
        status.push_str(&current_pos);

        // spaces on the sides
        status.insert(0, ' ');
        status.insert(status.len(), ' ');

        termutils::set_bg_color(STATUS_BAR_BG_COLOR);
        termutils::set_fg_color(STATUS_BAR_FG_COLOR);

        println!("{}\r", status);

        termutils::reset_bg_color();
        termutils::reset_fg_color();
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
