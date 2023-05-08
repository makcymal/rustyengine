/// Errors that can replace `Matr` or be obtained within `Result::Err::MatrErr`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrErr {
    AddSizesMismatch((usize, usize), (usize, usize)),
    MulSizesMismatch((usize, usize), (usize, usize)),
}

impl MatrErr {
    pub fn dbg(&self) {
        match self {
            Self::AddSizesMismatch(lhs, rhs) =>
                println!("LHS has {:?} rows and {:?} cols, RHS has {:?} rows and {:?} cols",
                         lhs.0, lhs.1, rhs.0, rhs.1),
            Self::MulSizesMismatch(lhs, rhs) =>
                println!("LHS has {:?} rows and {:?} cols, RHS has {:?} rows and {:?} cols",
                         lhs.0, lhs.1, rhs.0, rhs.1),
        }
    }
}