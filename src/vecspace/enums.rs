#[derive(Debug, PartialEq)]
pub enum MatrixifiedError {
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

#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

