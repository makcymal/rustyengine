use {
    std::{
        ops::Neg,
        iter::Sum,
    },
    num_traits as nt,
};

pub trait Num: nt::Num + nt::NumAssign + Into<f64> + Copy + Sum + Neg<Output=Self> {}
impl<T: nt::Num + nt::NumAssign + Into<f64> + Copy + Sum + Neg<Output=Self>> Num for T {}


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


#[inline]
pub fn pow_minus<T: Num>(deg: usize) -> T {
    match deg % 2 == 0 {
        true => T::one(),
        false => -T::one(),
    }
}
