#[derive(Debug, PartialEq)]
pub enum MatrixifiedError {
    NonSquareMatrix,
    NonThirdMatrix,
    NullDeterminant,
    InappropriateSizes,
    InvalidIndex,
    RowBelowAcceptable,
    RowAboveAcceptable,
    ColBelowAcceptable,
    ColAboveAcceptable,
}

#[derive(Debug, PartialEq)]
pub enum MatrixLine {
    Row,
    Col,
}
