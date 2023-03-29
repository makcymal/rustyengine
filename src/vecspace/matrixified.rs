use {
    crate::{
        globals::{Flt, EPSILON},
        utils::{
            Size, pow_minus,
        },
        vecspace::{
            enums::{
                MatrixifiedError::{self, *},
                Sign,
                MatrixType,
            },
        },
    },
    std::{
        ops::{Add, Div, Mul, Sub,
              Index, IndexMut, Neg,
        },
    },
};


// <<< Matrixified

pub trait Matrixified {
    fn zeros(_: Size) -> Self;
    fn fill_with(_: Size, _: Flt) -> Self;
    fn size(&self) -> Size;
    fn transpose(&mut self);
    fn elem(&self, _: (usize, usize)) -> &Flt;
    fn elem_mut(&mut self, _: (usize, usize)) -> &mut Flt;
    fn norm(&self) -> Flt;
    fn to_vector(self) -> Result<Vector, MatrixifiedError>;

    fn partial_eq(&self, other: &impl Matrixified) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                if *self.elem((row, col)) - *other.elem((row, col)) > EPSILON {
                    return false;
                }
            }
        }
        true
    }

    fn allow_add(&self, rhs: &impl Matrixified) -> Result<(), MatrixifiedError> {
        match self.size() == rhs.size() {
            true => Ok(()),
            false => Err(InappropriateSizes),
        }
    }

    fn allow_mul(&self, rhs: &impl Matrixified) -> Result<(), MatrixifiedError> {
        match self.size().rows() == rhs.size().cols() {
            true => Ok(()),
            false => Err(InappropriateSizes),
        }
    }

    fn m_add(&self, rhs: &impl Matrixified, sign: Sign)
             -> Result<Matrix, MatrixifiedError>
    {
        self.allow_add(rhs)?;

        let mut output = Matrix::zeros(self.size());
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                output[(row, col)] = match sign {
                    Sign::Plus => *self.elem((row, col)) + *rhs.elem((row, col)),
                    Sign::Minus => *self.elem((row, col)) - *rhs.elem((row, col)),
                };
            }
        }
        Ok(output)
    }


    fn m_mul(&self, rhs: &impl Matrixified)
             -> Result<Matrix, MatrixifiedError>
    {
        self.allow_mul(rhs)?;
        let output_size = Size::Rect((self.size().rows(), rhs.size().cols()));
        let mut output = Matrix::zeros(output_size);

        for row in 0..output_size.rows() {
            for col in 0..output_size.cols() {
                output[(row, col)] =
                    (0..self.size().cols())
                        .map(|i| *self.elem((row, i)) * *rhs.elem((i, col)))
                        .sum();
            }
        }
        Ok(output)
    }

    fn a(&mut self, num: Flt) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) += num;
            }
        }
    }

    fn s(&mut self, num: Flt) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) -= num;
            }
        }
    }

    fn m(&mut self, num: Flt) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) *= num;
            }
        }
    }

    fn d(&mut self, num: Flt) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) /= num;
            }
        }
    }
}

// Matrixified >>>


// <<< Matrix

#[derive(Debug, Clone)]
pub struct Matrix {
    inner: Vec<Vec<Flt>>,
    transposed: bool,
    pub determinant: Option<Flt>,
    initial_size: Size,
    pub actual_size: Size,
}

impl Matrix {
    pub fn identity(initial_size: Size) -> Result<Self, MatrixifiedError> {
        if initial_size.rows() != initial_size.cols() {
            return Err(NonSquareMatrix);
        }
        let mut inner = vec![vec![0.0; initial_size.cols()]; initial_size.rows()];
        for d in 0..initial_size.rows() {
            inner[d][d] = 1.0;
        }
        Ok(Self {
            inner,
            transposed: false,
            determinant: Some(1.0),
            initial_size,
            actual_size: initial_size,
        })
    }

    pub fn is_transposed(&self) -> bool {
        self.transposed
    }

    pub fn determine(&mut self) -> Result<(), MatrixifiedError> {
        if self.initial_size.rows() != self.initial_size.cols() {
            return Err(NonSquareMatrix);
        }

        if self.determinant.is_none() {
            let mut rows = vec![true; self.initial_size.rows()];
            let mut cols = vec![true; self.initial_size.cols()];
            self.determinant = Some(self.minor(&mut rows, &mut cols));
        }

        Ok(())
    }

    // computes inversed matrix for not-transposed matrix and then transposes it
    pub fn inverse(&self) -> Result<Self, MatrixifiedError> {
        if self.initial_size.rows() != self.initial_size.cols() {
            return Err(NonSquareMatrix);
        }
        let det;
        if let Some(d) = self.determinant {
            det = d;
        } else {
            return Err(UnknownDeterminant);
        }
        if det == 0.0 {
            return Err(ZeroDeterminant);
        }

        let mut rows = vec![true; self.initial_size.rows()];
        let mut cols = vec![true; self.initial_size.cols()];

        let mut inversed = Matrix::zeros(self.initial_size);
        for row in 0..self.initial_size.rows() {
            cols[row] = false;
            for col in 0..self.initial_size.cols() {
                rows[col] = false;
                inversed.inner[row][col] =
                    pow_minus(row + col) * self.minor(&mut rows, &mut cols) / det;
                rows[col] = true;
            }
            cols[row] = true;
        }

        if self.transposed {
            inversed.transpose();
        }

        Ok(inversed)
    }

    // does not pay any attention on whether the matrix is transposed or not
    fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> Flt {
        // just for ensurance
        assert_eq!(self.initial_size.rows(), self.initial_size.cols());

        // when this code is reached, matrix surely is square
        let mut row = 0;
        while row < self.initial_size.rows() && rows[row] == false { row += 1 }

        // row == self.initial_size.rows() || rows[row] = true

        if row == self.initial_size.rows() {
            return 1.0;
        } else {
            rows[row] = false;
        }

        let mut minor = 0.0;
        let mut j = 0;
        for col in 0..self.initial_size.cols() {
            if cols[col] {
                if -EPSILON >= -self.inner[row][col] && self.inner[row][col] <= EPSILON {
                    continue;
                }
                cols[col] = false;
                minor += pow_minus(j) * self.inner[row][col] * self.minor(rows, cols);
                cols[col] = true;
                j += 1;
            }
        }
        rows[row] = true;

        minor
    }
}

impl Matrixified for Matrix {
    fn zeros(initial_size: Size) -> Self {
        Self {
            inner: vec![vec![0.0; initial_size.cols()]; initial_size.rows()],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    fn fill_with(initial_size: Size, with: Flt) -> Self {
        Self {
            inner: vec![vec![with; initial_size.cols()]; initial_size.rows()],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    fn size(&self) -> Size {
        self.actual_size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.actual_size.transpose();
    }

    // use only after checking whether (r, c) is valid
    fn elem(&self, (row, col): (usize, usize)) -> &Flt {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[row][col],
            true => &self.inner[col][row],
        }
    }

    // use only after checking whether (r, c) is valid
    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut Flt {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &mut self.inner[row][col],
            true => &mut self.inner[col][row],
        }
    }

    fn norm(&self) -> Flt {
        self.inner
            .iter()
            .map(|row| row.iter()
                .map(|elem| *elem * *elem)
                .sum::<Flt>())
            .sum::<Flt>()
            .sqrt()
    }

    fn to_vector(mut self) -> Result<Vector, MatrixifiedError> {
        if self.size().rows() != 1 && self.size().cols() != 1 {
            return Err(NotAVector);
        }
        let is_vertical = self.size().is_vertical();
        if is_vertical {
            self.transpose();
        }

        let mut output = Vector::zeros(self.size());
        for col in 0..self.size().cols() {
            output[(0, col)] = self[(0, col)];
        }

        if is_vertical {
            output.transpose();
        }
        Ok(output)
    }
}

impl From<Vec<Vec<Flt>>> for Matrix {
    fn from(inner: Vec<Vec<Flt>>) -> Self {
        let size = Size::Rect((inner.len(), inner[0].len()));
        Self {
            inner,
            transposed: false,
            determinant: None,
            initial_size: size,
            actual_size: size,
        }
    }
}

// unary operators

impl Index<(usize, usize)> for Matrix {
    type Output = Flt;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Flt {
        self.elem_mut(index)
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-1.0);
        self
    }
}


// binary operators

impl<M: Matrixified> PartialEq<M> for Matrix {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

// Matrix + [Matrix | Vector] = Matrix
impl<M: Matrixified> Add<&M> for &Matrix {
    type Output = Result<Matrix, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)
    }
}

// Matrix - [Matrix | Vector] = Matrix
impl<M: Matrixified> Sub<&M> for &Matrix {
    type Output = Result<Matrix, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)
    }
}

// Matrix * [Matrix | Vector] = Matrix
impl<M: Matrixified> Mul<&M> for &Matrix {
    type Output = Result<Matrix, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Matrix / Matrix = Matrix
impl Div for &Matrix {
    type Output = Result<Matrix, MatrixifiedError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.m_mul(&(rhs.inverse()?))
    }
}

// Matrix >>>


// <<< Vector

#[derive(Debug, Clone)]
pub struct Vector {
    inner: Vec<Flt>,
    transposed: bool,
    length: usize,
    actual_size: Size,
}

impl Vector {
    pub fn is_transposed(&self) -> bool {
        self.transposed
    }
}

impl Matrixified for Vector {
    // size should looks like Pair { x: length, y: 1 }
    fn zeros(initial_size: Size) -> Self {
        if let Size::Col(rows) = initial_size {
            return Self {
                inner: vec![0.0; rows],
                transposed: true,
                length: rows,
                actual_size: initial_size,
            };
        }

        assert_eq!(initial_size.rows(), 1);

        Self {
            inner: vec![0.0; initial_size.cols()],
            transposed: false,
            length: initial_size.cols(),
            actual_size: initial_size,
        }
    }

    // size should looks like Pair { x: length, y: 1 }
    fn fill_with(initial_size: Size, with: Flt) -> Self {
        if let Size::Col(rows) = initial_size {
            return Self {
                inner: vec![with; rows],
                transposed: true,
                length: rows,
                actual_size: initial_size,
            };
        }

        assert_eq!(initial_size.rows(), 1);

        Self {
            inner: vec![with; initial_size.cols()],
            transposed: false,
            length: initial_size.cols(),
            actual_size: initial_size,
        }
    }

    fn size(&self) -> Size {
        self.actual_size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.actual_size.transpose();
    }

    // use only after checking whether (row, col) is valid
    fn elem(&self, (row, col): (usize, usize)) -> &Flt {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[col],
            true => &self.inner[row],
        }
    }

    // use only after checking whether (row, col) is valid
    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut Flt {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &mut self.inner[col],
            true => &mut self.inner[row],
        }
    }

    fn norm(&self) -> Flt {
        self.inner
            .iter()
            .map(|elem| *elem * *elem)
            .sum::<Flt>()
            .sqrt()
    }

    fn to_vector(self) -> Result<Vector, MatrixifiedError> {
        Ok(self)
    }
}

impl From<Vec<Flt>> for Vector {
    fn from(inner: Vec<Flt>) -> Self {
        let size = Size::Row(inner.len());
        Self {
            inner,
            transposed: false,
            length: size.cols(),
            actual_size: size,
        }
    }
}


// unary operators

impl Index<(usize, usize)> for Vector {
    type Output = Flt;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl IndexMut<(usize, usize)> for Vector {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Flt {
        self.elem_mut(index)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-1.0);
        self
    }
}


// binary operators

impl<M: Matrixified> PartialEq<M> for Vector {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

// Vector + [Matrix | Vector] = Vector
impl<M: Matrixified> Add<&M> for &Vector {
    type Output = Result<Vector, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)?.to_vector()
    }
}

// Vector - [Matrix | Vector] = Vector
impl<M: Matrixified> Sub<&M> for &Vector {
    type Output = Result<Vector, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)?.to_vector()
    }
}

// Vector * [Matrix | Vector] = Matrix
impl<M: Matrixified> Mul<&M> for &Vector {
    type Output = Result<Matrix, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Vector >>>

pub fn common_matrix(m_type: MatrixType) -> Matrix {
    let inner = match m_type {
        MatrixType::Identity => vec![vec![1.0, 0.0, 0.0],
                                     vec![0.0, 1.0, 0.0],
                                     vec![0.0, 0.0, 1.0]],
        MatrixType::NegIdentity => vec![vec![-1.0, 0.0, 0.0],
                                        vec![0.0, -1.0, 0.0],
                                        vec![0.0, 0.0, -1.0]],
        MatrixType::RevIdentity => vec![vec![0.0, 0.0, 1.0],
                                        vec![0.0, 1.0, 0.0],
                                        vec![1.0, 0.0, 0.0]],
        MatrixType::NegRevIdentity => vec![vec![0.0, 0.0, -1.0],
                                           vec![0.0, -1.0, 0.0],
                                           vec![-1.0, 0.0, 0.0]],
        MatrixType::Cross => vec![vec![1.0, 0.0, 1.0],
                                  vec![0.0, 1.0, 0.0],
                                  vec![1.0, 0.0, 1.0]],
        MatrixType::NegCross => vec![vec![-1.0, 0.0, -1.0],
                                     vec![0.0, -1.0, 0.0],
                                     vec![-1.0, 0.0, -1.0]],
        MatrixType::Rhomb => vec![vec![0.0, 1.0, 0.0],
                                  vec![1.0, 0.0, 1.0],
                                  vec![0.0, 1.0, 0.0]],
        MatrixType::NegRhomb => vec![vec![0.0, -1.0, 0.0],
                                     vec![-1.0, 0.0, -1.0],
                                     vec![0.0, -1.0, 0.0]],
        MatrixType::Ones => vec![vec![1.0, 1.0, 1.0],
                                 vec![1.0, 1.0, 1.0],
                                 vec![1.0, 1.0, 1.0]],
    };
    Matrix::from(inner)
}
