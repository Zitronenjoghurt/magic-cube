use crate::enums::colors::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    size: usize,
    squares: Vec<Color>,
}

impl Face {
    pub fn new(size: usize, color: Color) -> Face {
        Face {
            size,
            squares: vec![color; size],
        }
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
    
    pub fn squares(&self) -> &[Color] {
        &self.squares
    }
    
    pub fn squares_mut(&mut self) -> &mut [Color] {
        &mut self.squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_face() {
        let face = Face::new(9, Color::White);
        assert_eq!(9, face.size());
        assert_eq!(Color::White, face.squares()[0]);
        assert_eq!(Color::White, face.squares()[1]);
        assert_eq!(Color::White, face.squares()[2]);
        assert_eq!(Color::White, face.squares()[3]);
        assert_eq!(Color::White, face.squares()[4]);
        assert_eq!(Color::White, face.squares()[5]);
        assert_eq!(Color::White, face.squares()[6]);
        assert_eq!(Color::White, face.squares()[7]);
        assert_eq!(Color::White, face.squares()[8]);
    }
}