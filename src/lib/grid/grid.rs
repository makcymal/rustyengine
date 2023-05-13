use {
    crate::{
        util::Line,
        errs::{
            AnyRes,
            AnyErr::{self, *},
            GridErr::{self, *},
        },
    },
    super::raw_grid::{
        VecWrapper, RawGrid,
    },
    strum_macros::Display,
};


#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Repr {
    Matrix,
    Row,
    Col,
    RowList,
    ColList,
    Failure,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Grid<E> {
    Matrix(RawGrid<E>),
    Row(RawGrid<E>),
    Col(RawGrid<E>),
    RowList(RawGrid<E>),
    ColList(RawGrid<E>),
    Failure(AnyErr),
}


impl<E: Clone> Grid<E> {
    /// Returns grid of the given size filled with the given E
    pub fn fill_with(r: usize, c: usize, with: E) -> Self {
        match r == 0 || c == 0 {
            false => {
                let (mut lin, mut rec) = (vec![], vec![]);
                lin.resize(c, with);
                rec.resize(r, lin);
                Self::Matrix(RawGrid::from_rec(rec).unwrap())
            }
            true => Self::Failure(GridErr(IsEmpty)),
        }
    }
}


impl<'g, E> Grid<E> {
    pub fn from_lin(lin: Vec<E>) -> Self {
        match RawGrid::from_lin(lin) {
            Ok(grid) => Self::Matrix(grid),
            Err(err) => Self::Failure(err),
        }
    }

    pub fn from_rec(rec: Vec<Vec<E>>) -> Self {
        match RawGrid::from_rec(rec) {
            Ok(grid) => Self::Matrix(grid),
            Err(err) => Self::Failure(err),
        }
    }

    pub fn rawgrid(&self) -> &RawGrid<E> {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }

    pub fn rawgrid_mut(&mut self) -> &mut RawGrid<E> {
        match self {
            Self::Matrix(grid) | Self::Row(grid) | Self::Col(grid) |
            Self::RowList(grid) | Self::ColList(grid) => grid,
            _ => unreachable!(),
        }
    }

    pub fn is_transposed(&self) -> bool {
        self.rawgrid().is_transposed()
    }

    pub fn raw_transpose(mut self) -> Self {
        match self {
            Self::Matrix(mut grid) => Self::Matrix(grid.transpose()),
            Self::RowList(mut grid) => Self::RowList(grid.transpose()),
            Self::ColList(mut grid) => Self::ColList(grid.transpose()),
            Self::Row(_) => Self::Failure(GridErr(Untransposable(Repr::Row))),
            Self::Col(_) => Self::Failure(GridErr(Untransposable(Repr::Col))),
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    pub fn transpose(mut self) -> Self {
        match self {
            Self::Matrix(mut grid) => Self::Matrix(grid.transpose()),
            Self::RowList(mut grid) => Self::ColList(grid.transpose()),
            Self::ColList(mut grid) => Self::RowList(grid.transpose()),
            Self::Row(mut grid) => Self::Col(grid.transpose()),
            Self::Col(mut grid) => Self::Row(grid.transpose()),
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
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
        self.rawgrid().rows(false)
    }

    pub fn cols(&self) -> usize {
        self.rawgrid().cols(false)
    }

    /// Element in `i` position in `Row` / `Col`
    pub fn at(&self, i: usize) -> AnyRes<&E> {
        match self {
            Self::Row(grid) => {
                grid.att(0, i, false)
            }
            Self::Col(grid) => {
                grid.att(i, 0, false)
            }
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
            _ => Err(GridErr(IsNotLin(self.repr()))),
        }
    }

    /// Mut ref to element in `i` position in `Row` / `Col`
    pub fn at_mut(&mut self, i: usize) -> AnyRes<&mut E> {
        match self {
            Self::Row(grid) => {
                grid.att_mut(0, i, false)
            }
            Self::Col(grid) => {
                grid.att_mut(i, 0, false)
            }
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
            _ => Err(GridErr(IsNotLin(self.repr()))),
        }
    }

    /// Element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att(&self, i: usize, j: usize) -> AnyRes<&E> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att(i, j, false),
            Self::ColList(grid) => {
                match grid.att(j, i, false) {
                    Ok(val) => Ok(val),
                    Err(_) => Err(GridErr(OutOfBounds { size: (self.cols(), self.rows()), idx: (i, j) })),
                }
            }
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
            _ => Err(GridErr(IsNotRec(self.repr()))),
        }
    }

    /// Mut ref to element in `i` row and `j` column in `Matrix` / `RowList`
    /// or in `i` column and `j` row in `ColList` (or error)
    pub fn att_mut(&mut self, i: usize, j: usize) -> AnyRes<&mut E> {
        match self {
            Self::Matrix(grid) | Self::RowList(grid) => grid.att_mut(i, j, false),
            Self::ColList(grid) => grid.att_mut(j, i, false),
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
            _ => Err(GridErr(IsNotRec(self.repr()))),
        }
    }

    pub fn row_iter(&'g self) -> AnyRes<LineIter<'g, E>> {
        let repr = self.repr();
        match repr {
            Repr::RowList => Ok(LineIter {
                grid: self,
                line: Line::Row,
                curr: 0,
            }),
            _ => Err(GridErr(NotIterableByRows(repr)))
        }
    }

    pub fn col_iter(&'g self) -> AnyRes<LineIter<'g, E>> {
        let repr = self.repr();
        match repr {
            Repr::ColList => Ok(LineIter {
                grid: self,
                line: Line::Col,
                curr: 0,
            }),
            _ => Err(GridErr(NotIterableByCols(repr)))
        }
    }
}


// <<< Iterators

// 'g stands for grid, E stands for Element
#[derive(Debug, Clone, PartialEq)]
pub struct LineIter<'g, E> {
    grid: &'g Grid<E>,
    line: Line,
    curr: usize,
}

impl<'g, E> Iterator for LineIter<'g, E> {
    type Item = ElemIter<'g, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let item;
        match self.line {
            Line::Row => {
                item = match self.curr < self.grid.rows() {
                    true => Some(ElemIter {
                        grid: self.grid,
                        dir: Line::Row,
                        row: self.curr,
                        col: 0,
                    }),
                    false => None,
                };
            }
            Line::Col => {
                item = match self.curr < self.grid.cols() {
                    true => Some(ElemIter {
                        grid: self.grid,
                        dir: Line::Col,
                        row: 0,
                        col: self.curr,
                    }),
                    false => None,
                }
            }
        }
        self.curr += 1;
        item
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ElemIter<'g, E> {
    grid: &'g Grid<E>,
    dir: Line,
    row: usize,
    col: usize,
}

impl<'g, E> Iterator for ElemIter<'g, E> {
    type Item = &'g E;

    fn next(&mut self) -> Option<Self::Item> {
        let item;
        match self.dir {
            Line::Row => {
                item = match self.grid.att(self.row, self.col) {
                    Ok(item) => Some(item),
                    Err(_) => None,
                };
                self.col += 1
            },
            Line::Col => {
                item = match self.grid.att(self.col, self.row) {
                    Ok(item) => Some(item),
                    Err(_) => None,
                };
                self.row += 1
            },
        };
        item
    }
}

// Iterators >>>
