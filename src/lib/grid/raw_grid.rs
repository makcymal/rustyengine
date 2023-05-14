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
#[derive(Debug, Clone)]
pub(in super) enum VecWrapper<E> {
    /// One-dimensional `Vec`
    Lin(Vec<E>),
    /// Two-dimensional `Vec`
    Rec(Vec<Vec<E>>),
}

impl<E> VecWrapper<E> {
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
    pub(in super) fn att(&self, r: usize, c: usize) -> &E {
        match self {
            VecWrapper::Lin(vec) => &vec[c],
            VecWrapper::Rec(vec) => &vec[r][c],
        }
    }

    /// Mut ref to element in `r` row and `c` column
    pub(in super) fn att_mut(&mut self, r: usize, c: usize) -> &mut E {
        match self {
            VecWrapper::Lin(vec) => &mut vec[c],
            VecWrapper::Rec(vec) => &mut vec[r][c],
        }
    }
}


/// One- or two-dimensional `Vec` with transposing flag
#[derive(Debug, Clone)]
pub struct RawGrid<E> {
    vec: VecWrapper<E>,
    trans: bool,
}

impl<E> RawGrid<E> {
    /// Constructor for one-dimensional `Vec`, not transposed
    pub fn from_lin(lin: Vec<E>) -> AnyRes<Self> {
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
    pub fn from_rec(rec: Vec<Vec<E>>) -> AnyRes<Self> {
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
    pub fn att(&self, r: usize, c: usize, t: bool) -> &E {
        match self.trans ^ t {
            false => self.vec.att(r, c),
            true => self.vec.att(c, r)
        }
    }

    /// Mut ref to element in `r` row and `c` column,
    /// accounting `self.trans` and passed `t` flag
    pub fn att_mut(&mut self, r: usize, c: usize, t: bool) -> &mut E {
        match self.trans ^ t {
            false => self.vec.att_mut(r, c),
            true => self.vec.att_mut(c, r)
        }
    }
}

impl<E: PartialEq> PartialEq for RawGrid<E> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..self.rows(false) {
            for c in 0..self.cols(false) {
                if self.att(r, c, false) != other.att(r, c, false) {
                    return false;
                }
            }
        }
        true
    }
}
