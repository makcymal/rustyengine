use {
    thiserror::Error,
};


/// Errors that can replace `Matr` or be obtained within `Result::Err::MatrErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum MatrErr {
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

    #[error("determinant of non-square matr, size: {0:?}")]
    DeterminantOfNonSquare((usize, usize)),

    #[error("inverse of matr with null determinant")]
    NullDeterminant,

    #[error("matrix was divided by number zero")]
    ZeroDivision,
}
