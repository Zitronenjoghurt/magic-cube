use crate::enums::colors::Color;
use crate::entities::face::Face;
use crate::enums::face_side::FaceSide;

#[derive(Debug, Clone)]
pub struct Cube {
    size: usize,
    faces: [Face; 6]
}

impl Cube {
    /// Creates a 6-colored magic cube, all faces will be uniformly colored
    ///
    /// # Arguments
    /// * `size` - Edge length of the cube between 1 and 256
    ///
    /// # Examples
    /// ```
    /// use magic_cube::entities::cube::Cube;
    /// use magic_cube::enums::colors::Color;
    /// use magic_cube::enums::face_side::FaceSide;
    ///
    /// let cube = Cube::new(3).unwrap();
    /// assert_eq!(cube.size(), 3);
    /// assert_eq!(cube.get_face(FaceSide::Top).squares()[0], Color::White);
    /// assert_eq!(cube.get_face(FaceSide::Front).squares()[0], Color::Green);
    /// assert_eq!(cube.get_face(FaceSide::Right).squares()[0], Color::Red);
    /// assert_eq!(cube.get_face(FaceSide::Back).squares()[0], Color::Blue);
    /// assert_eq!(cube.get_face(FaceSide::Left).squares()[0], Color::Orange);
    /// assert_eq!(cube.get_face(FaceSide::Bottom).squares()[0], Color::Yellow);
    /// ```
    pub fn new(size: usize) -> Result<Cube, &'static str> {
        let cube = Cube {
            size,
            faces: [
                Face::new(size, Color::White)?,
                Face::new(size, Color::Green)?,
                Face::new(size, Color::Red)?,
                Face::new(size, Color::Blue)?,
                Face::new(size, Color::Orange)?,
                Face::new(size, Color::Yellow)?,
            ]
        };
        Ok(cube)
    }

    /// Returns the edge length of the cube
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    pub fn faces_mut(&mut self) -> &mut [Face] {
        &mut self.faces
    }

    pub fn get_face(&self, side: FaceSide) -> &Face {
        &self.faces[side as usize]
    }
    
    pub fn get_face_mut(&mut self, side: FaceSide) -> &mut Face {
        &mut self.faces[side as usize]
    }
}