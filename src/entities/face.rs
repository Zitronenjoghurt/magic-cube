use crate::enums::colors::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    size: u8,
    squares: Box<[Color]>,
    rotation: u8,
}

impl Face {
    pub const MIN_SIZE: u8 = 1;

    pub const MAX_SIZE: u8 = 255;

    /// Creates a new cube face with uniform color
    ///
    /// # Arguments
    /// * `size` - Length of one side of the face between 1 and 255
    /// * `color` - Initial color for all squares
    ///
    /// # Examples
    /// ```
    /// use magic_cube::entities::face::Face;
    /// use magic_cube::enums::colors::Color;
    ///
    /// let face = Face::new(9, Color::White);
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
    pub fn new(size: u8, color: Color) -> Face {
        let square_count = (size * size) as usize;
        let squares = vec![color; square_count].into_boxed_slice();
        Face { size, squares, rotation: 0 }
    }

    /// Edge length of the face
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Current rotation of the face (clock-wise)
    /// 0 => 0°
    /// 1 => 90°
    /// 2 => 180°
    /// 3 => 270°
    pub fn rotation(&self) -> u8 {
        self.rotation
    }

    pub fn squares(&self) -> &[Color] {
        &self.squares
    }

    pub fn squares_mut(&mut self) -> &mut [Color] {
        &mut self.squares
    }

    /// (0,0) starts in the bottom left
    pub fn get_index(&self, x: u8, y: u8) -> Result<u8, &'static str> {
        if x >= self.size || y >= self.size {
            return Err("Coordinates out of range for Face");
        }

        let s = self.size;
        Ok(match self.rotation {
            0 => x + y * s,
            1 => x + (s - 1 - y) * s,
            2 => (s - 1 - x) + (s - 1 - y) * s,
            3 => (s - 1 - x) + y * s,
            _ => unreachable!()
        })
    }

    pub fn rotate(&mut self, clockwise: bool, amount: u8) {
        self.rotation = match clockwise {
            true => (self.rotation + (amount % 4)) % 4,
            false => (self.rotation + (4 - (amount % 4))) % 4
        };
    }

    pub fn rotate_clockwise(&mut self, amount: u8) {
        self.rotate(true, amount);
    }

    pub fn rotate_counter_clockwise(&mut self, amount: u8) {
        self.rotate(false, amount);
    }
}