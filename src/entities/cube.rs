use crate::enums::colors::Color;
use crate::entities::face::Face;
use crate::enums::face_side::FaceSide;

#[derive(Debug, Clone)]
pub struct Cube {
    height: u32,
    width: u32,
    faces: [Face; 6]
}

impl Cube {
    pub fn new(height: u32, width: u32) -> Cube {
        let size = (height * width) as usize;
        Cube {
            height,
            width,
            faces: [
                Face::new(size, Color::White),
                Face::new(size, Color::Green),
                Face::new(size, Color::Red),
                Face::new(size, Color::Blue),
                Face::new(size, Color::Orange),
                Face::new(size, Color::Yellow),
            ]
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
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