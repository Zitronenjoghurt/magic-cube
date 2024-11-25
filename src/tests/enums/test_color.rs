use crate::enums::colors::Color;

#[test]
fn test_default() {
    let default_color = Color::default();
    assert_eq!(default_color, Color::White);
}