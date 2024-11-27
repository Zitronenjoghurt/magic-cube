#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(usize)]
pub enum Color {
    White = 0,
    Yellow = 1,
    Green = 2,
    Red = 3,
    Blue = 4,
    Orange = 5,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}