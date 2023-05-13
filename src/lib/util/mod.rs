use {
    strum_macros::Display,
};

#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Line {
    Row, Col,
}

impl Line {
    pub fn inv(&self) -> Self {
        match self {
            Self::Row => Self::Col,
            Self::Col => Self::Row,
        }
    }
}


#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Cardinal {
    North, East, South, West,
}

impl Cardinal {
    pub fn inv(&self) -> Self {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::South => Cardinal::North,
            Cardinal::East => Cardinal::West,
            Cardinal::West => Cardinal::East,
        }
    }
}


#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum HandSide {
    Left, Right
}

impl HandSide {
    pub fn inv(&self) -> Self {
        match self {
            HandSide::Left => HandSide::Right,
            HandSide::Right => HandSide::Left,
        }
    }
}
