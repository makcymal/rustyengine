use {
    crate::{
        enums::{
            MatrErr::{self, *},
            MatrRes::{self, *},
        },
    },
};


/// Provides possibity of picking a suitable storage of elements;
#[derive(Debug, Clone)]
pub enum Inner {
    /// One-dimension vector;
    Solo(Vec<f64>),
    /// Two-dimension matrix;
    Duet(Vec<Vec<f64>>),
}

impl Inner {
    /// Whether it's one-dimension vector;
    fn is_solo(&self) -> bool {
        if let Inner::Solo(_) = self {
            true
        } else {
            false
        }
    }

    /// Whether it's two-dimension matrix;
    fn is_duet(&self) -> bool {
        if let Inner::Duet(_) = self {
            true
        } else {
            false
        }
    }

    /// Whether it's empty at all;
    fn is_empty(&self) -> bool {
        match self {
            Inner::Solo(inner) => inner.len() != 0,
            Inner::Duet(inner) => inner.len() != 0 && inner[0].len() != 0,
        }
    }

    /// Whether it has shape of right rectangle;
    /// Suppose it's not empty at all
    fn is_rect(&self) -> bool {
        match self {
            Inner::Duet(inner) => {
                let mut cols = None;
                for r in 0..inner.len() {
                    if cols.is_none() {
                        cols = Some(inner[r].len());
                    } else if inner[r].len() != cols.unwrap() {
                        return false;
                    }
                }
                true
            }
            _ => true,
        }
    }

    /// Number of rows;
    fn rows(&self) -> usize {
        match self {
            Self::Solo(_) => 1,
            Self::Duet(inner) => inner.len(),
        }
    }

    /// Number of cols;
    /// Suppose self as Duet isn't emtpy;
    fn cols(&self) -> usize {
        match self {
            Self::Solo(inner) => inner.len(),
            Self::Duet(inner) => inner[0].len(),
        }
    }
}


/// Describes how to treat the inner;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repr {
    /// Treats an instance as arbitrary non-empty matrix with shape of rectangle;
    Matrix,
    /// Treats an instance as non-empty square matrix;
    /// Skips checking whether it's square:
    Square,
    /// Treats an instance as non-empty row;
    /// May have inner of Solo or Duet, but row is the single Vec<f64>;
    /// Therefore, trans field is always false and calling transpose is prohibited;
    /// To transpose switch it to Col;
    Row,
    /// Treats an instance as non-empty row;
    /// May have inner of Solo or Duet, but row is the single Vec<f64>;
    /// Therefore, trans field is always false and calling transpose is prohibited;
    /// To transpose switch it to Col;
    Col,
    /// Treats an instance as non-empty list of rows;
    /// Row is an elem of top-level Vec, so it's Vec too;
    /// Therefore trans field is always false;
    /// Prohibits calling transpose() and col_iter() on it;
    RowList,
    /// Treats an instance as non-empty list of cols;
    /// Col is an elem of top-level Vec, so it's Vec too;
    /// Therefore trans field is always true;
    /// Prohibits calling transpose() and row_iter() on it;
    ColList,
    /// When inabortable errors occuries, it's filled with related error;
    /// Always check whether such an option is picked;
    Err(MatrErr),
}


/// Single struct provides polymorphism of Matrix / Vector / VectorList;
/// Prohibited to be empty;
#[derive(Debug, Clone)]
pub struct Matrixify {
    /// Size of inner cannot be changed, only via recreating it;
    inner: Inner,
    /// Whether to get inner[i][j] as inner[i][j] or inner[j][i];
    trans: bool,
    /// Repr is watching at inner through the trans field;
    /// Although repr depends on trans, it can block transpose() calls;
    repr: Repr,
}

impl Matrixify {
    /// Returns matrixify of the given size filled with zeros;
    pub fn zero(size: (usize, usize), repr: Repr) -> Self {
        Self::fill_with(size, repr, 0.0)
    }

    /// Returns matrixify of the given size filled with the given f64;
    pub fn fill_with((r, c): (usize, usize), repr: Repr, with: f64) -> Self {
        // for row (Inner::Solo)
        if let Repr::Row = repr {
            if r == 1 {
                let inner = vec![with; c];
                Self {
                    inner: Inner::Solo(inner),
                    trans: false,
                    repr: Repr::Row,
                }
            } else {
                let inner = vec![];
                Self {
                    inner: Inner::Solo(inner),
                    trans: false,
                    repr: Repr::Err(MultidimRow),
                }
            }
        }
        // for col (Inner::Solo)
        else if let Repr::Col = repr {
            if c == 1 {
                let inner = vec![with; r];
                Self {
                    inner: Inner::Solo(inner),
                    trans: true,
                    repr: Repr::Col,
                }
            } else {
                let inner = vec![];
                Self {
                    inner: Inner::Solo(inner),
                    trans: true,
                    repr: Repr::Err(MultidimCol),
                }
            }
        }
        // for two-dim (Inner::Duet)
        else {
            let inner = vec![vec![with; c]; r];
            Self {
                inner: Inner::Duet(inner),
                trans: false,
                repr,
            }
        }
            .against_empty_atall()
            .against_crooked_square()
    }

    /// Returns diag(1..1);
    pub fn identity(side: usize) -> Self {
        let mut id = Self::zero((side, side), Repr::Square);
        // id now is surely non-empty and square
        for d in 0..side {
            *id.att_mut((d, d)).unwrap() = 1.0;
        }
        id
    }

    /// Transposes Matrix / Square
    pub fn transpose(&mut self) -> MatrRes<()> {
        match self.repr {
            Repr::Matrix | Repr::Square => {
                self.trans = !self.trans;
                Go(())
            },
            Repr::Err(_) => UnhandledMatrErr,
            _ => Untransposable,
        }
    }

    /// Access to trans field;
    pub fn is_transposed(&self) -> bool {
        self.trans
    }

    /// Access to repr field;
    pub fn repr(&self) -> Repr {
        self.repr
    }

    /// Size of Matrix / Square / RowList / ColList inner (or error);
    pub fn inner_size(&self) -> MatrRes<(usize, usize)> {
        match self.repr {
            Repr::Matrix | Repr::Square | Repr::RowList | Repr::ColList => {
                match self.trans {
                    false => Go((self.inner.rows(), self.inner.cols())),
                    true => Go((self.inner.cols(), self.inner.rows())),
                }
            }
            Repr::Err(_) => UnhandledMatrErr,
            _ => TreatSoloAsDuet,
        }
    }

    /// Length of Row / Col inner (or error);
    pub fn inner_len(&self) -> MatrRes<usize> {
        match self.repr {
            Repr::Row | Repr::Col => Go(self.inner.cols()),
            Repr::Err(_) => UnhandledMatrErr,
            _ => TreatDuetAsSolo,
        }
    }

    /// Ref to element on position i in Row / Col (or error);
    pub fn at(&self, i: usize) -> MatrRes<&f64> {
        match &self.inner {
            Inner::Solo(inner) => {
                if 0 <= i && i < inner.len() {
                    Go(&inner[i])
                } else {
                    SoloOutOfBounds
                }
            }
            _ => TreatDuetAsSolo,
        }
    }

    /// Mut ref to element on position i in Row / Col (or error);
    pub fn at_mut(&mut self, i: usize) -> MatrRes<&mut f64> {
        match &mut self.inner {
            Inner::Solo(inner) => {
                if 0 <= i && i < inner.len() {
                    Go(&mut inner[i])
                } else {
                    SoloOutOfBounds
                }
            }
            _ => TreatDuetAsSolo,
        }
    }

    /// Ref to element on position (r, c) in Matrix / Square / RowList / ColList (or error);
    pub fn att(&self, (mut r, mut c): (usize, usize)) -> MatrRes<&f64> {
        if self.trans {
            (r, c) = (c, r);
        }
        match &self.inner {
            Inner::Duet(inner) => {
                if 0 <= r && r < inner.len() && 0 <= c && c < inner[0].len() {
                    Go(&inner[r][c])
                } else {
                    DuetOutOfBounds
                }
            }
            _ => TreatSoloAsDuet,
        }
    }

    /// Mut ref to element on position (r, c) in Matrix / Square / RowList / ColList (or error);
    pub fn att_mut(&mut self, (mut r, mut c): (usize, usize)) -> MatrRes<&mut f64> {
        if self.trans {
            (r, c) = (c, r);
        }
        match &mut self.inner {
            Inner::Duet(inner) => {
                if 0 <= r && r < inner.len() && 0 <= c && c < inner[0].len() {
                    Go(&mut inner[r][c])
                } else {
                    DuetOutOfBounds
                }
            }
            _ => TreatSoloAsDuet,
        }
    }

    /// Checks whether inner at least one element;
    pub fn against_empty_atall(mut self) -> Self {
        if self.inner.is_empty() {
            self.repr = Repr::Err(EmptyAtAll);
        }
        self
    }

    /// Checks whether inner is a rectangle;
    /// Suppose inner isn't empty at all;
    pub fn against_curve_sides(mut self) -> Self {
        if !self.inner.is_rect() {
            self.repr = Repr::Err(CurveSides);
        }
        self
    }

    /// Checks whether inner is square;
    /// Suppose inner isn't empty at all and has straight sides;
    pub fn against_crooked_square(mut self) -> Self {
        if let Repr::Square = self.repr {
            match &self.inner {
                Inner::Solo(inner) => {
                    if inner.len() != 1 {
                        self.repr = Repr::Err(CrookedSquare);
                    }
                }
                Inner::Duet(inner) => {
                    if inner.len() != inner[0].len() {
                        self.repr = Repr::Err(CrookedSquare);
                    }
                }
            }
        }
        self
    }
}

impl From<(Vec<f64>, Repr)> for Matrixify {
    fn from((inner, mut repr): (Vec<f64>, Repr)) -> Self {
        if let Repr::Err(_) = repr {
            repr = Repr::Err(ErrByDesign);
        }
        Self {
            inner: Inner::Solo(inner),
            trans: false,
            repr,
        }
            .against_curve_sides()
            .against_empty_atall()
            .against_crooked_square()
    }
}

impl From<(Vec<Vec<f64>>, Repr)> for Matrixify {
    fn from((inner, mut repr): (Vec<Vec<f64>>, Repr)) -> Self {
        if let Repr::Err(_) = repr {
            repr = Repr::Err(ErrByDesign);
        }
        Self {
            inner: Inner::Duet(inner),
            trans: false,
            repr,
        }
            .against_curve_sides()
            .against_empty_atall()
            .against_crooked_square()
    }
}


pub struct RowIter<'m> {
    matr: &'m Matrixify,
    curr: usize,
}


pub struct ColIter<'m> {
    matr: &'m Matrixify,
    curr: usize,
}


pub struct ElemIter<'m> {
    matr: &'m Matrixify,
    base: usize,
    curr: usize,
    dir: Direction,
}


enum Direction {
    Hor,
    Ver,
}