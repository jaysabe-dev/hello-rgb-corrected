#[derive(Clone, Copy)]
pub enum Component {
    H,
    S,
    V,
}

impl Component {
    pub fn prev(self) -> Self {
        match self {
            Self::H => Self::V,
            Self::S => Self::H,
            Self::V => Self::S,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::H => Self::S,
            Self::S => Self::V,
            Self::V => Self::H,
        }
    }
}
