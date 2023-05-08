use {
    crate::grid::Repr
};

/// Errors that can replace `Grid` or be obtained within `Result::Err::GridErr`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridErr {
    CurveSides(usize),
    IsEmpty,
    IsNotLin(Repr),
    IsNotRec(Repr),
    OutOfBounds((usize, usize), (usize, usize)),
    TooManyRows(usize),
    TooManyCols(usize),
    UnhandledFailure,
    Untransposable(Repr),
}

impl GridErr {
    pub fn dbg(&self) {
        match self {
            Self::CurveSides(r) =>
                println!("Two-dim vector of curve shape has been passed. First row of unexpected len: {:?}", r),
            Self::IsEmpty => println!("Trying to create empty Grid or Matr"),
            Self::IsNotLin(repr) =>
                println!("Trying to access on one-dim index in {:?}", repr),
            Self::IsNotRec(repr) =>
                println!("Trying to access on two-dim index in {:?}", repr),
            Self::OutOfBounds(idx, size) =>
                println!("Trying to access element on {:?} row and {:?} col,\
                while there are only {:?} rows and {:?} cols",
                         idx.0, idx.1, size.0, size.1),
            Self::TooManyRows(r) => println!("Converting to single row while there are {:?} rows", r),
            Self::TooManyCols(c) => println!("Converting to single col while there are {:?} cols", c),
            Self::UnhandledFailure => println!("Handle all the errors"),
            Self::Untransposable(repr) => println!("Trying to transpose on {:?}", repr),
        }
    }
}
