use thiserror::Error;

/// Errors that can replace `Matrix` or be obtained within `ReRes::ReErr::MatrixErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum MathErr {
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
    DimMismatch { lhs: usize, rhs: usize },

    #[error("inverse of matrix with null determinant")]
    NullDeterminant,

    #[error("division by number zero")]
    ZeroDivision,

    #[error("trying to operate in 3-dim space")]
    NotIn3Dim,

    #[error("trying to create rotation matrix from {0} to {0} axis")]
    RotationInOneAxis(usize),

    #[error("trying to set global DIM variable to 0")]
    ZeroDimSpace,
}
