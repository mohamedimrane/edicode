use crate::{
    buffer::{Buffer, Row},
    cursor::Position,
    message::Message,
    terminal_utils as termutils,
};
use std::io::{self, Write};
use termion::{event::Key, input::TermRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BAR_BG_COLOR: termion::color::Rgb = termion::color::Rgb(52, 120, 198);
const STATUS_BAR_FG_COLOR: termion::color::Rgb = termion::color::Rgb(255, 255, 255);

#[derive(PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
}

pub struct Editor {
    buffers: Vec<Buffer>,
    current_buffer: usize,
    terminal_size: (u16, u16),
    mode: Mode,
    prompt_bar_message: Message,
    cursor_position: Position,
    offset: Position,
    should_quit: bool,
}

impl Default for Editor {
    fn default() -> Self {
        let buffers = vec![{
            let args: Vec<String> = std::env::args().collect();

            if args.len() > 1 {
                Buffer::open(&args[1]).unwrap_or_default()
            } else {
                Buffer::default()
            }
        }];

        let mut terminal_size = termion::terminal_size().unwrap();
        terminal_size.1 -= 3;

        Self {
            buffers,
            current_buffer: 0,
            terminal_size,
            mode: Mode::Normal,
            prompt_bar_message: Message::default(),
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
            self.draw_command_bar();
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

        self.prompt_bar_message = Message::default();

        match pressed_key {
            // non mode specific
            Key::Esc => self.mode = Mode::Normal,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),

            // normal mode specific
            Key::Char(':') if self.mode == Mode::Normal => {
                if let Ok(Some(command)) = self.prompt(":") {
                    self.process_command(command)?;
                }
            }
            Key::Char('i') if self.mode == Mode::Normal => self.mode = Mode::Insert,
            Key::Char('k') | Key::Char('j') | Key::Char('h') | Key::Char('l')
                if self.mode == Mode::Normal =>
            {
                self.move_cursor(pressed_key)
            }

            // insert mode specific
            Key::Backspace if self.mode == Mode::Insert => {
                let x = self.cursor_position.x;
                let y = self.cursor_position.y;

                if !(x == 0 && y == 0) {
                    if x == 0 {
                        self.cursor_position.y = y.saturating_sub(1);
                        self.cursor_position.x = self.buffers[self.current_buffer]
                            .row(y.saturating_sub(1))
                            .unwrap()
                            .len();
                    } else {
                        self.move_cursor(Key::Left);
                    }

                    self.buffers[self.current_buffer].delete(&Position { x, y });
                }
            }
            Key::Char(c) if self.mode == Mode::Insert => {
                self.buffers[self.current_buffer].insert(c, &self.cursor_position);
                self.move_cursor(Key::Right);
            }
            _ => (),
        };

        self.scroll();

        Ok(())
    }

    fn process_command(&mut self, command: String) -> Result<(), io::Error> {
        let command = command.split(' ').collect::<Vec<&str>>();
        match command[0] {
            "w" => {
                self.command_save_file(&command)?;
                Ok(())
            }
            "q" => {
                self.command_quit(&command)?;
                Ok(())
            }
            "wq" | "x" => {
                self.command_save_file(&command)?;
                self.command_quit(&command)?;
                Ok(())
            }
            "n" => {
                self.command_new_buffer(&command)?;
                Ok(())
            }
            "o" => {
                self.command_open_file(&command)?;
                Ok(())
            }
            "" => Ok(()),
            _ => {
                self.prompt_bar_message =
                    Message::new_error(format!("Unknown command: {}", command[0]));
                Ok(())
            }
        }
    }

    fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, io::Error> {
        let mut result = String::new();

        loop {
            self.prompt_bar_message = Message::new_normal(format!("{}{}", prompt, result));

            self.refresh_screen()?;

            match Self::read_key()? {
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Backspace => {
                    result.pop();
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
        }

        self.prompt_bar_message = Message::default();

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
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

        let height = self.buffers[self.current_buffer].len();
        let mut width = if let Some(row) = self.buffers[self.current_buffer].row(*y) {
            row.len()
        } else {
            0
        };

        match pressed_key {
            Key::Up | Key::Char('k') => *y = y.saturating_sub(1),
            Key::Down | Key::Char('j') => {
                if *y < height {
                    *y = y.saturating_add(1)
                }
            }
            Key::Left | Key::Char('h') => {
                if *x > 0 {
                    *x -= 1;
                } else if *y > 0 {
                    *y -= 1;

                    if let Some(row) = self.buffers[self.current_buffer].row(*y) {
                        *x = row.len();
                    } else {
                        *x = 0;
                    }
                }
            }
            Key::Right | Key::Char('l') => {
                if *x < width {
                    *x += 1;
                } else if *y < height {
                    *y += 1;
                    *x = 0;
                }
            }
            _ => unreachable!(),
        };

        width = if let Some(row) = self.buffers[self.current_buffer].row(*y) {
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
            let buffer = &self.buffers[self.current_buffer];

            if let Some(row) = buffer.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if buffer.is_empty() && terminal_row == self.terminal_size.1 / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal_size.0 as usize;
        let mode = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        };
        let file_name = if let Some(name) = self.buffers[self.current_buffer].save_location.clone()
        {
            name
        } else {
            "[scratch]".to_string()
        };
        let is_dirty = if self.buffers[self.current_buffer].is_dirty() {
            "[+]"
        } else {
            ""
        };
        let current_pos = format!(
            "{}:{}",
            self.cursor_position.y + 1,
            self.cursor_position.x + 1
        );

        let mut status = String::new();

        let left_side = format!("{}   {} {}", mode, file_name, is_dirty);
        let right_side = format!("{}", current_pos);

        status.push_str(&left_side);

        // separator
        let len = left_side.len() + right_side.len() + 2; // +1 is the space at the start of the status bar
        status.push_str(&" ".repeat(width - len));

        status.push_str(&right_side);

        // spaces on the sides
        status.insert(0, ' ');
        status.insert(status.len(), ' ');

        termutils::set_bg_color(STATUS_BAR_BG_COLOR);
        termutils::set_fg_color(STATUS_BAR_FG_COLOR);

        println!("{}\r", status);

        termutils::reset_bg_color();
        termutils::reset_fg_color();
    }

    fn draw_command_bar(&self) {
        termutils::clear_line();
        println!("{}", self.prompt_bar_message);
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

    fn command_save_file(&mut self, command: &Vec<&str>) -> Result<(), io::Error> {
        let buffer = &mut self.buffers[self.current_buffer];
        let mut save_location = buffer.save_location.clone().unwrap_or_default();
        if let Some(new_save_location) = command.get(1).copied() {
            save_location = new_save_location.to_string();
            buffer.save_location = Some(save_location.clone());
        }

        if save_location.is_empty() {
            self.prompt_bar_message =
                Message::new_error("Can't save with no path set!".to_string());
            return Ok(());
        }

        buffer.save(&save_location)?;
        self.prompt_bar_message = Message::new_normal(format!("\"{}\" written", save_location));

        Ok(())
    }

    fn command_quit(&mut self, _command: &Vec<&str>) -> Result<(), io::Error> {
        self.should_quit = true;
        Ok(())
    }

    fn command_new_buffer(&mut self, _command: &Vec<&str>) -> Result<(), io::Error> {
        self.add_buffer(Buffer::default());
        self.cursor_position = Position::default();
        Ok(())
    }

    fn command_open_file(&mut self, command: &Vec<&str>) -> Result<(), io::Error> {
        if let Some(file_location) = command.get(1) {
            let buffer = match Buffer::open(file_location) {
                Ok(buffer) => buffer,
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => {
                        self.prompt_bar_message = Message::new_error(e.to_string());
                        return Ok(());
                    }
                    _ => return Result::Err(e),
                },
            };

            self.add_buffer(buffer);
        } else {
            self.prompt_bar_message = Message::new_error("File path not given!".to_string());
        }

        Ok(())
    }

    fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
        self.current_buffer += 1;
    }
}

fn die(e: io::Error) {
    termutils::clear();
    panic!("{}", e);
}
