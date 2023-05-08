use {
    crate::{
        errs::{
            AnyRes::{self, *},
            AnyErr::{self, *},
            GridErr::{self, *},
        }
    },
    super::raw_grid::{
        VecWrapper, RawGrid,
    },
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repr {
    Matrix,
    Row,
    Col,
    RowList,
    ColList,
    Failure,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Grid<T> {
    Matrix(RawGrid<T>),
    Row(RawGrid<T>),
    Col(RawGrid<T>),
    RowList(RawGrid<T>),
    ColList(RawGrid<T>),
    Failure(AnyErr),
}


impl<T: Clone> Grid<T> {
    /// Returns grid of the given size filled with the given T
    pub fn fill_with(r: usize, c: usize, with: T) -> Self {
        match r == 0 || c == 0 {
            false => {
                let (mut lin, mut rec) = (vec![], vec![]);
                lin.reserve(c);
                lin.fill(with);
                rec.reserve(r);
                rec.fill(lin);
                Self::Matrix(RawGrid::from_rec(rec).unwrap())
            }
            true => Self::Failure(GridErr(IsEmpty)),
        }
    }
}


impl<T> Grid<T> {
    pub fn from_lin(lin: Vec<T>) -> Self {
        match RawGrid::from_lin(lin) {
            Go(grid) => Self::Matrix(grid),
            No(err) => Self::Failure(err),
        }
    }

    pub fn from_rec(rec: Vec<Vec<T>>) -> Self {
        match RawGrid::from_rec(rec) {
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

    pub fn repr(&self) -> Repr {
        match self {
            Self::Matrix(_) => Repr::Matrix,
            Self::Row(_) => Repr::Row,
            Self::Col(_) => Repr::Col,
            Self::RowList(_) => Repr::RowList,
            Self::ColList(_) => Repr::ColList,
            Self::Failure(_) => Repr::Failure,
        }
    }

    pub fn to_matrix(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::Matrix(grid),
        }
    }

    pub fn to_row(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => {
                match grid.rows(false) {
                    1 => Self::Row(grid),
                    r => Self::Failure(GridErr(TooManyRows(r))),
                }
            }
        }
    }

    pub fn to_col(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => {
                match grid.cols(false) {
                    1 => Self::Col(grid),
                    c => Self::Failure(GridErr(TooManyCols(c))),
                }
            }
        }
    }

    pub fn to_rowlist(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::RowList(grid),
        }
    }

    pub fn to_collist(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Matrix(grid) |
            Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => Self::ColList(grid),
        }
    }

    pub fn into(self, repr: Repr) -> Self {
        match repr {
            Repr::Matrix => self.to_matrix(),
            Repr::Row => self.to_row(),
            Repr::Col => self.to_col(),
            Repr::RowList => self.to_rowlist(),
            Repr::ColList => self.to_collist(),
            Repr::Failure => unreachable!(),
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
            Self::Matrix(mut grid) => Self::Matrix(grid.transpose()),
            Self::RowList(mut grid) => Self::RowList(grid.transpose()),
            Self::ColList(mut grid) => Self::ColList(grid.transpose()),
            Self::Row(_) => Self::Failure(GridErr(Untransposable(Repr::Row))),
            Self::Col(_) => Self::Failure(GridErr(Untransposable(Repr::Col))),
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    /// Element in `i` position in `Row` / `Col`
    pub fn at(&self, i: usize) -> AnyRes<&T> {
        match self {
            Self::Row(grid) => {
                grid.att(0, i, false)
            }
            Self::Col(grid) => {
                grid.att(i, 0, false)
            }
            Self::Failure(_) => No(GridErr(UnhandledFailure)),
            _ => No(GridErr(IsNotLin(self.repr()))),
        }
    }

    /// Mut ref to element in `i` position in `Row` / `Col`
    pub fn at_mut(&mut self, i: usize) -> AnyRes<&mut T> {
        match self {
            Self::Row(grid) => {
                grid.att_mut(0, i, false)
            }
            Self::Col(grid) => {
                grid.att_mut(i, 0, false)
            }
            Self::Failure(_) => No(GridErr(UnhandledFailure)),
            _ => No(GridErr(IsNotLin(self.repr()))),
        }
    }

    /// Element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att(&self, i: usize, j: usize) -> AnyRes<&T> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att(i, j, false),
            Self::ColList(grid) => grid.att(j, i, false),
            Self::Failure(_) => No(GridErr(UnhandledFailure)),
            _ => No(GridErr(IsNotRec(self.repr()))),
        }
    }

    /// Mut ref to element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att_mut(&mut self, i: usize, j: usize) -> AnyRes<&mut T> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att_mut(i, j, false),
            Self::ColList(grid) => grid.att_mut(j, i, false),
            Self::Failure(_) => No(GridErr(UnhandledFailure)),
            _ => No(GridErr(IsNotRec(self.repr()))),
        }
    }

    pub fn grid(&self) -> &RawGrid<T> {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }

    pub fn grid_mut(&mut self) -> &mut RawGrid<T> {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }
}