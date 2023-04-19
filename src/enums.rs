/// Errors that can arise within matrixify module.
#[derive(Debug, PartialEq)]
pub enum MatrixifyErr {
    NonSquareMatrix,
    NonThirdMatrix,
    ZeroDeterminant,
    UnknownDeterminant,
    InappropriateSizes,
    NotAVector,
}

/// Errors that can arise while applying rotations.
#[derive(Debug, PartialEq)]
pub enum RotationErr {
    NonExistentAxis,
    RepeatedAxis,
}
