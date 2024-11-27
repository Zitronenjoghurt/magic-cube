use crate::entities::cube::Cube;
use crate::enums::colors::Color;
use crate::enums::face_side::FaceSide;

#[test]
fn test_initialization() {
    let cube = Cube::new(3);
    assert_eq!(cube.size(), 3);
    
    assert_eq!(cube.get_face(FaceSide::Top).squares()[0], Color::White);
    assert_eq!(cube.get_face(FaceSide::Front).squares()[0], Color::Green);
    assert_eq!(cube.get_face(FaceSide::Right).squares()[0], Color::Red);
    assert_eq!(cube.get_face(FaceSide::Back).squares()[0], Color::Blue);
    assert_eq!(cube.get_face(FaceSide::Left).squares()[0], Color::Orange);
    assert_eq!(cube.get_face(FaceSide::Bottom).squares()[0], Color::Yellow);
    
    assert_eq!(cube.faces()[0].squares()[0], Color::White);
    assert_eq!(cube.faces()[1].squares()[0], Color::Green);
    assert_eq!(cube.faces()[2].squares()[0], Color::Red);
    assert_eq!(cube.faces()[3].squares()[0], Color::Blue);
    assert_eq!(cube.faces()[4].squares()[0], Color::Orange);
    assert_eq!(cube.faces()[5].squares()[0], Color::Yellow);
}