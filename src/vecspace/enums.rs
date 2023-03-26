#[derive(Debug, PartialEq)]
pub enum MatrixifiedError {
    NonSquareMatrix,
    NonThirdMatrix,
    ZeroDeterminant,
    UnknownDeterminant,
    InappropriateSizes,
    InvalidIndex,
    DivByVector,
    NotAVector,
    ZeroDivision,
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

#[derive(Debug, PartialEq)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

