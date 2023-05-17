use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
        },
    },
};


/// `VecWrapper` is a workaround to deal with `Vec<E>` as well as `Vec<Vec<E>>`
#[derive(Debug, Clone)]
pub(in super) enum VecWrapper<E> {
    /// `Vec<E>`
    Single(Vec<E>),
    /// Two-dimensional `Vec<Vec<E>>`
    Double(Vec<Vec<E>>),
}

impl<E> VecWrapper<E> {
    /// Whether it wraps `Vec<E>` or not
    pub(in super) fn is_single(&self) -> bool {
        match self {
            VecWrapper::Single(_) => true,
            _ => false
        }
    }

    /// Whether it wraps `Vec<Vec<E>>` or not
    pub(in super) fn is_double(&self) -> bool {
        match self {
            VecWrapper::Double(_) => true,
            _ => false
        }
    }

    /// Whether it isn't empty and has shape or right rectangle
    pub(in super) fn is_valid(&self) -> ReRes<()> {
        match self {
            VecWrapper::Single(vec) => {
                if vec.len() == 0 {
                    Err(GridErr(IsEmpty))
                } else {
                    Ok(())
                }
            }
            VecWrapper::Double(vec) => {
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
            VecWrapper::Single(_) => 1,
            VecWrapper::Double(vec) => vec.len(),
        }
    }

    /// Number of columns, under assumption vec isn't empty
    pub(in super) fn cols(&self) -> usize {
        match self {
            VecWrapper::Single(vec) => vec.len(),
            VecWrapper::Double(vec) => vec[0].len(),
        }
    }

    /// Element in `r` row and `c` column
    pub(in super) fn att(&self, r: usize, c: usize) -> &E {
        match self {
            VecWrapper::Single(vec) => &vec[c],
            VecWrapper::Double(vec) => &vec[r][c],
        }
    }

    /// Mut ref to element in `r` row and `c` column
    pub(in super) fn att_mut(&mut self, r: usize, c: usize) -> &mut E {
        match self {
            VecWrapper::Single(vec) => &mut vec[c],
            VecWrapper::Double(vec) => &mut vec[r][c],
        }
    }
}

impl<E: Clone> VecWrapper<E> {
    /// Increases or decreases number of rows or columns by extending or reducing filling with given value
    pub(in super) fn resize(mut self, r: usize, c: usize, with: E) -> Self {
        self = match self {
            Self::Single(mut single) => {
                single.resize(c, with.clone());
                Self::Single(single)
            }
            Self::Double(mut double) => {
                for row in &mut double {
                    row.resize(c, with.clone());
                }
                Self::Double(double)
            }
        };
        match self {
            Self::Single(mut single) => {
                if r > 1 {
                    let mut double = vec![single];
                    double.resize(r, vec![with.clone(); c]);
                    Self::Double(double)
                } else {
                    Self::Single(single)
                }
            }
            Self::Double(mut double) => {
                if r > 1 {
                    double.resize(r, vec![with.clone(); c]);
                    Self::Double(double)
                } else {
                    Self::Single(double.remove(0))
                }
            }
        }
    }
}

impl<E: Default + Clone> VecWrapper<E> {
    /// Increases or decreases number of rows or columns by extending or reducing filling with default
    pub(in super) fn resize_default(mut self, r: usize, c: usize) -> Self {
        self = match self {
            Self::Single(mut single) => {
                single.resize(c, E::default());
                Self::Single(single)
            }
            Self::Double(mut double) => {
                for row in &mut double {
                    row.resize(c, E::default());
                }
                Self::Double(double)
            }
        };
        match self {
            Self::Single(mut single) => {
                if r > 1 {
                    let mut double = vec![single];
                    double.resize(r, vec![E::default(); c]);
                    Self::Double(double)
                } else {
                    Self::Single(single)
                }
            }
            Self::Double(mut double) => {
                if r > 1 {
                    double.resize(r, vec![E::default(); c]);
                    Self::Double(double)
                } else {
                    Self::Single(double.remove(0))
                }
            }
        }
    }
}


/// `RawGrid` is easy-transposable `VecWrapper`
#[derive(Debug, Clone)]
pub struct RawGrid<E> {
    vec: VecWrapper<E>,
    trans: bool,
}

impl<E> RawGrid<E> {
    /// Constructor for one-dimensional `Vec<E>`, not transposed
    pub fn from_single(single: Vec<E>) -> ReRes<Self> {
        let vec = VecWrapper::Single(single);
        match vec.is_valid() {
            Ok(_) => Ok(Self {
                vec,
                trans: false,
            }),
            Err(err) => Err(err),
        }
    }

    /// Constructor for two-dimensional `Vec<Vec<E>>`, not transposed
    pub fn from_double(double: Vec<Vec<E>>) -> ReRes<Self> {
        let vec = VecWrapper::Double(double);
        match vec.is_valid() {
            Ok(_) => Ok(Self {
                vec,
                trans: false,
            }),
            Err(err) => Err(err),
        }
    }

    /// Public access to field `self.trans`
    pub fn is_transposed(&self) -> bool {
        self.trans
    }

    /// Transposes (inverses `self.trans` flag) and returns self
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

    /// Whether `self` element-wise equals to `other` treating the given predicate.
    /// Predicate should answer the same quastion: whether elements are equal
    pub fn eq(&self, other: &Self, p: impl Fn(&E, &E) -> bool) -> bool {
        for r in 0..self.rows(false) {
            for c in 0..self.cols(false) {
                if !p(self.att(r, c, false), other.att(r, c, false)) {
                    return false
                }
            }
        }
        true
    }
}

impl<E: Clone> RawGrid<E> {
    /// Appends given set of rows at the tail
    pub fn append_rows(mut self, tail: Self, t: bool) -> ReRes<Self> {
        if self.cols(false) != tail.cols(t) {
            return Err(GridErr(RowsAppendMismatch { dest: self.cols(false), tail: tail.cols(t) }));
        }
        let (ex_rows, new_rows, cols) = (self.rows(false), tail.rows(t), self.cols(false));
        self.vec = match self.trans {
            false => self.vec.resize(ex_rows + new_rows, cols, tail.att(0, 0, false).clone()),
            true => self.vec.resize(ex_rows, cols + new_rows, tail.att(0, 0, false).clone()),
        };
        for r in 0..new_rows {
            let er = ex_rows + r;
            for c in 0..cols {
                *self.att_mut(er, c, false) = tail.att(r, c, t).clone();
            }
        }
        Ok(self)
    }

    /// Appends given set of cols at the tail
    pub fn append_cols(mut self, tail: Self, t: bool) -> ReRes<Self> {
        if self.rows(false) != tail.rows(t) {
            return Err(GridErr(ColsAppendMismatch { dest: self.rows(false), tail: tail.rows(t) }));
        }
        let (ex_cols, new_cols, rows) = (self.cols(false), tail.cols(t), self.rows(false));
        self.vec = match self.trans {
            false => self.vec.resize(rows, ex_cols + new_cols, tail.att(0, 0, false).clone()),
            true => self.vec.resize(rows + new_cols, ex_cols, tail.att(0, 0, false).clone()),
        };
        for c in 0..new_cols {
            let ec = ex_cols + c;
            for r in 0..rows {
                *self.att_mut(r, ec, false) = tail.att(r, c, t).clone();
            }
        }
        Ok(self)
    }
}
