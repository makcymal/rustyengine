use {
    crate::{
        linal::enums::{
            Inner::{self, *},
            Repr::{self, *},
            MatrErr::{self, *},
            Dir,
        },
    },
};


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
    /// Returns diag(1..1);
    pub fn identity(side: usize, mut repr: Repr) -> Self {
        let mut id = Self::zero((side, side), repr);
        // id now is surely non-empty and square
        for d in 0..side {
            *id.att_mut((d, d)).unwrap() = 1.0;
        }
        id
    }

    /// Returns matrixify of the given size filled with zeros;
    pub fn zero(size: (usize, usize), mut repr: Repr) -> Self {
        Self::fill_with(size, repr, 0.0)
    }

    /// Returns matrixify of the given size filled with the given f64;
    pub fn fill_with((r, c): (usize, usize), mut repr: Repr, with: f64) -> Self {
        if let Failure(_) = repr {
            repr = Failure(ErrByDesign);
        }
        // for row (Inner::Solo)
        if let Row = repr {
            if r == 1 {
                let inner = vec![with; c];
                Self {
                    inner: Solo(inner),
                    trans: false,
                    repr: Row,
                }
            } else {
                let inner = vec![];
                Self {
                    inner: Solo(inner),
                    trans: false,
                    repr: Failure(TooManyRows),
                }
            }
        }
        // for col (Inner::Solo)
        else if let Col = repr {
            if c == 1 {
                let inner = vec![with; r];
                Self {
                    inner: Solo(inner),
                    trans: true,
                    repr: Col,
                }
            } else {
                let inner = vec![];
                Self {
                    inner: Solo(inner),
                    trans: true,
                    repr: Failure(TooManyCols),
                }
            }
        }
        // for two-dim (Inner::Duet)
        else {
            let inner = vec![vec![with; c]; r];
            Self {
                inner: Duet(inner),
                trans: false,
                repr,
            }
        }.validate()
    }

    /// Returns matrixify with the given one-dim inner;
    pub fn from_solo(inner: Vec<f64>, mut repr: Repr) -> Self {
        if let Failure(_) = repr {
            repr = Failure(ErrByDesign);
        }
        Self {
            inner: Solo(inner),
            trans: false,
            repr,
        }
            .validate()
            .against_crooked_square()
    }

    /// Returns matrixify with the given two-dim inner;
    pub fn from_duet(inner: Vec<Vec<f64>>, mut repr: Repr) -> Self {
        if let Failure(_) = repr {
            repr = Failure(ErrByDesign);
        }
        Self {
            inner: Duet(inner),
            trans: false,
            repr,
        }
            .validate()
            .against_crooked_square()
    }

    /// Access to trans field;
    pub fn is_transposed(&self) -> bool {
        self.trans
    }

    /// Transposes Matrix / Square / RowList / ColList;
    pub fn transpose(mut self) -> Self {
        match self.repr {
            Matrix | Square | RowList | ColList => self.trans = !self.trans,
            Failure(_) => self.repr = Failure(UnhandledFailure),
            _ => self.repr = Failure(Untransposable),
        }
        self
    }

    /// Access to repr field;
    pub fn repr(&self) -> Repr {
        self.repr
    }

    /// Tries to represent self in the given way;
    pub fn switch(mut self, into: Repr) -> Self {
        if let Failure(_) = self.repr {
            self.repr = Failure(UnhandledFailure);
        }
        match into {
            Square => {
                if self.inner.rows() == self.inner.cols() {
                    self.repr = Square;
                } else {
                    self.repr = Failure(CrookedSquare);
                }
            },
            Row => {
                match self.repr {
                    Col => {
                        self.trans = !self.trans;
                        self.repr = Row;
                    }
                    Row => (),
                    Matrix | Square | RowList | ColList => {
                        if self.rows().unwrap() == 1 {
                            self.repr = Row;
                        } else {
                            self.repr = Failure(NotLinear);
                        }
                    },
                    _ => unreachable!(),
                }
            },
            Col => {
                match self.repr {
                    Row => {
                        self.trans = !self.trans;
                        self.repr = Col;
                    }
                    Col => (),
                    Matrix | Square | RowList | ColList => {
                        if self.cols().unwrap() == 1 {
                            self.repr = Col;
                        } else {
                            self.repr = Failure(NotLinear);
                        }
                    },
                    _ => unreachable!(),
                }
            }
            RowList => {
                match self.repr {
                    Col | ColList => {
                        self.trans = !self.trans;
                        self.repr = RowList;
                    },
                    Failure(_) => unreachable!(),
                    _ => self.repr = RowList,
                }
            },
            ColList => {
                match self.repr {
                    Row | RowList => {
                        self.trans = !self.trans;
                        self.repr = ColList;
                    },
                    Failure(_) => unreachable!(),
                    _ => self.repr = ColList,
                }
            },
            Matrix => self.repr = Matrix,
            Failure(_) => unreachable!(),
        }
        self
    }

    /// Number of rows in inner accounting trans field;
    pub fn rows(&self) -> Result<usize, MatrErr> {
        match self.repr {
            Failure(_) => Err(UnhandledFailure),
            _ => match self.trans {
                false => Ok(self.inner.rows()),
                true => Ok(self.inner.cols()),
            },
        }
    }

    /// Number of cols in inner accounting trans field;
    pub fn cols(&self) -> Result<usize, MatrErr> {
        match self.repr {
            Failure(_) => Err(UnhandledFailure),
            _ => match self.trans {
                false => Ok(self.inner.cols()),
                true => Ok(self.inner.rows()),
            },
        }
    }

    /// Ref to element on position i in Row / Col (or error);
    pub fn at(&self, i: usize) -> Result<&f64, MatrErr> {
        match self.repr {
            Row => {
                match self.trans {
                    false => self.inner.att((0, i)),
                    true => self.inner.att((i, 0)),
                }
            },
            Col => {
                match self.trans {
                    false => self.inner.att((i, 0)),
                    true => self.inner.att((0, i)),
                }
            },
            Failure(_) => Err(UnhandledFailure),
            _ => Err(NotLinear),
        }
    }

    /// Mut ref to element on position i in Row / Col (or error);
    pub fn at_mut(&mut self, i: usize) -> Result<&mut f64, MatrErr> {
        match self.repr {
            Row => {
                match self.trans {
                    false => self.inner.att_mut((0, i)),
                    true => self.inner.att_mut((i, 0)),
                }
            },
            Col => {
                match self.trans {
                    false => self.inner.att_mut((i, 0)),
                    true => self.inner.att_mut((0, i)),
                }
            },
            Failure(_) => Err(UnhandledFailure),
            _ => Err(NotLinear),
        }
    }

    /// Ref to element on position (r, c) in Matrix / Square / RowList / ColList (or error);
    pub fn att(&self, (mut r, mut c): (usize, usize)) -> Result<&f64, MatrErr> {
        if self.trans {
            (r, c) = (c, r);
        }
        if let ColList = self.repr {
            (r, c) = (c, r)
        }
        match self.repr {
            Matrix | Square | RowList | ColList => self.inner.att((r, c)),
            Failure(_) => Err(UnhandledFailure),
            _ => Err(IsLinear),
        }
    }

    /// Mut ref to element on position (r, c) in Matrix / Square / RowList / ColList (or error);
    pub fn att_mut(&mut self, (mut r, mut c): (usize, usize)) -> Result<&mut f64, MatrErr> {
        if self.trans {
            (r, c) = (c, r);
        }
        if let ColList = self.repr {
            (r, c) = (c, r)
        }
        match self.repr {
            Matrix | Square | RowList | ColList => self.inner.att_mut((r, c)),
            Failure(err) => {
                dbg!(err);
                Err(UnhandledFailure)
            },
            _ => Err(IsLinear),
        }
    }

    /// Checks whether inner at least one element;
    pub fn validate(mut self) -> Self {
        if let Err(err) = self.inner.validate() {
            self.repr = Failure(err);
        }
        self
    }

    /// Checks whether inner is square;
    /// Suppose inner isn't empty at all and has straight sides;
    pub fn against_crooked_square(mut self) -> Self {
        if let Square = self.repr {
            match &self.inner {
                Solo(inner) => {
                    if inner.len() != 1 {
                        self.repr = Failure(CrookedSquare);
                    }
                }
                Duet(inner) => {
                    if inner.len() != inner[0].len() {
                        self.repr = Failure(CrookedSquare);
                    }
                }
            }
        }
        self
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
    dir: Dir,
}
