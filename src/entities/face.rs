use crate::enums::colors::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub squares: Vec<Color>,
}

impl Face {
    pub fn new(size: usize, color: Color) -> Face {
        Face {
            squares: vec![color; size],
        }
    }
}