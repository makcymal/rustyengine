/// Errors that can arise in matrixify module.
#[derive(Debug, PartialEq)]
pub enum MatrixifyErr {
    NonSquareMatrix,
    NonThirdMatrix,
    ZeroDeterminant,
    UnknownDeterminant,
    InappropriateSizes,
    NotAVector,
}
