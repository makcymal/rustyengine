use {
    std::ops::{Index, IndexMut},
    crate::{
        util::{
            LineTp,
            Idx,
        },
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
        },
    },
    super::raw_grid::{
        VecWrapper, RawGrid,
    },
    strum_macros::Display,
};


/// How `RawGrid` can be treaten
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Repr {
    /// Arbitrary `RawGrid`.
    /// Indexing through rows first, then columns
    Arbitrary,
    /// `RawGrid` with equal numbers of rows and columns.
    /// Avoids checks like `self.rows() == self.cols()`.
    /// Indexing through rows first, then columns
    Square,
    /// `RawGrid` with `rows() == 1`.
    /// It provides all the `Arbitrary` features, but also can treat as horizontal vector.
    /// Indexing through elements
    Row,
    /// `RawGrid` with `cols() == 1`.
    /// It provides all the `Arbitrary` features, but also can treat as vertical vector.
    /// Indexing through elements
    Col,
    /// Treat `RawGrid` as set of rows.
    /// Indexing through rows first, then columns
    MultiRow,
    /// Treat `RawGrid` as set of columns.
    /// Indexing through columns first, then rows
    MultiCol,
    /// `RawGrid` can be replaced with error message
    Failure,
}


/// `Grid` holds collection of elements of type `E`, structured in rectangular table.
/// The same variants of treating `RawGrid` as the `Repr`
#[derive(Debug, Clone, PartialEq)]
pub enum Grid<E> {
    Arbitrary(RawGrid<E>),
    Square(RawGrid<E>),
    Row(RawGrid<E>),
    Col(RawGrid<E>),
    MultiRow(RawGrid<E>),
    MultiCol(RawGrid<E>),
    Failure(ReErr),
}


impl<E: Clone> Grid<E> {
    /// Returns grid of the given size filled with the given E
    pub fn fill(r: usize, c: usize, with: E) -> Self {
        match r == 0 || c == 0 {
            false => {
                let (mut lin, mut rec) = (vec![], vec![]);
                lin.resize(c, with);
                rec.resize(r, lin);
                Self::Arbitrary(RawGrid::from_double(rec).unwrap())
            }
            true => Self::Failure(GridErr(IsEmpty)),
        }
    }
}

impl<'g, E> Grid<E> {
    /// Constructor for single `Vec<E>`, not transposed, with `Arbitrary` representation
    pub fn from_single(single: Vec<E>) -> Self {
        match RawGrid::from_single(single) {
            Ok(grid) => Self::Arbitrary(grid),
            Err(err) => Self::Failure(err),
        }
    }

    /// Constructor for double `Vec<Vec<E>>`, not transposed, with `Arbitrary` representation
    pub fn from_double(double: Vec<Vec<E>>) -> Self {
        match RawGrid::from_double(double) {
            Ok(grid) => Self::Arbitrary(grid),
            Err(err) => Self::Failure(err),
        }
    }

    /// `RawGrid` moved out from `self`
    pub fn rawgrid(self) -> RawGrid<E> {
        match self {
            Self::Arbitrary(rg) | Self::Square(rg) | Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => rg,
            _ => unreachable!(),
        }
    }

    /// Ref to `RawGrid` wrapped in any representation except `Self::Failure`
    pub fn rawgrid_ref(&self) -> &RawGrid<E> {
        match self {
            Self::Arbitrary(rg) | Self::Square(rg) | Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => rg,
            _ => unreachable!(),
        }
    }

    /// Mut ref to `RawGrid` wrapped in any representation except `Self::Failure`
    pub fn rawgrid_mut(&mut self) -> &mut RawGrid<E> {
        match self {
            Self::Arbitrary(rg) | Self::Square(rg) | Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => rg,
            _ => unreachable!(),
        }
    }

    /// Whether flag `trans` in `RawGrid` is `true`
    pub fn is_transposed(&self) -> bool {
        self.rawgrid_ref().is_transposed()
    }

    /// Transposes only `RawGrid` without switching between `Row` and `Col` or `MultiRow` and `MultiCol`
    pub fn raw_transpose(mut self) -> Self {
        match self {
            Self::Arbitrary(mut grid) => Self::Arbitrary(grid.transpose()),
            Self::Square(mut grid) => Self::Square(grid.transpose()),
            Self::MultiRow(mut grid) => Self::MultiRow(grid.transpose()),
            Self::MultiCol(mut grid) => Self::MultiCol(grid.transpose()),
            Self::Row(_) => Self::Failure(GridErr(Untransposable(Repr::Row))),
            Self::Col(_) => Self::Failure(GridErr(Untransposable(Repr::Col))),
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    /// Transposes only `RawGrid` switching between `Row` and `Col` or `MultiRow` and `MultiCol`
    pub fn transpose(mut self) -> Self {
        match self {
            Self::Arbitrary(mut grid) => Self::Arbitrary(grid.transpose()),
            Self::Square(mut grid) => Self::Square(grid.transpose()),
            Self::MultiRow(mut grid) => Self::MultiCol(grid.transpose()),
            Self::MultiCol(mut grid) => Self::MultiRow(grid.transpose()),
            Self::Row(mut grid) => Self::Col(grid.transpose()),
            Self::Col(mut grid) => Self::Row(grid.transpose()),
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    /// Whether `RawGrid` is represented as `Arbitrary`
    pub fn is_arbitrary(&self) -> bool {
        match self {
            Self::Arbitrary(_) => true,
            _ => false,
        }
    }

    /// Whether `RawGrid` is represented as `Square` or `self.rows() == self.cols()`
    pub fn is_square(&self) -> bool {
        match self {
            Self::Square(_) => true,
            Self::Failure(_) => false,
            _ => self.rows() == self.cols(),
        }
    }

    /// Whether `RawGrid` is represented as `Row`
    pub fn is_row(&self) -> bool {
        match self {
            Self::Row(_) => true,
            _ => false,
        }
    }

    /// Whether `RawGrid` is represented as `Col`
    pub fn is_col(&self) -> bool {
        match self {
            Self::Col(_) => true,
            _ => false,
        }
    }

    /// Whether `RawGrid` is represented as `MultiRow`
    pub fn is_multirow(&self) -> bool {
        match self {
            Self::MultiRow(_) => true,
            _ => false,
        }
    }

    /// Whether `RawGrid` is represented as `MultiCol`
    pub fn is_multicol(&self) -> bool {
        match self {
            Self::MultiCol(_) => true,
            _ => false,
        }
    }

    /// Whether `RawGrid` is represented as `Failure`
    pub fn is_failure(&self) -> bool {
        match self {
            Self::Failure(_) => true,
            _ => false,
        }
    }

    /// `Repr` of how `RawGrid` is treaten
    pub fn repr(&self) -> Repr {
        match self {
            Self::Arbitrary(_) => Repr::Arbitrary,
            Self::Square(_) => Repr::Square,
            Self::Row(_) => Repr::Row,
            Self::Col(_) => Repr::Col,
            Self::MultiRow(_) => Repr::MultiRow,
            Self::MultiCol(_) => Repr::MultiCol,
            Self::Failure(_) => Repr::Failure,
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_arbitrary(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Arbitrary(rg) | Self::Square(rg) |
            Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => Self::Arbitrary(rg),
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_square(self) -> Self {
        if self.rows() == self.cols() {
            match self {
                Self::Arbitrary(rg) | Self::Square(rg) |
                Self::Row(rg) | Self::Col(rg) |
                Self::MultiRow(rg) | Self::MultiCol(rg) => Self::Square(rg),
                _ => Self::Failure(GridErr(UnhandledFailure)),
            }
        } else {
            Self::Failure(GridErr(NotSquare((self.rows(), self.cols()))))
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_row(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Arbitrary(rg) | Self::Square(rg) |
            Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => {
                match rg.rows(false) {
                    1 => Self::Row(rg),
                    r => Self::Failure(GridErr(TooManyRows(r))),
                }
            }
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_col(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Arbitrary(rg) | Self::Square(rg) |
            Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => {
                match rg.cols(false) {
                    1 => Self::Col(rg),
                    c => Self::Failure(GridErr(TooManyCols(c))),
                }
            }
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_multirow(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Arbitrary(rg) | Self::Square(rg) |
            Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => Self::MultiRow(rg),
        }
    }

    /// Trying to convert, if fails then returns `Self::Failure` with relevant error
    pub fn to_multicol(self) -> Self {
        match self {
            Self::Failure(_) => Self::Failure(GridErr(UnhandledFailure)),
            Self::Arbitrary(rg) | Self::Square(rg) |
            Self::Row(rg) | Self::Col(rg) |
            Self::MultiRow(rg) | Self::MultiCol(rg) => Self::MultiCol(rg),
        }
    }

    /// Trying to convert into given `Repr`, if fails then returns `Self::Failure` with relevant error
    pub fn into(self, repr: Repr) -> Self {
        match repr {
            Repr::Arbitrary => self.to_arbitrary(),
            Repr::Square => self.to_square(),
            Repr::Row => self.to_row(),
            Repr::Col => self.to_col(),
            Repr::MultiRow => self.to_multirow(),
            Repr::MultiCol => self.to_multicol(),
            Repr::Failure => Self::Failure(GridErr(ConvertedToFailure)),
        }
    }

    pub fn rows(&self) -> usize {
        self.rawgrid_ref().rows(false)
    }

    pub fn cols(&self) -> usize {
        self.rawgrid_ref().cols(false)
    }

    /// Ref to element in `i` position in `Row` / `Col`
    pub fn at(&self, i: usize) -> &E {
        match self {
            Self::Row(rg) => rg.att(0, i, false),
            Self::Col(rg) => rg.att(i, 0, false),
            Self::Failure(err) => panic!("calling at({:?}) on Failure({:?})", i, err),
            _ => panic!("calling at({:?}) on {:?}", i, self.repr()),
        }
    }

    /// Mut ref to element in `i` position in `Row` / `Col`
    pub fn at_mut(&mut self, i: usize) -> &mut E {
        match self {
            Self::Row(rg) => rg.att_mut(0, i, false),
            Self::Col(rg) => rg.att_mut(i, 0, false),
            Self::Failure(err) => panic!("calling at({:?}) on Failure({:?})", i, err),
            _ => panic!("calling at({:?}) on {:?}", i, self.repr()),
        }
    }

    /// Ref to element in `i` row and `j` column in `Matrix` or `Square` or `MultiRow`,
    /// in `i` column and `j` row in `MultiCol`, in `j` column in `Row`, in `i` row in `Col`
    pub fn att(&self, i: usize, j: usize) -> &E {
        match self {
            Self::Arbitrary(rg) | Self::Square(rg) | Self::MultiRow(rg) => rg.att(i, j, false),
            Self::MultiCol(rg) => rg.att(j, i, false),
            Self::Row(rg) => rg.att(0, j, false),
            Self::Col(rg) => rg.att(i, 0, false),
            Self::Failure(err) => panic!("calling at({:?}, {:?}) on Failure({:?})", i, j, err),
            _ => panic!("calling at({:?}, {:?}) on {:?}", i, j, self.repr()),
        }
    }

    /// Mut ref to element in `i` row and `j` column in `Matrix` or `Square` or `MultiRow`
    /// or in `i` column and `j` row in `MultiCol`, in `j` column in `Row`, in `i` row in `Col`
    pub fn att_mut(&mut self, i: usize, j: usize) -> &mut E {
        match self {
            Self::Arbitrary(rg) | Self::Square(rg) | Self::MultiRow(rg) => rg.att_mut(i, j, false),
            Self::MultiCol(rg) => rg.att_mut(j, i, false),
            Self::Row(rg) => rg.att_mut(0, j, false),
            Self::Col(rg) => rg.att_mut(i, 0, false),
            Self::Failure(err) => panic!("calling at({:?}, {:?}) on Failure({:?})", i, j, err),
            _ => panic!("calling at({:?}, {:?}) on {:?}", i, j, self.repr()),
        }
    }

    /// Constructs `Line` that implements `Iterator`. Calling `next()` makes step to next line.
    /// `Line` is specified to be `Row` or `Col` on the basis of `self.repr()`
    pub fn iter(&'g self) -> ReRes<Line<'g, E>> {
        self.ag_failed()?.ag_not_stratified()?;
        match self {
            Self::Row(_) | Self::MultiRow(_) | Self::Col(_) | Self::MultiCol(_) => Ok(Line {
                grid: self,
                curr: 0,
            }),
            _ => unreachable!()
        }
    }

    /// Whether both operands aren't represented as `Failure`
    pub fn approve_ops(&self, other: &Self) -> ReRes<()> {
        self.ag_failed()?;
        other.ag_failed()?;
        Ok(())
    }
}

// Rows and cols appending
impl<E: Clone> Grid<E> {
    /// Appends given set of rows at the tail
    pub fn append_rows(mut self, mut tail: Self) -> ReRes<Self> {
        self.approve_ops(&tail)?;
        match self.rawgrid().append_rows(tail.rawgrid(), false) {
            Ok(rg) => Ok(Self::MultiRow(rg)),
            Err(err) => Err(err),
        }
    }

    /// Appends given set of cols at the tail
    pub fn append_cols(mut self, mut tail: Self) -> ReRes<Self> {
        self.approve_ops(&tail)?;
        match self.rawgrid().append_cols(tail.rawgrid(), false) {
            Ok(rg) => Ok(Self::MultiCol(rg)),
            Err(err) => Err(err),
        }
    }
}

/// Checks against some conditions that can be chained
impl<E> Grid<E> {
    pub fn ag_single_indexed(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Row | Repr::Col => Err(GridErr(SingleIndexed(self.repr()))),
            _ => Ok(self)
        }
    }

    pub fn ag_double_indexed(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Row | Repr::Col => Ok(self),
            _ => Err(GridErr(DoubleIndexed(self.repr()))),
        }
    }

    pub fn ag_failed(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Failure => Err(GridErr(UnhandledFailure)),
            _ => Ok(self),
        }
    }

    pub fn ag_untransposable(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Row | Repr::Col => Err(GridErr(Untransposable(self.repr()))),
            _ => Ok(self),
        }
    }

    pub fn ag_not_row_or_col(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Row | Repr::Col => Ok(self),
            _ => Err(GridErr(NotRowOrCol)),
        }
    }

    pub fn ag_too_many_rows(&self) -> ReRes<&Self> {
        match self.rows() {
            1 => Ok(self),
            r => Err(GridErr(TooManyRows(r)))
        }
    }

    pub fn ag_too_many_cols(&self) -> ReRes<&Self> {
        match self.cols() {
            1 => Ok(self),
            c => Err(GridErr(TooManyCols(c)))
        }
    }

    pub fn ag_not_multi_row_or_col(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::MultiRow | Repr::MultiCol => Ok(self),
            _ => Err(GridErr(NotMultiRowOrCol))
        }
    }

    pub fn ag_not_stratified(&self) -> ReRes<&Self> {
        match self.repr() {
            Repr::Row | Repr::MultiRow | Repr::Col | Repr::MultiCol => Ok(self),
            _ => Err(GridErr(NotMultiRowOrCol))
        }
    }

    pub fn ag_not_square(&self) -> ReRes<&Self> {
        match self.rows() == self.cols() {
            true => Ok(self),
            false => Err(GridErr(NotSquare((self.rows(), self.cols()))))
        }
    }
}


impl<E> Index<Idx> for Grid<E> {
    type Output = E;

    fn index(&self, idx: Idx) -> &Self::Output {
        match self {
            Self::Failure(err) => panic!("indexing into Grid::Failure({:?})", err),
            Self::Row(_) | Self::Col(_) => {
                match idx {
                    Idx::Single(idx) => self.at(idx),
                    Idx::Double(_) => panic!("rectangular indexing into {:?}", self.repr()),
                }
            }
            Self::Arbitrary(_) | Self::Square(_) | Self::MultiRow(_) | Self::MultiCol(_) => {
                match idx {
                    Idx::Double(idx) => self.att(idx.0, idx.1),
                    Idx::Single(_) => panic!("linear indexing into {:?}", self.repr()),
                }
            }
        }
    }
}

impl<E> IndexMut<Idx> for Grid<E> {
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        match self {
            Self::Failure(err) => panic!("indexing into Grid::Failure({:?})", err),
            Self::Row(_) | Self::Col(_) => {
                match idx {
                    Idx::Single(idx) => self.at_mut(idx),
                    Idx::Double(_) => panic!("rectangular indexing into {:?}", self.repr()),
                }
            }
            Self::Arbitrary(_) | Self::Square(_) | Self::MultiRow(_) | Self::MultiCol(_) => {
                match idx {
                    Idx::Double(idx) => self.att_mut(idx.0, idx.1),
                    Idx::Single(_) => panic!("linear indexing into {:?}", self.repr()),
                }
            }
        }
    }
}


// <<< Iterators

/// Wrapper on particular `Row` or `Col` in `Grid`, that is specified on the basis of `grid.repr()`
#[derive(Debug, Clone, PartialEq)]
pub struct Line<'g, E> {
    grid: &'g Grid<E>,
    curr: usize,
}

impl<'g, E> Line<'g, E> {
    /// Constructor
    pub fn new(grid: &'g Grid<E>, curr: usize) -> Self {
        Self { grid, curr }
    }

    /// Provides indexing in particular line of `Grid`
    pub fn at(&self, idx: usize) -> &E {
        self.grid.att(self.curr, idx)
    }
}

/// Calling `next()` makes step to next line
impl<'g, E> Iterator for Line<'g, E> {
    type Item = Elem<'g, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let item =
            if self.grid.repr() == Repr::Row || self.grid.repr() == Repr::MultiRow {
                match self.curr < self.grid.rows() {
                    true => Some(Elem {
                        grid: self.grid,
                        row: self.curr,
                        col: 0,
                    }),
                    false => None,
                }
            } else if self.grid.repr() == Repr::Col || self.grid.repr() == Repr::MultiCol {
                match self.curr < self.grid.cols() {
                    true => Some(Elem {
                        grid: self.grid,
                        row: 0,
                        col: self.curr,
                    }),
                    false => None,
                }
            } else {
                None
            };
        self.curr += 1;
        item
    }
}

impl<'g, E> Index<usize> for Line<'g, E> {
    type Output = E;

    fn index(&self, idx: usize) -> &Self::Output {
        self.grid.att(self.curr, idx)
    }
}


/// Wrapper on particular element of `Grid`
#[derive(Debug, Clone, PartialEq)]
pub struct Elem<'g, E> {
    grid: &'g Grid<E>,
    row: usize,
    col: usize,
}

impl<'g, E> Elem<'g, E> {
    /// Value of current element
    pub fn get(&self) -> &E {
        self.grid.att(self.row, self.col)
    }
}

/// Calling `next()` makes step to next element in the line,
/// specified with `grid.repr()` and `self.row` or `self.col`
impl<'g, E> Iterator for Elem<'g, E> {
    type Item = &'g E;

    fn next(&mut self) -> Option<Self::Item> {
        let item =
            if self.grid.repr() == Repr::Row || self.grid.repr() == Repr::MultiRow {
                self.col += 1;
                match self.col - 1 < self.grid.cols() {
                    true => Some(self.grid.att(self.row, self.col - 1)),
                    false => None
                }
            } else if self.grid.repr() == Repr::Col {
                self.row += 1;
                match self.row - 1 < self.grid.rows() {
                    true => Some(self.grid.at(self.row - 1)),
                    false => None
                }
            } else if self.grid.repr() == Repr::MultiCol {
                self.row += 1;
                match self.row - 1 < self.grid.rows() {
                    true => Some(self.grid.att(self.col, self.row - 1)),
                    false => None
                }
            } else {
                None
            };
        item
    }
}

// Iterators >>>
