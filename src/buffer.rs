use crate::highlighting::HighlightType;
use crate::terminal_utils::color_fg;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Default)]
pub struct Buffer {
    pub save_location: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
}

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<HighlightType>,
}

impl Buffer {
    pub fn open(file_name: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(file_name)?;

        let mut rows = Vec::new();

        for value in contents.lines() {
            let mut row = Row::from(value);
            row.highlight();
            rows.push(row);
        }

        Ok(Self {
            save_location: Some(file_name.to_string()),
            rows,
            dirty: false,
        })
    }

    pub fn save(&mut self, save_location: &str) -> Result<(), io::Error> {
        let mut file = fs::File::create(save_location)?;
        for row in &self.rows {
            file.write_all(row.as_bytes())?;
            file.write_all(b"\n")?;
        }

        self.dirty = false;

        Ok(())
    }

    pub fn insert(&mut self, c: char, at: &crate::cursor::Position) {
        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        match at.y.cmp(&self.len()) {
            std::cmp::Ordering::Equal => {
                let mut row = Row::default();
                row.insert(0, c);
                self.rows.push(row);
            }
            std::cmp::Ordering::Less => self.row_mut(at.y).unwrap().insert(at.x, c),
            _ => (),
        }

        self.row_mut(at.y).unwrap().highlight();
        self.dirty = true;
    }

    pub fn delete(&mut self, at: &crate::cursor::Position, backspace: bool) {
        if at.y >= self.len() {
            return;
        }

        if !backspace {
            self.row_mut(at.y).unwrap().delete(at.x);
            self.row_mut(at.y).unwrap().highlight();
            return;
        }

        if at.x == 0 && at.y == 0 {
            return;
        }

        if at.x == 0 {
            let string = self.row(at.y).unwrap().string.clone();

            self.row_mut(at.y - 1).unwrap().string.push_str(&string);

            self.rows.remove(at.y);

            return;
        } else {
            self.row_mut(at.y).unwrap().delete(at.x.saturating_sub(1));
        }

        self.row_mut(at.y).unwrap().highlight();
        self.row_mut(at.y.saturating_sub(1)).unwrap().highlight();
        self.dirty = true;
    }

    pub fn insert_newline(&mut self, at: &crate::cursor::Position) {
        if let Some(row) = self.row(at.y) {
            if at.x == row.len() {
                self.rows.insert(at.y + 1, Row::default());
                return;
            }

            let row = self.row_mut(at.y).unwrap();
            let new_row = Row::from(&row.string[at.x..]);
            row.string = row.string[..at.x].to_string();
            self.rows.insert(at.y + 1, new_row);
            return;
        }

        self.rows.push(Row::default());

        self.row_mut(at.y).unwrap().highlight();
        self.row_mut(at.y + 1).unwrap().highlight();

        self.dirty = true;
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();

        for (index, c) in self
            .string
            .get(start..end)
            .unwrap_or_default()
            .to_string()
            .chars()
            .enumerate()
        {
            result.push_str(&color_fg(
                c,
                self.highlighting
                    .get(index)
                    .unwrap_or(&HighlightType::default())
                    .to_color(),
            ));
        }

        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            return;
        }

        self.string.insert(at, c);
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let first_seg = &self.string[..at];
        let second_seg = &self.string[at + 1..];
        let mut final_string = String::new();
        final_string.push_str(first_seg);
        final_string.push_str(second_seg);
        self.string = final_string;
    }

    pub fn highlight(&mut self) {
        let mut highlighting = Vec::new();
        for c in self.string.chars() {
            if c.is_ascii_digit() {
                highlighting.push(HighlightType::Number);
            } else {
                highlighting.push(HighlightType::None)
            }
        }
        self.highlighting = highlighting;
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
            highlighting: Vec::new(),
        }
    }
}
