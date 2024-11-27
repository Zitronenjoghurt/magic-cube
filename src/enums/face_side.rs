#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(usize)]
pub enum FaceSide {
    Top = 0,
    Front = 1,
    Right = 2,
    Back = 3,
    Left = 4,
    Bottom = 5,
}