use crate::entities::face::Face;
use crate::enums::colors::Color;

#[test]
fn test_initialization() {
    let face = Face::new(3, Color::White).unwrap();
    assert_eq!(face.size(), 3);

    let squares = face.squares();
    assert_eq!(squares.len(), 9);
    assert_eq!(squares[0], Color::White);
    assert_eq!(squares[1], Color::White);
    assert_eq!(squares[2], Color::White);
    assert_eq!(squares[3], Color::White);
    assert_eq!(squares[4], Color::White);
    assert_eq!(squares[5], Color::White);
    assert_eq!(squares[6], Color::White);
    assert_eq!(squares[7], Color::White);
    assert_eq!(squares[8], Color::White);

    let face_error = Face::new(412, Color::Red).unwrap_err();
    assert_eq!(face_error, "Size out of range for Face")
}