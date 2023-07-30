use termion::color::{Color, Rgb};

#[derive(PartialEq, Eq)]
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

impl Default for HighlightType {
    fn default() -> Self {
        Self::None
    }
}
