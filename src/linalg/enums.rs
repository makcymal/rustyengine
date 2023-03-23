#[derive(Debug, PartialEq)]
pub enum MatrixError {
    NonSquareMatrix,
    NonThirdMatrix,
    ZeroDeterminant,
    UnknownDeterminant,
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

