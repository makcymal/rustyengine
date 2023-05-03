

/// Provides possibity of picking a suitable storage of elements;
#[derive(Debug, Clone)]
pub enum Inner {
    /// One-dimension vector;
    Lin(Vec<f64>),
    /// Two-dimension matrix;
    Rec(Vec<Vec<f64>>),
}

impl Inner {
    /// Whether it's one-dimension vector;
    pub fn is_solo(&self) -> bool {
        match self {
            Inner::Lin(_) => true,
            _ => false,
        }
    }

    /// Whether it's two-dimension matrix;
    pub fn is_duet(&self) -> bool {
        match self {
            Inner::Rec(_) => true,
            _ => false,
        }
    }

    /// Whether it's empty at all and has shape or right rectangle;
    pub fn validate(&self) -> Result<(), MatrErr> {
        match self {
            Inner::Lin(inner) => {
                if inner.len() == 0 {
                    Err(MatrErr::EmptyAtAll)
                } else {
                    Ok(())
                }
            }
            Inner::Rec(inner) => {
                if inner.len() == 0 {
                    Err(MatrErr::EmptyAtAll)
                } else {
                    let mut cols = None;
                    for r in 0..inner.len() {
                        if cols.is_none() {
                            cols = Some(inner[r].len());
                        } else if inner[r].len() != cols.unwrap() {
                            return Err(MatrErr::CurveSides);
                        }
                    }
                    if cols.unwrap() == 0 {
                        Err(MatrErr::EmptyAtAll)
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }

    /// Number of rows;
    pub fn rows(&self) -> usize {
        match self {
            Self::Lin(_) => 1,
            Self::Rec(inner) => inner.len(),
        }
    }

    /// Number of cols;
    /// Suppose self isn't emtpy;
    pub fn cols(&self) -> usize {
        match self {
            Self::Lin(inner) => inner.len(),
            Self::Rec(inner) => inner[0].len(),
        }
    }

    /// Ref to element on position (r, c) (or error);
    pub fn att(&self, (r, c): (usize, usize)) -> Result<&f64, MatrErr> {
        if 0 <= r && r < self.rows() && 0 <= c && c < self.cols() {
            match self {
                Inner::Lin(inner) => Ok(&inner[c]),
                Inner::Rec(inner) => Ok(&inner[r][c]),
            }
        } else {
            Err(MatrErr::OutOfBounds)
        }
    }

    /// Mut ref to element on position (r, c) (or error);
    pub fn att_mut(&mut self, (r, c): (usize, usize)) -> Result<&mut f64, MatrErr> {
        if 0 <= r && r < self.rows() && 0 <= c && c < self.cols() {
            match self {
                Inner::Lin(inner) => Ok(&mut inner[c]),
                Inner::Rec(inner) => Ok(&mut inner[r][c]),
            }
        } else {
            Err(MatrErr::OutOfBounds)
        }
    }
}


/// Describes how to treat the inner;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repr {
    /// Treats Matrixify instance as arbitrary non-empty matrix with shape of rectangle;
    Matrix,
    /// Treats Matrixify instance as non-empty square matrix;
    /// Skips checking whether it's square:
    Square,
    /// Treats Matrixify instance as non-empty row;
    /// Calling transpose() is prohibited, to transpose switch() it to Col;
    Row,
    /// Treats Matrixify instance as non-empty row;
    /// Calling transpose() is prohibited, to transpose switch() it to Row;
    Col,
    /// Treats an instance as non-empty list of rows;
    /// Calling transpose() isn't prohibited, but rows will be entirely different;
    /// Calling switch() to ColList transposes it and then sets ColList;
    /// Furthermore, calling col_iter() is prohibited;
    RowList,
    /// Treats an instance as non-empty list of cols;
    /// Calling transpose() isn't prohibited, but cols will be entirely different;
    /// Calling switch() to RowList transposes it and then sets RowList;
    /// Furthermore, calling row_iter() is prohibited;
    ColList,
    /// When inabortable errors occuries, it's filled with related error;
    /// Always check if matrixify.repr is Repr::Err(_);
    Failure(MatrErr),
}


/// Errors that can replace Matrixify inner or be obtained within Result;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrErr {
    ErrByDesign,
    UnhandledFailure,
    EmptyAtAll,
    CurveSides,
    CrookedSquare,
    TooManyRows,
    TooManyCols,
    OutOfBounds,
    Untransposable,
    IsLinear,
    NotLinear,
}


pub enum Dir {
    Hor,
    Ver,
}
