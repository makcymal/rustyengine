use {
    thiserror::Error,
};


/// Errors that can replace `Matrix` or be obtained within `ReRes::ReErr::MatrixErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum MatrixErr {
    #[error("add or sub on LHS of size {lhs:?}, RHS of size {rhs:?}")]
    AddSizesMismatch {
        lhs: (usize, usize),
        rhs: (usize, usize),
    },

    #[error("mul on LHS of size {lhs:?}, RHS of size {rhs:?}")]
    MulSizesMismatch {
        lhs: (usize, usize),
        rhs: (usize, usize),
    },

    #[error("scalar product between vectors with dim {lhs:?} and {rhs:?}")]
    ScalarProdDimMismatch {
        lhs: usize,
        rhs: usize,
    },

    #[error("vector product between vectors with dim {lhs:?} and {rhs:?}")]
    VectorProdDimMismatch {
        lhs: usize,
        rhs: usize,
    },

    #[error("inverse of matrix with null determinant")]
    NullDeterminant,

    #[error("division by number zero")]
    ZeroDivision,

    #[error("unclear whether rows or cols to use")]
    TooArbitrary,
}

/// Errors that can be obtained within `Result::Err::CoordSysErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum CoordSysErr {
    #[error("matrix of basis vectors is not square: {0:?}")]
    CurveBasis((usize, usize)),
}
