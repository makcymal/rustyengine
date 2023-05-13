use {
    crate::{
        errs::{
            AnyRes,
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
    pub(in super) fn is_lin(&self) -> bool {
        match self {
            VecWrapper::Lin(_) => true,
            _ => false
        }
    }

    pub(in super) fn is_rec(&self) -> bool {
        match self {
            VecWrapper::Rec(_) => true,
            _ => false
        }
    }

    /// Whether it's empty and has shape or right rectangle
    pub(in super) fn is_valid(&self) -> AnyRes<()> {
        match self {
            VecWrapper::Lin(vec) => {
                if vec.len() == 0 {
                    Err(GridErr(IsEmpty))
                } else {
                    Ok(())
                }
            }
            VecWrapper::Rec(vec) => {
                if vec.len() == 0 {
                    Err(GridErr(IsEmpty))
                } else {
                    let mut cols = None;
                    for r in 0..vec.len() {
                        if cols.is_none() {
                            cols = Some(vec[r].len());
                        } else if vec[r].len() != cols.unwrap() {
                            return Err(GridErr(CurveSides(r)));
                        }
                    }
                    if cols.unwrap() == 0 {
                        Err(GridErr(IsEmpty))
                    } else {
                        Ok(())
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
    pub(in super) fn att(&self, r: usize, c: usize) -> &T {
        match self {
            VecWrapper::Lin(vec) => &vec[c],
            VecWrapper::Rec(vec) => &vec[r][c],
        }
    }

    /// Mut ref to element in `r` row and `c` column
    pub(in super) fn att_mut(&mut self, r: usize, c: usize) -> &mut T {
        match self {
            VecWrapper::Lin(vec) => &mut vec[c],
            VecWrapper::Rec(vec) => &mut vec[r][c],
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
            Ok(_) => Ok(Self {
                vec,
                trans: false,
            }),
            Err(err) => Err(err),
        }
    }

    /// Constructor for two-dimensional `Vec`, not transposed
    pub fn from_rec(rec: Vec<Vec<T>>) -> AnyRes<Self> {
        let vec = VecWrapper::Rec(rec);
        match vec.is_valid() {
            Ok(_) => Ok(Self {
                vec,
                trans: false,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn is_transposed(&self) -> bool {
        self.trans
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
            false => {
                match &self.vec {
                    VecWrapper::Lin(lin) => {
                        match r == 0 && c < lin.len() {
                            true => Ok(self.vec.att(r, c)),
                            false => Err(GridErr(OutOfBounds { size: (1, lin.len()), idx: (r, c) }))
                        }
                    },
                    VecWrapper::Rec(rec) => {
                        match r < rec.len() && c < rec[0].len() {
                            true => Ok(self.vec.att(r, c)),
                            false => Err(GridErr(OutOfBounds { size: (rec.len(), rec[0].len()), idx: (r, c) }))
                        }
                    }
                }
            },
            true => {
                match &self.vec {
                    VecWrapper::Lin(lin) => {
                        match c == 0 && r < lin.len() {
                            true => Ok(self.vec.att(c, r)),
                            false => Err(GridErr(OutOfBounds { size: (lin.len(), 1), idx: (r, c) }))
                        }
                    },
                    VecWrapper::Rec(rec) => {
                        match c < rec.len() && r < rec[0].len() {
                            true => Ok(self.vec.att(c, r)),
                            false => Err(GridErr(OutOfBounds { size: (rec[0].len(), rec.len()), idx: (r, c) }))
                        }
                    }
                }
            }
        }
    }

    /// Mut ref to element in `r` row and `c` column,
    /// accounting `self.trans` and passed `t` flag
    pub fn att_mut(&mut self, r: usize, c: usize, t: bool) -> AnyRes<&mut T> {
        match self.trans ^ t {
            false => {
                match &self.vec {
                    VecWrapper::Lin(lin) => {
                        match r == 0 && c < lin.len() {
                            true => Ok(self.vec.att_mut(r, c)),
                            false => Err(GridErr(OutOfBounds { size: (1, lin.len()), idx: (r, c) }))
                        }
                    },
                    VecWrapper::Rec(rec) => {
                        match r == rec.len() && c < rec[0].len() {
                            true => Ok(self.vec.att_mut(r, c)),
                            false => Err(GridErr(OutOfBounds { size: (rec.len(), rec[0].len()), idx: (r, c) }))
                        }
                    }
                }
            },
            true => {
                match &self.vec {
                    VecWrapper::Lin(lin) => {
                        match c == 0 && r < lin.len() {
                            true => Ok(self.vec.att_mut(c, r)),
                            false => Err(GridErr(OutOfBounds { size: (lin.len(), 1), idx: (c, r) }))
                        }
                    },
                    VecWrapper::Rec(rec) => {
                        match c == rec.len() && r < rec[0].len() {
                            true => Ok(self.vec.att_mut(c, r)),
                            false => Err(GridErr(OutOfBounds { size: (rec[0].len(), rec.len()), idx: (c, r) }))
                        }
                    }
                }
            }
        }
    }
}
