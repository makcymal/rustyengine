use {
    super::enums::{
        Inner::{self, *},
        Repr::{self, *},
        MatrRes::{self, *},
        MatrErr::{self, *},
    },
};

struct Grid {
    inner: Inner,
    trans: bool,
}

impl Grid {
    fn from_lin(lin: Vec<f64>) -> MatrRes<Self> {
        let inner = Lin(lin);
        match inner.is_valid() {
            Go(_) => Go(Self {
                inner,
                trans: false,
            }),
            No(err) => No(err),
        }
    }

    fn from_rec(rec: Vec<Vec<f64>>) -> MatrRes<Self> {
        let inner = Rec(rec);
        match inner.is_valid() {
            Go(_) => Go(Self {
                inner,
                trans: false,
            }),
            No(err) => No(err),
        }
    }

    fn transpose(&mut self) {
        self.trans = !self.trans;
    }

    fn rows(&self, t: bool) -> usize {
        match self.trans ^ t {
            false => self.inner.rows(),
            true => self.inner.cols(),
        }
    }

    fn cols(&self, t: bool) -> usize {
        match self.trans ^ t {
            false => self.inner.cols(),
            true => self.inner.rows(),
        }
    }

    fn att(&self, r: usize, c: usize, t: bool) -> MatrRes<f64> {
        match self.trans ^ t {
            false => self.inner.att(r, c),
            true => self.inner.att(c, r),
        }
    }

    fn att_mut(&mut self, r: usize, c: usize, t: bool) -> MatrRes<&mut f64> {
        match self.trans ^ t {
            false => self.inner.att_mut(r, c),
            true => self.inner.att_mut(c, r),
        }
    }
}


// todo: replace Matr with
// enum Matr {
//     Matrix(Grid),
//     Row(Grid),
//     Col(Grid),
//     RowList(Grid),
//     ColList(Grid),
//     Failure(MatrErr),
// }


struct Matr {
    grid: Grid,
    repr: Repr,
}

impl Matr {
    /// Returns diag(1..1)
    pub fn identity(side: usize) -> Self {
        let mut id = Self::zero(side, side);
        // id now is surely non-empty and square
        for d in 0..side {
            *id.att_mut(d, d).unwrap() = 1.0;
        }
        id
    }

    /// Returns matrixify of the given size filled with zeros
    pub fn zero(r: usize, c: usize) -> Self {
        Self::fill_with(r, c, 0.0)
    }

    /// Returns matrixify of the given size filled with the given f64;
    pub fn fill_with(r: usize, c: usize, with: f64) -> Self {
        match r == 0 || c == 0 {
            false => Self {
                grid: Grid::from_rec(vec![vec![with; c]; r]).unwrap(),
                repr: Matrix,
            },
            true => Self {
                grid: Grid::from_rec(vec![vec![]]).unwrap(),
                repr: Failure(EmptyInner),
            },
        }
    }

    pub fn from_lin(lin: Vec<f64>) -> Self {
        match Grid::from_lin(lin) {
            Go(grid) => Self {
                grid,
                repr: Matrix,
            },
            No(err) => Self {
                grid: Grid { inner: Lin(vec![]), trans: false },
                repr: Failure(err),
            },
        }
    }

    pub fn from_rec(rec: Vec<Vec<f64>>) -> Self {
        match Grid::from_rec(rec) {
            Go(grid) => Self {
                grid,
                repr: Matrix,
            },
            No(err) => Self {
                grid: Grid { inner: Rec(vec![vec![]]), trans: false },
                repr: Failure(err),
            },
        }
    }

    pub fn repr(&self) -> Repr {
        self.repr
    }

    pub fn rows(&self) -> usize {
        self.grid.rows(false)
    }

    pub fn cols(&self) -> usize {
        self.grid.cols(false)
    }

    pub fn transpose(mut self) -> Self {
        match self.repr {
            Row | Col => self.repr = Failure(Untransposable),
            Failure(_) => self.repr = Failure(UnhandledFailure),
            _ => self.grid.transpose(),
        }
        self
    }

    pub fn switch(mut self, into: Repr) -> Self {
        if let Failure(err) = self.repr {
            self.repr = Failure(UnhandledFailure);
            return self;
        }
        match into {
            Row => {
                if self.grid.rows(false) != 1 {
                    self.repr = Failure(NotLin);
                    return self;
                }
            }
            Col => {
                if self.grid.cols(false) != 1 {
                    self.repr = Failure(NotLin);
                    return self;
                }
            }
            Failure(_) => unreachable!(),
            _ => (),
        }
        self.repr = into;
        self
    }

    /// Element in `i` position in `Row` / `Col`
    pub fn at(&self, i: usize) -> MatrRes<f64> {
        match self.repr {
            Row => {
                self.grid.att(0, i, false)
            }
            Col => {
                self.grid.att(i, 0, false)
            }
            Failure(_) => No(UnhandledFailure),
            _ => No(NotLin),
        }
    }

    /// Mut ref to element in `i` position in `Row` / `Col`
    pub fn at_mut(&mut self, i: usize) -> MatrRes<&mut f64> {
        match self.repr {
            Row => {
                self.grid.att_mut(0, i, false)
            }
            Col => {
                self.grid.att_mut(i, 0, false)
            }
            Failure(_) => No(UnhandledFailure),
            _ => No(NotLin),
        }
    }

    /// Element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att(&self, i: usize, j: usize) -> MatrRes<f64> {
        match self.repr {
            Matrix | RowList => self.grid.att(i, j, false),
            ColList => self.grid.att(j, i, false),
            Failure(_) => No(UnhandledFailure),
            _ => No(NotRec),
        }
    }

    /// Mut ref to element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att_mut(&mut self, i: usize, j: usize) -> MatrRes<&mut f64> {
        match self.repr {
            Matrix | RowList => self.grid.att_mut(i, j, false),
            ColList => self.grid.att_mut(j, i, false),
            Failure(_) => No(UnhandledFailure),
            _ => No(NotRec),
        }
    }

    pub fn add_assign(mut self, rhs: &Self, t: bool) -> Self {
        if self.grid.rows(false) != rhs.grid.rows(t) ||
            self.grid.cols(false) != rhs.grid.cols(t)
        {
            self.repr = Failure(AddSizesMismatch);
            return self;
        }
        for r in 0..self.grid.rows(false) {
            for c in 0..self.grid.cols(false) {
                *self.grid.att_mut(r, c, false).unwrap() +=
                    rhs.grid.att(r, c, t).unwrap();
            }
        }
        self
    }
}