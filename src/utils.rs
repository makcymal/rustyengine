use crate::globals::Flt;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    // (y, x) = (rows, cols) = (height, width)
    Rect((usize, usize)),
    Row(usize),
    Col(usize),
}

impl Size {
    pub fn rows(&self) -> usize {
        match self {
            Size::Rect((y, _)) => *y,
            Size::Row(_) => 1,
            Size::Col(y) => *y,
        }
    }

    pub fn cols(&self) -> usize {
        match self {
            Size::Rect((_, x)) => *x,
            Size::Row(x) => *x,
            Size::Col(_) => 1,
        }
    }

    pub fn transpose(&mut self) {
        *self = match self {
            Size::Rect((y, x)) => Size::Rect((*x, *y)),
            Size::Row(x) => Size::Col(*x),
            Size::Col(y) => Size::Row(*y),
        };
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Size::Row(_) => false,
            Size::Col(_) => true,
            Size::Rect((y, x)) => y > x,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Size::Row(_) => true,
            Size::Col(_) => false,
            Size::Rect((y, x)) => x > y,
        }
    }

    pub fn contains(&self, row: usize, col: usize) -> bool {
        0 <= row && row < self.rows() && 0 <= col && col < self.cols()
    }
}

impl From<&Size> for (usize, usize) {
    fn from(size: &Size) -> Self {
        match size {
            Size::Rect((y, x)) => (*y, *x),
            Size::Row(x) => (1, *x),
            Size::Col(y) => (*y, 1),
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        let lhs: (usize, usize) = self.into();
        let rhs: (usize, usize) = other.into();
        lhs == rhs
    }
}


#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}


#[inline]
pub fn pow_minus(deg: usize) -> Flt {
    match deg % 2 == 0 {
        true => 1.0,
        false => -1.0,
    }
}
