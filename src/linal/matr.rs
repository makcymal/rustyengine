use {
    super::enums::{
        Inner::{self, *},
        MatrRes::{self, *},
        MatrErr::{self, *},
    },
};

#[derive(Debug, Clone, PartialEq)]
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


#[derive(Debug, Clone, PartialEq)]
enum Matr {
    Matrix(Grid),
    Row(Grid),
    Col(Grid),
    RowList(Grid),
    ColList(Grid),
    Failure(MatrErr),
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
            false => Self::Matrix(Grid::from_rec(vec![vec![with; c]; r]).unwrap()),
            true => Self::Failure(EmptyInner),
        }
    }

    pub fn from_lin(lin: Vec<f64>) -> Self {
        match Grid::from_lin(lin) {
            Go(grid) => Self::Matrix(grid),
            No(err) => Self::Failure(err),
        }
    }

    pub fn from_rec(rec: Vec<Vec<f64>>) -> Self {
        match Grid::from_rec(rec) {
            Go(grid) => Self::Matrix(grid),
            No(err) => Self::Failure(err),
        }
    }

    pub fn is_matrix(&self) -> bool {
        match self {
            Self::Matrix(_) => true,
            _ => false,
        }
    }

    pub fn is_row(&self) -> bool {
        match self {
            Self::Row(_) => true,
            _ => false,
        }
    }

    pub fn is_col(&self) -> bool {
        match self {
            Self::Col(_) => true,
            _ => false,
        }
    }

    pub fn is_rowlist(&self) -> bool {
        match self {
            Self::RowList(_) => true,
            _ => false,
        }
    }

    pub fn is_collist(&self) -> bool {
        match self {
            Self::ColList(_) => true,
            _ => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match self {
            Self::Failure(_) => true,
            _ => false,
        }
    }

    pub fn to_matrix(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(UnhandledFailure),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::Matrix(grid),
        }
    }

    pub fn to_row(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(UnhandledFailure),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => {
                match grid.rows(false) == 1 {
                    true => Self::Row(grid),
                    false => Self::Failure(TooManyRows),
                }
            },
        }
    }

    pub fn to_col(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(UnhandledFailure),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => {
                match grid.cols(false) == 1 {
                    true => Self::Col(grid),
                    false => Self::Failure(TooManyCols),
                }
            },
        }
    }

    pub fn to_rowlist(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(UnhandledFailure),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::RowList(grid),
        }
    }

    pub fn to_collist(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(UnhandledFailure),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::ColList(grid),
        }
    }

    pub fn rows(&self) -> usize {
        self.grid().rows(false)
    }

    pub fn cols(&self) -> usize {
        self.grid().cols(false)
    }

    pub fn transpose(mut self) -> Self {
        match self {
            Self::Matrix(mut grid) => {
                grid.transpose();
                Self::Matrix(grid)
            },
            Self::RowList(mut grid) => {
                grid.transpose();
                Self::RowList(grid)
            },
            Self::ColList(mut grid) => {
                grid.transpose();
                Self::ColList(grid)
            },
            Self::Row(_) | Self::Col(_) => Self::Failure(Untransposable),
            Self::Failure(_) => Self::Failure(UnhandledFailure),
        }
    }

    /// Element in `i` position in `Row` / `Col`
    pub fn at(&self, i: usize) -> MatrRes<f64> {
        match self {
            Self::Row(grid) => {
                grid.att(0, i, false)
            }
            Self::Col(grid) => {
                grid.att(i, 0, false)
            }
            Self::Failure(_) => No(UnhandledFailure),
            _ => No(NotLin),
        }
    }

    /// Mut ref to element in `i` position in `Row` / `Col`
    pub fn at_mut(&mut self, i: usize) -> MatrRes<&mut f64> {
        match self {
            Self::Row(grid) => {
                grid.att_mut(0, i, false)
            }
            Self::Col(grid) => {
                grid.att_mut(i, 0, false)
            }
            Self::Failure(_) => No(UnhandledFailure),
            _ => No(NotLin),
        }
    }

    /// Element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att(&self, i: usize, j: usize) -> MatrRes<f64> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att(i, j, false),
            Self::ColList(grid) => grid.att(j, i, false),
            Self::Failure(_) => No(UnhandledFailure),
            _ => No(NotRec),
        }
    }

    /// Mut ref to element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att_mut(&mut self, i: usize, j: usize) -> MatrRes<&mut f64> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att_mut(i, j, false),
            Self::ColList(grid) => grid.att_mut(j, i, false),
            Self::Failure(_) => No(UnhandledFailure),
            _ => No(NotRec),
        }
    }

    pub fn add(&self, rhs: &Self, t: bool) -> Self {
        let mut output = self.clone();
        output.add_assign(rhs, t)
    }

    pub fn add_assign(mut self, rhs: &Self, t: bool) -> Self {
        let lhs_rows = self.grid().rows(false);
        let lhs_cols = self.grid().cols(false);

        if lhs_rows != rhs.grid().rows(t) || lhs_cols != rhs.grid().cols(t) {
            return Self::Failure(AddSizesMismatch);
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.grid_mut().att_mut(r, c, false).unwrap() += rhs.grid().att(r, c, t).unwrap();
            }
        }
        self
    }

    pub fn sub(&self, rhs: &Self, t: bool) -> Self {
        let mut output = self.clone();
        output.sub_assign(rhs, t)
    }

    pub fn sub_assign(mut self, rhs: &Self, t: bool) -> Self {
        let lhs_rows = self.grid().rows(false);
        let lhs_cols = self.grid().cols(false);

        if lhs_rows != rhs.grid().rows(t) || lhs_cols != rhs.grid().cols(t) {
            return Self::Failure(AddSizesMismatch);
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.grid_mut().att_mut(r, c, false).unwrap() -= rhs.grid().att(r, c, t).unwrap();
            }
        }
        self
    }

    pub fn mul(&self, rhs: &Self, t: bool) -> Self {
        let out_rows = self.grid().rows(false);
        let out_cols = rhs.grid().cols(t);

        if self.grid().cols(false) != rhs.grid().rows(t) {
            return Self::Failure(MulSizesMismatch);
        }

        let mut output = Self::zero(out_rows, out_cols);
        for r in 0..out_rows {
            for c in 0..out_cols {
                *output.att_mut(r, c).unwrap() =
                    (0..self.grid().cols(false))
                        .map(|i| self.att(r, i).unwrap() * rhs.att(i, c).unwrap())
                        .sum();
            }
        }
        output
    }

    fn grid(&self) -> &Grid {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }

    fn grid_mut(&mut self) -> &mut Grid {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }
}