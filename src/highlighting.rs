use termion::color::{Color, Rgb};

#[derive(PartialEq, Eq)]
pub enum HighlightType {
    Number,
    None,
}

pub struct HighlightingOptions {
    pub highlight_numbers: bool,
}

impl HighlightType {
    pub fn to_color(&self) -> impl Color {
        use HighlightType::*;

        match self {
            Number => Rgb(220, 163, 163),
            None => Rgb(255, 255, 255),
        }
    }
}

impl HighlightingOptions {
    pub fn highlight_numbers(&self) -> bool {
        self.highlight_numbers
    }
}

impl Default for HighlightType {
    fn default() -> Self {
        Self::None
    }
}
