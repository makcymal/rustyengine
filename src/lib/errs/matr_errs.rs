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
}

// impl MatrErr {
//     pub fn dbg(&self) {
//         match self {
//             Self::AddSizesMismatch(lhs, rhs) =>
//                 println!("LHS has {:?} rows and {:?} cols, RHS has {:?} rows and {:?} cols",
//                          lhs.0, lhs.1, rhs.0, rhs.1),
//             Self::MulSizesMismatch(lhs, rhs) =>
//                 println!("LHS has {:?} rows and {:?} cols, RHS has {:?} rows and {:?} cols",
//                          lhs.0, lhs.1, rhs.0, rhs.1),
//         }
//     }
// }