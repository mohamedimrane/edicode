use std::{fs, io};

use crate::terminal_utils;

#[derive(Default)]
pub struct File {
    pub name: Option<String>,
    rows: Vec<Row>,
}

#[derive(Default)]
pub struct Row {
    string: String,
}

impl File {
    pub fn open(file_name: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(file_name)?;

        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            name: Some(file_name.to_string()),
            rows,
        })
    }

    pub fn insert(&mut self, c: char, at: &crate::cursor::Position) {
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            self.rows.get_mut(at.y).unwrap().insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &crate::cursor::Position) {
        if at.y >= self.len() {
            terminal_utils::set_bg_color(termion::color::Rgb(100, 100, 100));
            return;
        }

        self.rows.get_mut(at.y).unwrap().delete(at.x);
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
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
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            self.string.insert(at, c);
        }
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() || at == 0 {
            return;
        }

        let first_seg = &self.string[..at];
        let second_seg = &self.string[at + 1..];
        let mut final_string = String::new();
        final_string.push_str(first_seg);
        final_string.push_str(second_seg);
        self.string = final_string;
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
        }
    }
}
