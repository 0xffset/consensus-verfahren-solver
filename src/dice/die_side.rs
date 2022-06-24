use std::fmt::Display;

/// The values a die's side can have
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DieSide {
    Zero,
    One,
    DontCare,
}

impl Display for DieSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DieSide::Zero => "0",
            DieSide::One => "1",
            DieSide::DontCare => "-",
        };
        write!(f, "{s}")
    }
}
