use crate::entities::face::Face;
use crate::enums::colors::Color;

#[test]
fn test_initialization() {
    let face = Face::new(3, Color::White);
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
}

#[test]
fn test_rotation() {
    let mut face = Face::new(4, Color::White);

    let index_error1 = face.get_index(0, 4).unwrap_err();
    assert_eq!(index_error1, "Coordinates out of range for Face");

    let index_error2 = face.get_index(4, 0).unwrap_err();
    assert_eq!(index_error2, "Coordinates out of range for Face");

    assert_eq!(face.rotation(), 0);
    assert_eq!(face.get_index(0, 0).unwrap(), 0);
    assert_eq!(face.get_index(1, 0).unwrap(), 1);
    assert_eq!(face.get_index(2, 0).unwrap(), 2);
    assert_eq!(face.get_index(3, 0).unwrap(), 3);
    assert_eq!(face.get_index(0, 1).unwrap(), 4);
    assert_eq!(face.get_index(1, 1).unwrap(), 5);
    assert_eq!(face.get_index(2, 1).unwrap(), 6);
    assert_eq!(face.get_index(3, 1).unwrap(), 7);
    assert_eq!(face.get_index(0, 2).unwrap(), 8);
    assert_eq!(face.get_index(1, 2).unwrap(), 9);
    assert_eq!(face.get_index(2, 2).unwrap(), 10);
    assert_eq!(face.get_index(3, 2).unwrap(), 11);
    assert_eq!(face.get_index(0, 3).unwrap(), 12);
    assert_eq!(face.get_index(1, 3).unwrap(), 13);
    assert_eq!(face.get_index(2, 3).unwrap(), 14);
    assert_eq!(face.get_index(3, 3).unwrap(), 15);

    face.rotate_clockwise(1);
    assert_eq!(face.rotation(), 1);
    assert_eq!(face.get_index(0, 0).unwrap(), 12);
    assert_eq!(face.get_index(1, 0).unwrap(), 13);
    assert_eq!(face.get_index(2, 0).unwrap(), 14);
    assert_eq!(face.get_index(3, 0).unwrap(), 15);
    assert_eq!(face.get_index(0, 1).unwrap(), 8);
    assert_eq!(face.get_index(1, 1).unwrap(), 9);
    assert_eq!(face.get_index(2, 1).unwrap(), 10);
    assert_eq!(face.get_index(3, 1).unwrap(), 11);
    assert_eq!(face.get_index(0, 2).unwrap(), 4);
    assert_eq!(face.get_index(1, 2).unwrap(), 5);
    assert_eq!(face.get_index(2, 2).unwrap(), 6);
    assert_eq!(face.get_index(3, 2).unwrap(), 7);
    assert_eq!(face.get_index(0, 3).unwrap(), 0);
    assert_eq!(face.get_index(1, 3).unwrap(), 1);
    assert_eq!(face.get_index(2, 3).unwrap(), 2);
    assert_eq!(face.get_index(3, 3).unwrap(), 3);

    face.rotate_clockwise(1);
    assert_eq!(face.rotation(), 2);
    assert_eq!(face.get_index(0, 0).unwrap(), 15);
    assert_eq!(face.get_index(1, 0).unwrap(), 14);
    assert_eq!(face.get_index(2, 0).unwrap(), 13);
    assert_eq!(face.get_index(3, 0).unwrap(), 12);
    assert_eq!(face.get_index(0, 1).unwrap(), 11);
    assert_eq!(face.get_index(1, 1).unwrap(), 10);
    assert_eq!(face.get_index(2, 1).unwrap(), 9);
    assert_eq!(face.get_index(3, 1).unwrap(), 8);
    assert_eq!(face.get_index(0, 2).unwrap(), 7);
    assert_eq!(face.get_index(1, 2).unwrap(), 6);
    assert_eq!(face.get_index(2, 2).unwrap(), 5);
    assert_eq!(face.get_index(3, 2).unwrap(), 4);
    assert_eq!(face.get_index(0, 3).unwrap(), 3);
    assert_eq!(face.get_index(1, 3).unwrap(), 2);
    assert_eq!(face.get_index(2, 3).unwrap(), 1);
    assert_eq!(face.get_index(3, 3).unwrap(), 0);

    face.rotate_clockwise(1);
    assert_eq!(face.rotation(), 3);
    assert_eq!(face.get_index(0, 0).unwrap(), 3);
    assert_eq!(face.get_index(1, 0).unwrap(), 2);
    assert_eq!(face.get_index(2, 0).unwrap(), 1);
    assert_eq!(face.get_index(3, 0).unwrap(), 0);
    assert_eq!(face.get_index(0, 1).unwrap(), 7);
    assert_eq!(face.get_index(1, 1).unwrap(), 6);
    assert_eq!(face.get_index(2, 1).unwrap(), 5);
    assert_eq!(face.get_index(3, 1).unwrap(), 4);
    assert_eq!(face.get_index(0, 2).unwrap(), 11);
    assert_eq!(face.get_index(1, 2).unwrap(), 10);
    assert_eq!(face.get_index(2, 2).unwrap(), 9);
    assert_eq!(face.get_index(3, 2).unwrap(), 8);
    assert_eq!(face.get_index(0, 3).unwrap(), 15);
    assert_eq!(face.get_index(1, 3).unwrap(), 14);
    assert_eq!(face.get_index(2, 3).unwrap(), 13);
    assert_eq!(face.get_index(3, 3).unwrap(), 12);

    face.rotate_clockwise(1);
    assert_eq!(face.rotation(), 0);

    face.rotate_counter_clockwise(1);
    assert_eq!(face.rotation(), 3);
}