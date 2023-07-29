use termion::color::{Color, Reset, Rgb};

pub enum HighlightType {
    Number,
    None,
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
