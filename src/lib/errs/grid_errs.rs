use {
    crate::grid::Repr,
    thiserror::Error,
};

/// Errors that can replace `Grid` or be obtained within `Result::Err::GridErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum GridErr {
    #[error("two-dim vector of curve shape has been passed, first row of unexpected len: {0}")]
    CurveSides(usize),

    #[error("trying to create empty Grid or Matr")]
    IsEmpty,

    #[error("trying to access on one-dim index in {0}")]
    IsNotLin(Repr),

    #[error("trying to access on two-dim index in {0}")]
    IsNotRec(Repr),


    #[error("Trying to access element on {idx:?}, while the actual size is {size:?}")]
    OutOfBounds {
        size: (usize, usize),
        idx: (usize, usize),
    },

    #[error("converting to single row while there are {0} rows")]
    TooManyRows(usize),
    #[error("converting to single col while there are {0} cols")]
    TooManyCols(usize),

    #[error("make sure you are handling all the errors")]
    UnhandledFailure,

    #[error("trying to transpose on {0}")]
    Untransposable(Repr),

    #[error("trying to iterate by rows on {0}")]
    NotIterableByRows(Repr),

    #[error("trying to iterate by cols on {0}")]
    NotIterableByCols(Repr),
}

// impl GridErr {
//     pub fn dbg(&self) {
//         match self {
//             Self::CurveSides(r) =>
//                 println!("Two-dim vector of curve shape has been passed. First row of unexpected len: {:?}", r),
//             Self::IsEmpty => println!("Trying to create empty Grid or Matr"),
//             Self::IsNotLin(repr) =>
//                 println!("Trying to access on one-dim index in {:?}", repr),
//             Self::IsNotRec(repr) =>
//                 println!("Trying to access on two-dim index in {:?}", repr),
//             Self::OutOfBounds(idx, size) =>
//                 println!("Trying to access element on {:?} row and {:?} col,\
//                 while there are only {:?} rows and {:?} cols",
//                          idx.0, idx.1, size.0, size.1),
//             Self::TooManyRows(r) => println!("Converting to single row while there are {:?} rows", r),
//             Self::TooManyCols(c) => println!("Converting to single col while there are {:?} cols", c),
//             Self::UnhandledFailure => println!("Handle all the errors"),
//             Self::Untransposable(repr) => println!("Trying to transpose on {:?}", repr),
//         }
//     }
// }
