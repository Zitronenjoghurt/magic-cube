use crate::enums::colors::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    size: usize,
    squares: Box<[Color]>,
}

impl Face {
    pub const MIN_SIZE: usize = 1;
    
    /// Considering the smallest usize of 16-bit, the safest max size would be sqrt(2^16) = 2^4 = 256
    pub const MAX_SIZE: usize = 256;

    /// Creates a new cube face with uniform color
    ///
    /// # Arguments
    /// * `size` - Length of one side of the face between 1 and 256
    /// * `color` - Initial color for all squares
    ///
    /// # Examples
    /// ```
    /// use magic_cube::entities::face::Face;
    /// use magic_cube::enums::colors::Color;
    ///
    /// let face = Face::new(9, Color::White).unwrap();
    /// assert_eq!(9, face.size());
    /// assert_eq!(face.squares()[0], Color::White);
    /// assert_eq!(face.squares()[1], Color::White);
    /// assert_eq!(face.squares()[2], Color::White);
    /// assert_eq!(face.squares()[3], Color::White);
    /// assert_eq!(face.squares()[4], Color::White);
    /// assert_eq!(face.squares()[5], Color::White);
    /// assert_eq!(face.squares()[6], Color::White);
    /// assert_eq!(face.squares()[7], Color::White);
    /// assert_eq!(face.squares()[8], Color::White);
    /// ```
    pub fn new(size: usize, color: Color) -> Result<Face, &'static str> {
        if !(Face::MIN_SIZE..=Face::MAX_SIZE).contains(&size) {
            return Err("Size out of range for Face");
        }

        let squares = vec![color; size * size].into_boxed_slice();
        Ok(Face { size, squares })
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