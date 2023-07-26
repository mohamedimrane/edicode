use std::{
    fs,
    io::{self, Write},
};

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

    pub fn save(&self) -> Result<(), io::Error> {
        if let Some(name) = &self.name {
            let mut file = fs::File::create(name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

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
    }

    pub fn delete(&mut self, at: &crate::cursor::Position) {
        if at.y >= self.len() || (at.x == 0 && at.y == 0) {
            return;
        }

        if at.x == 0 {
            let string = self.row(at.y).unwrap().string.clone();

            self.row_mut(at.y - 1).unwrap().string.push_str(&string);

            self.rows.remove(at.y);

            return;
        }

        self.row_mut(at.y).unwrap().delete(at.x);
    }

    pub fn insert_newline(&mut self, at: &crate::cursor::Position) {
        if let Some(row) = self.row(at.y) {
            if at.x == row.len() {
                self.rows.insert(at.y + 1, Row::default());
            } else {
                let row = self.row_mut(at.y).unwrap();
                let new_row = Row::from(&row.string[at.x..]);
                row.string = row.string[..at.x].to_string();
                self.rows.insert(at.y + 1, new_row);
            }
        } else {
            self.rows.push(Row::default());
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
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

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
        }
    }
}
