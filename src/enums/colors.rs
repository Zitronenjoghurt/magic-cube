#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    White,
    Yellow,
    Green,
    Red,
    Blue,
    Orange,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}