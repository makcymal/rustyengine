use {
    crate::{
        errs::{
            AnyRes::{self, *},
            AnyErr::{self, *},
            GridErr::{self, *}
        },
    },
};


/// Provides possibity of picking a suitable storage of elements
#[derive(Debug, Clone, PartialEq)]
pub(in super) enum VecWrapper<T> {
    /// One-dimensional `Vec`
    Lin(Vec<T>),
    /// Two-dimensional `Vec`
    Rec(Vec<Vec<T>>),
}

impl<T> VecWrapper<T> {
    /// Whether it's empty and has shape or right rectangle
    pub(in super) fn is_valid(&self) -> AnyRes<()> {
        match self {
            VecWrapper::Lin(vec) => {
                if vec.len() == 0 {
                    No(GridErr(IsEmpty))
                } else {
                    Go(())
                }
            }
            VecWrapper::Rec(vec) => {
                if vec.len() == 0 {
                    No(GridErr(IsEmpty))
                } else {
                    let mut cols = None;
                    for r in 0..vec.len() {
                        if cols.is_none() {
                            cols = Some(vec[r].len());
                        } else if vec[r].len() != cols.unwrap() {
                            return No(GridErr(CurveSides(r)));
                        }
                    }
                    if cols.unwrap() == 0 {
                        No(GridErr(IsEmpty))
                    } else {
                        Go(())
                    }
                }
            }
        }
    }

    /// Number of rows
    pub(in super) fn rows(&self) -> usize {
        match self {
            VecWrapper::Lin(_) => 1,
            VecWrapper::Rec(vec) => vec.len(),
        }
    }

    /// Number of columns, under assumption vec isn't empty
    pub(in super) fn cols(&self) -> usize {
        match self {
            VecWrapper::Lin(vec) => vec.len(),
            VecWrapper::Rec(vec) => vec[0].len(),
        }
    }

    /// Element in `r` row and `c` column
    pub(in super) fn att(&self, r: usize, c: usize) -> AnyRes<&T> {
        match self {
            VecWrapper::Lin(vec) => {
                if r == 0 && 0 <= c && c < vec.len() {
                    Go(&vec[c])
                } else {
                    No(GridErr(OutOfBounds((r, c), (1, vec.len()))))
                }
            }
            VecWrapper::Rec(vec) => {
                if 0 <= r && r < vec.len() && 0 <= c && c < vec[0].len() {
                    Go(&vec[r][c])
                } else {
                    No(GridErr(OutOfBounds((r, c), (vec.len(), vec[0].len()))))
                }
            }
        }
    }

    /// Mut ref to element in `r` row and `c` column
    pub(in super) fn att_mut(&mut self, r: usize, c: usize) -> AnyRes<&mut T> {
        match self {
            VecWrapper::Lin(vec) => {
                if r == 0 && 0 <= c && c < vec.len() {
                    Go(&mut vec[c])
                } else {
                    No(GridErr(OutOfBounds((r, c), (1, vec.len()))))
                }
            }
            VecWrapper::Rec(vec) => {
                if 0 <= r && r < vec.len() && 0 <= c && c < vec[0].len() {
                    Go(&mut vec[r][c])
                } else {
                    No(GridErr(OutOfBounds((r, c), (vec.len(), vec[0].len()))))
                }
            }
        }
    }
}


/// One- or two-dimensional `Vec` with transposing flag
#[derive(Debug, Clone, PartialEq)]
pub struct RawGrid<T> {
    vec: VecWrapper<T>,
    trans: bool,
}

impl<T> RawGrid<T> {
    /// Constructor for one-dimensional `Vec`, not transposed
    pub fn from_lin(lin: Vec<T>) -> AnyRes<Self> {
        let vec = VecWrapper::Lin(lin);
        match vec.is_valid() {
            Go(_) => Go(Self {
                vec,
                trans: false,
            }),
            No(err) => No(err),
        }
    }

    /// Constructor for two-dimensional `Vec`, not transposed
    pub fn from_rec(rec: Vec<Vec<T>>) -> AnyRes<Self> {
        let vec = VecWrapper::Rec(rec);
        match vec.is_valid() {
            Go(_) => Go(Self {
                vec,
                trans: false,
            }),
            No(err) => No(err),
        }
    }

    /// Transposes (inverses `trans` flag) and returns self
    pub fn transpose(mut self) -> Self {
        self.trans = !self.trans;
        self
    }

    /// Number of rows, accounting `self.trans` and passed `t` flag
    pub fn rows(&self, t: bool) -> usize {
        match self.trans ^ t {
            false => self.vec.rows(),
            true => self.vec.cols(),
        }
    }

    /// Number of columns, accounting `self.trans` and passed `t` flag
    pub fn cols(&self, t: bool) -> usize {
        match self.trans ^ t {
            false => self.vec.cols(),
            true => self.vec.rows(),
        }
    }

    /// Element in `r` row and `c` column, accounting `self.trans` and passed `t` flag
    pub fn att(&self, r: usize, c: usize, t: bool) -> AnyRes<&T> {
        match self.trans ^ t {
            false => self.vec.att(r, c),
            true => self.vec.att(c, r),
        }
    }

    /// Mut ref to element in `r` row and `c` column,
    /// accounting `self.trans` and passed `t` flag
    pub fn att_mut(&mut self, r: usize, c: usize, t: bool) -> AnyRes<&mut T> {
        match self.trans ^ t {
            false => self.vec.att_mut(r, c),
            true => self.vec.att_mut(c, r),
        }
    }
}
