#[derive(Clone, PartialEq)]
pub enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

type S = Segment;

impl Segment {
    pub fn as_usize(&self) -> usize {
        match self {
            S::Top => 0,
            S::TopLeft => 1,
            S::TopRight => 2,
            S::Middle => 3,
            S::BottomLeft => 4,
            S::BottomRight => 5,
            S::Bottom => 6,
        }
    }
}

pub const SEGMENTS0: [S; 6] = [
    S::Top,
    S::TopLeft,
    S::TopRight,
    S::BottomLeft,
    S::BottomRight,
    S::Bottom,
];
pub const SEGMENTS1: [S; 2] = [S::TopRight, S::BottomRight];
pub const SEGMENTS2: [S; 5] = [S::Top, S::TopRight, S::Middle, S::BottomLeft, S::Bottom];
pub const SEGMENTS3: [S; 5] = [S::Top, S::TopRight, S::Middle, S::BottomRight, S::Bottom];
pub const SEGMENTS4: [S; 4] = [S::TopLeft, S::TopRight, S::Middle, S::BottomRight];
pub const SEGMENTS5: [S; 5] = [S::Top, S::TopLeft, S::Middle, S::BottomRight, S::Bottom];
pub const SEGMENTS6: [S; 6] = [
    S::Top,
    S::TopLeft,
    S::Middle,
    S::BottomLeft,
    S::BottomRight,
    S::Bottom,
];
pub const SEGMENTS7: [S; 3] = [S::Top, S::TopRight, S::BottomRight];
pub const SEGMENTS8: [S; 7] = [
    S::Top,
    S::TopLeft,
    S::TopRight,
    S::Middle,
    S::BottomLeft,
    S::BottomRight,
    S::Bottom,
];
pub const SEGMENTS9: [S; 6] = [
    S::Top,
    S::TopLeft,
    S::TopRight,
    S::Middle,
    S::BottomRight,
    S::Bottom,
];
