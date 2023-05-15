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
    Emptiness,

    #[error("trying to access on double index in {0}")]
    SingleIndexed(Repr),

    #[error("trying to access on single index in {0}")]
    DoubleIndexed(Repr),


    // #[error("Trying to access element on {idx:?}, while the actual size is {size:?}")]
    // OutOfBounds {
    //     size: (usize, usize),
    //     idx: (usize, usize),
    // },

    #[error("operating on single row while there are {0} rows")]
    TooManyRows(usize),
    #[error("operating on single col while there are {0} cols")]
    TooManyCols(usize),

    #[error("converted into(Repr::Failure)")]
    ConvertedToFailure,

    #[error("make sure you are handling all the errors")]
    UnhandledFailure,

    #[error("trying to transpose on {0}")]
    Untransposable(Repr),

    #[error("trying to iterate by rows on {0}")]
    NotIterableByLines(Repr),

    #[error("trying to operate on matrix as on the row or col")]
    NotRowOrCol,

    #[error("trying to operate on matrix as on the set of rows or cols")]
    NotMultiRowOrCol,

    #[error("treating non-square matrix as square of size: {0:?}")]
    NotSquare((usize, usize)),

    #[error("appending rows of len {tail:?} to rows of len {dest:?}")]
    RowsAppendMismatch {
        dest: usize,
        tail: usize,
    },

    #[error("appending cols of len {tail:?} to cols of len {dest:?}")]
    ColsAppendMismatch {
        dest: usize,
        tail: usize,
    }
}
