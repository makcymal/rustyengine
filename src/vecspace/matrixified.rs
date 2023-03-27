use {
    crate::{
        vecspace::{
            enums::{
                MatrixifiedError::{self, *},
                Ops,
                Sign,
            },
        },
        utils::{
            Num, Size, pow_minus,
        },
    },
    std::{
        ops::{Add, Div,
              Index, IndexMut, Mul, Neg, Sub,
        },
    },
};


// <<< Matrixified

pub trait Matrixified {
    type Elem;

    fn zeros(_: Size) -> Self;
    fn fill_with(_: Size, _: Self::Elem) -> Self where Self::Elem: Num;
    fn size(&self) -> Size;
    fn transpose(&mut self);
    fn elem(&self, _: (usize, usize)) -> &Self::Elem;
    fn elem_mut(&mut self, _: (usize, usize)) -> &mut Self::Elem;
    fn norm(&self) -> f64;
    fn to_vector(self) -> Result<Vector<Self::Elem>, MatrixifiedError>;

    fn partial_eq(&self, other: &impl Matrixified<Elem=Self::Elem>) -> bool where Self::Elem: Num {
        if self.size() != other.size() {
            return false;
        }
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                if self.elem((row, col)) != other.elem((row, col)) {
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

    fn m_add(&self, rhs: &impl Matrixified<Elem=Self::Elem>, sign: Sign)
             -> Result<Matrix<Self::Elem>, MatrixifiedError>
        where Self::Elem: Num
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


    fn m_mul(&self, rhs: &impl Matrixified<Elem=Self::Elem>)
             -> Result<Matrix<Self::Elem>, MatrixifiedError>
        where Self::Elem: Num
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

    fn a(&mut self, num: Self::Elem) where Self::Elem: Num {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) += num;
            }
        }
    }

    fn s(&mut self, num: Self::Elem) where Self::Elem: Num {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) -= num;
            }
        }
    }

    fn m(&mut self, num: Self::Elem) where Self::Elem: Num {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) *= num;
            }
        }
    }

    fn d(&mut self, num: Self::Elem) where Self::Elem: Num {
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
pub struct Matrix<T> {
    inner: Vec<Vec<T>>,
    transposed: bool,
    pub determinant: Option<T>,
    initial_size: Size,
    pub actual_size: Size,
}

impl<T: Num> Matrix<T> {
    pub const fn empty() -> Self {
        Self {
            inner: vec![],
            transposed: false,
            determinant: None,
            initial_size: Size::Rect((0, 0)),
            actual_size: Size::Rect((0, 0)),
        }
    }

    pub fn identity(initial_size: Size) -> Result<Self, MatrixifiedError> {
        if initial_size.rows() != initial_size.cols() {
            return Err(NonSquareMatrix);
        }
        let mut inner = vec![vec![T::zero(); initial_size.cols()]; initial_size.rows()];
        for d in 0..initial_size.rows() {
            inner[d][d] = T::one();
        }
        Ok(Self {
            inner,
            transposed: false,
            determinant: Some(T::one()),
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
        if det == T::zero() {
            return Err(ZeroDeterminant);
        }

        let mut rows = vec![true; self.initial_size.rows()];
        let mut cols = vec![true; self.initial_size.cols()];

        let mut inversed = Matrix::<T>::zeros(self.initial_size);
        for row in 0..self.initial_size.rows() {
            cols[row] = false;
            for col in 0..self.initial_size.cols() {
                rows[col] = false;
                inversed.inner[row][col] =
                    pow_minus::<T>(row + col) * self.minor(&mut rows, &mut cols) / det;
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
    fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> T {
        // just for ensurance
        assert_eq!(self.initial_size.rows(), self.initial_size.cols());

        // when this code is reached, matrix surely is square
        let mut row = 0;
        while row < self.initial_size.rows() && rows[row] == false { row += 1 }

        if row == self.initial_size.rows() {
            return T::one();
        } else {
            rows[row] = false;
        }

        let mut minor = T::zero();
        for col in 0..self.initial_size.cols() {
            let mut j = 0;
            if cols[col] {
                cols[col] = false;
                minor += pow_minus::<T>(j) * self.inner[row][col] * self.minor(rows, cols);
                cols[col] = true;
            }
        }
        rows[row] = true;

        minor
    }
}

impl<T: Num> Matrixified for Matrix<T> {
    type Elem = T;

    fn zeros(initial_size: Size) -> Self {
        Self {
            inner: vec![vec![T::zero(); initial_size.cols()]; initial_size.rows()],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    fn fill_with(initial_size: Size, with: T) -> Self {
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
    fn elem(&self, (row, col): (usize, usize)) -> &Self::Elem {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[row][col],
            true => &self.inner[col][row],
        }
    }

    // use only after checking whether (r, c) is valid
    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Elem {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &mut self.inner[row][col],
            true => &mut self.inner[col][row],
        }
    }

    fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|row| row.iter()
                .map(|elem| (*elem * *elem).into())
                .sum::<f64>())
            .sum::<f64>()
            .sqrt()
    }

    fn to_vector(mut self) -> Result<Vector<T>, MatrixifiedError> {
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

impl<T: Num> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(inner: Vec<Vec<T>>) -> Self {
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

impl<T: Num> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl<T: Num> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.elem_mut(index)
    }
}

impl<T: Num> Neg for Matrix<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-T::one());
        self
    }
}


// binary operators

impl<T: Num, M: Matrixified<Elem=T>> PartialEq<M> for Matrix<T> {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

// Matrix + [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Add<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)
    }
}

// Matrix - [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Sub<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)
    }
}

// Matrix * [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Mul<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Matrix / Matrix = Matrix
impl<T: Num> Div for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.m_mul(&(rhs.inverse()?))
    }
}

// Matrix >>>


// <<< Vector

#[derive(Debug, Clone)]
pub struct Vector<T> {
    inner: Vec<T>,
    transposed: bool,
    length: usize,
    actual_size: Size,
}

impl<T: Num> Vector<T> {
    pub fn is_transposed(&self) -> bool {
        self.transposed
    }
}

impl<T: Num> Matrixified for Vector<T> {
    type Elem = T;

    // size should looks like Pair { x: length, y: 1 }
    fn zeros(initial_size: Size) -> Self {
        if let Size::Col(rows) = initial_size {
            return Self {
                inner: vec![T::zero(); rows],
                transposed: true,
                length: rows,
                actual_size: initial_size,
            };
        }

        assert_eq!(initial_size.rows(), 1);

        Self {
            inner: vec![T::zero(); initial_size.cols()],
            transposed: false,
            length: initial_size.cols(),
            actual_size: initial_size,
        }
    }

    // size should looks like Pair { x: length, y: 1 }
    fn fill_with(initial_size: Size, with: T) -> Self {
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
    fn elem(&self, (row, col): (usize, usize)) -> &Self::Elem {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[col],
            true => &self.inner[row],
        }
    }

    // use only after checking whether (row, col) is valid
    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Elem {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &mut self.inner[col],
            true => &mut self.inner[row],
        }
    }

    fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|elem| (*elem * *elem).into())
            .sum::<f64>()
            .sqrt()
    }

    fn to_vector(self) -> Result<Vector<T>, MatrixifiedError> {
        Ok(self)
    }
}

impl<T: Num> From<Vec<T>> for Vector<T> {
    fn from(inner: Vec<T>) -> Self {
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

impl<T: Num> Index<(usize, usize)> for Vector<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl<T: Num> IndexMut<(usize, usize)> for Vector<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.elem_mut(index)
    }
}

impl<T: Num> Neg for Vector<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-T::one());
        self
    }
}


// binary operators

impl<T: Num, M: Matrixified<Elem=T>> PartialEq<M> for Vector<T> {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

// Vector + [Matrix | Vector] = Vector
impl<T: Num, M: Matrixified<Elem=T>> Add<&M> for &Vector<T> {
    type Output = Result<Vector<T>, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)?.to_vector()
    }
}

// Vector - [Matrix | Vector] = Vector
impl<T: Num, M: Matrixified<Elem=T>> Sub<&M> for &Vector<T> {
    type Output = Result<Vector<T>, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)?.to_vector()
    }
}

// Vector * [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Mul<&M> for &Vector<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Vector >>>
