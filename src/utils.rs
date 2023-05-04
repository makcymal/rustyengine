use {
    crate::linal::{
        Point, Vector
    },
};

/// Enum describes shape of Matrixify implementors.
/// Matrix is intended to have size of Rect and Vector of Row or Col.
#[derive(Debug, Clone, Copy)]
pub enum Size {
    /// (y, x) = (rows, cols) = (height, width)
    Rect((usize, usize)),
    /// y = rows = height = 1
    Row(usize),
    /// x = cols = width = 1
    Col(usize),
}

impl Size {
    /// Returns first tuple value if self is Rect, else 1 or length if it's Row or Col respectively.
    pub fn rows(&self) -> usize {
        match self {
            Size::Rect((y, _)) => *y,
            Size::Row(_) => 1,
            Size::Col(y) => *y,
        }
    }

    /// Returns second tuple value if self is Rect, else length or 1 if it's Row or Col respectively.
    pub fn cols(&self) -> usize {
        match self {
            Size::Rect((_, x)) => *x,
            Size::Row(x) => *x,
            Size::Col(_) => 1,
        }
    }

    /// Transposes Size.
    pub fn transpose(&mut self) {
        *self = match self {
            Size::Rect((y, x)) => Size::Rect((*x, *y)),
            Size::Row(x) => Size::Col(*x),
            Size::Col(y) => Size::Row(*y),
        };
    }

    /// Whether size have more rows than columns.
    pub fn is_vertical(&self) -> bool {
        match self {
            Size::Row(_) => false,
            Size::Col(_) => true,
            Size::Rect((y, x)) => y > x,
        }
    }

    /// Whether size have more columns that rows.
    pub fn is_horizontal(&self) -> bool {
        match self {
            Size::Row(_) => true,
            Size::Col(_) => false,
            Size::Rect((y, x)) => x > y,
        }
    }

    /// Checks whether the given point contains in rectangle, which left bottom angle placed in the origin.
    pub fn contains(&self, row: usize, col: usize) -> bool {
        row < self.rows() && col < self.cols()
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Rect((0, 0))
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


/// Describes two possibilties of Add operation.
#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}


#[inline]
pub fn pow_minus(deg: usize) -> f64 {
    match deg % 2 == 0 {
        true => 1.0,
        false => -1.0,
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum AnyVal {
    None,
    Point(Point),
    Vector(Vector),
    Float(f64),
}
