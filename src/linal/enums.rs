/// Provides possibity of picking a suitable storage of elements;
#[derive(Debug, Clone)]
pub enum Inner {
    Lin(Vec<f64>),
    Rec(Vec<Vec<f64>>),
}

impl Inner {
    /// Whether it's empty and has shape or right rectangle
    pub fn is_valid(&self) -> MatrRes<()> {
        match self {
            Inner::Lin(inner) => {
                if inner.len() == 0 {
                    MatrRes::No(MatrErr::EmptyInner)
                } else {
                    MatrRes::Go(())
                }
            }
            Inner::Rec(inner) => {
                if inner.len() == 0 {
                    MatrRes::No(MatrErr::EmptyInner)
                } else {
                    let mut cols = None;
                    for r in 0..inner.len() {
                        if cols.is_none() {
                            cols = Some(inner[r].len());
                        } else if inner[r].len() != cols.unwrap() {
                            return MatrRes::No(MatrErr::CurveSides);
                        }
                    }
                    if cols.unwrap() == 0 {
                        MatrRes::No(MatrErr::EmptyInner)
                    } else {
                        MatrRes::Go(())
                    }
                }
            }
        }
    }

    /// Number of rows
    pub fn rows(&self) -> usize {
        match self {
            Inner::Lin(_) => 1,
            Inner::Rec(inner) => inner.len(),
        }
    }

    /// Number of columns
    pub fn cols(&self) -> usize {
        match self {
            Inner::Lin(inner) => inner.len(),
            Inner::Rec(inner) => inner[0].len(),
        }
    }

    /// Element in `r` row and `c` column
    pub fn att(&self, r: usize, c: usize) -> MatrRes<f64> {
        match self {
            Inner::Lin(inner) => {
                if r == 0 && 0 <= c && c < inner.len() {
                    MatrRes::Go(inner[c])
                } else {
                    MatrRes::No(MatrErr::OutOfBounds)
                }
            },
            Inner::Rec(inner) => {
                if 0 <= r && r < inner.len() && 0 <= c && c < inner[0].len() {
                    MatrRes::Go(inner[r][c])
                } else {
                    MatrRes::No(MatrErr::OutOfBounds)
                }
            }
        }
    }

    /// Mut ref to element in `r` row and `c` column
    pub fn att_mut(&mut self, r: usize, c: usize) -> MatrRes<&mut f64> {
        match self {
            Inner::Lin(inner) => {
                if r == 0 && 0 <= c && c < inner.len() {
                    MatrRes::Go(&mut inner[c])
                } else {
                    MatrRes::No(MatrErr::OutOfBounds)
                }
            },
            Inner::Rec(inner) => {
                if 0 <= r && r < inner.len() && 0 <= c && c < inner[0].len() {
                    MatrRes::Go(&mut inner[r][c])
                } else {
                    MatrRes::No(MatrErr::OutOfBounds)
                }
            }
        }
    }
}


/// Describes how to treat the `Matr.grid`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Repr {
    #[default]
    Matrix,
    Row,
    Col,
    RowList,
    ColList,
    Failure(MatrErr),
}


/// Result for actions related to `Matr`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrRes<T> {
    Go(T),
    No(MatrErr),
}

impl<T> MatrRes<T> {
    pub fn unwrap(self) -> T {
        match self {
            MatrRes::Go(value) => value,
            _ => panic!("Calling unwrap() on MatrRes::No(err)"),
        }
    }
}


/// Errors that can replace `Matr.repr` or be obtained within `MatrRes`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrErr {
    EmptyInner,
    CurveSides,
    UnhandledFailure,
    Untransposable,
    NotLin,
    NotRec,
    NotSqr,
    OutOfBounds,
    AddSizesMismatch,
    MulSizesMismatch,
}
