use termion::color::{Color, Rgb};

#[derive(PartialEq, Eq)]
pub enum HighlightType {
    Number,
    String,
    None,
}

pub struct HighlightingOptions {
    pub highlight_numbers: bool,
    pub highlight_strings: Option<Vec<char>>,
}

impl HighlightType {
    pub fn to_color(&self) -> impl Color {
        use HighlightType::*;

        match self {
            Number => Rgb(220, 163, 163),
            String => Rgb(211, 54, 130),
            None => Rgb(255, 255, 255),
        }
    }
}

impl HighlightingOptions {
    pub fn highlight_numbers(&self) -> bool {
        self.highlight_numbers
    }

    pub fn highlight_strings(&self) -> bool {
        self.highlight_strings.is_some()
    }

    pub fn is_string_delimiter(&self, c: char) -> bool {
        if let Some(string_delimiters) = &self.highlight_strings {
            string_delimiters.contains(&c)
        } else {
            false
        }
    }
}

impl Default for HighlightType {
    fn default() -> Self {
        Self::None
    }
}
