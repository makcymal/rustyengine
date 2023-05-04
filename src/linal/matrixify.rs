/// Matrix and Vector structs of arbitrary size.
/// Doesn't depend on any global state variable related to linear algebra.

use {
    crate::{
        globals::EPSILON,
        utils::{
            pow_minus, Sign, Size,
        },
        errs::MatrixifyErr::{self, *},
    },
    std::{
        ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
    },
};


// <<< Matrixify

/// Trait defining behaviour of struct that may be represented as Matrix.
/// All elements are supposed to be f64.
pub trait Matrixify {
    /// Returns object of any implementor, filled with zeros.
    /// Size intended to be Rect for Matrix and Row or Col for Vector.
    fn zeros(_: Size) -> Self;

    /// Returns object of any implementor, filled with arbitrary value.
    /// Size intended to be Rect for Matrix and Row or Col for Vector.
    fn fill_with(_: Size, _: f64) -> Self;

    /// Returns Size object related to the given implementor.
    fn size(&self) -> Size;

    /// Transposes implementor.
    fn transpose(&mut self);

    /// Return immutable reference to element placed on given index that is (row_index, col_index).
    /// Note that after transposing element on (row, col) can be obtained on (col, row).
    /// Panics if the index is out of bounds.
    fn elem(&self, _: (usize, usize)) -> &f64;

    /// Return mutable reference to element placed on given index that is (row_index, col_index).
    /// Panics if the index is out of bounds.
    fn elem_mut(&mut self, _: (usize, usize)) -> &mut f64;

    /// Computes norm of matrix as the sqrt of sum over all squared elements.
    fn norm(&self) -> f64;

    /// Takes implementor to ownership and tries to convert it into Vector.
    /// If successful, returns Vector, else MatrixifyErr::NotAVector.
    fn to_vector(self) -> Result<Vector, MatrixifyErr>;

    /// Checks whether LHS is equal to RHS.
    /// Note that equal implementors shouldn't be both transposed or not, even they shouldn't be of one type.
    /// Default implementation is provided.
    fn partial_eq(&self, other: &impl Matrixify) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                if (*self.elem((row, col)) - *other.elem((row, col))).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }

    /// Checks whether can be applied + or - operation on LHS and RHS
    /// If not, meaningful message is returned, else unit.
    /// Default implementation is provided.
    fn allow_add(&self, rhs: &impl Matrixify) -> Result<(), String> {
        match self.size() == rhs.size() {
            true => Ok(()),
            false => Err(format!("LHS size is: {:#?}, RHS size is: {:#?}",
                                 self.size(), rhs.size())),
        }
    }

    /// Checks whether can be applied * operation on LHS and RHS
    /// If not, meaningful message is returned, else unit.
    /// Default implementation is provided.
    fn allow_mul(&self, rhs: &impl Matrixify) -> Result<(), String> {
        match self.size().cols() == rhs.size().rows() {
            true => Ok(()),
            false => Err(format!("LHS size is: {:?}, RHS size is: {:?}",
                                 self.size(), rhs.size())),
        }
    }

    /// Takes immutable references to operands that are both Matrixify implementors
    /// and sign defines whether + or - to apply.
    /// Returns new Matrix, panics if operands have inappropriate sizes.
    /// Default implementation is provided.
    fn mat_add(&self, rhs: &impl Matrixify, sign: Sign) -> Matrix {
        if let Err(msg) = self.allow_add(rhs) {
            panic!("{}", msg);
        }

        let mut output = Matrix::zeros(self.size());
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                output[(row, col)] = match sign {
                    Sign::Plus => *self.elem((row, col)) + *rhs.elem((row, col)),
                    Sign::Minus => *self.elem((row, col)) - *rhs.elem((row, col)),
                };
            }
        }
        output
    }

    /// Takes immutable references to operands that are both Matrixify implementors and multiplies them.
    /// Returns new Matrix, panics if operands have inappropriate sizes.
    /// Default implementation is provided.
    fn mat_mul(&self, rhs: &impl Matrixify) -> Matrix {
        if let Err(msg) = self.allow_mul(rhs) {
            panic!("{}", msg);
        }

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
        output
    }

    /// Applies assign mul operations with f64 as RHS.
    /// Default implementation is provided.
    fn num_mul(&mut self, num: f64) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) *= num;
            }
        }
    }

    /// Applies assign div operations with f64 as RHS. Panics if RHS = 0.
    /// Default implementation is provided.
    fn num_div(&mut self, num: f64) {
        if num.abs() < EPSILON {
            panic!("Zero division");
        }
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) /= num;
            }
        }
    }
}

// Matrixify >>>


// <<< Matrix

/// Matrix of arbitrary size.
#[derive(Debug, Clone)]
pub struct Matrix {
    /// Values themselves.
    inner: Vec<Vec<f64>>,
    /// Whether matrix should be represented as transposed or not.
    transposed: bool,
    /// Determinant is computed lazy, on demand only and then stored.
    pub determinant: Option<f64>,
    /// As long as size can be computed from inner and transposed, it's redundant field, but it's used too often.
    /// Size may be of any variant (Rect, Row, Col), but it's intended to be a Rect.
    size: Size,
}

impl Matrix {
    /// Returns diag(1,..,1) if size is square. If not, MatrixifyErr::NonSquareMatrix is returned.
    pub fn identity(size: Size) -> Result<Self, MatrixifyErr> {
        if size.rows() != size.cols() {
            return Err(NonSquareMatrix);
        }
        let mut inner = vec![vec![0.0; size.cols()]; size.rows()];
        for d in 0..size.rows() {
            inner[d][d] = 1.0;
        }
        Ok(Self {
            inner,
            transposed: false,
            determinant: Some(1.0),
            size,
        })
    }

    /// Provides access to the private field - transposed.
    pub fn is_transposed(&self) -> bool {
        self.transposed
    }

    /// Computes determinant if it's possible. If not, MatrixifyErr::NonSquareMatrix is returned.
    /// Main reason why it doesn't return det value is that it's used in Div operation, while Matrix
    /// should be borrowed mutably, that is undesirable for RHS on Div operation.
    /// It doesn't matter whether Matrix is transposed or not.
    pub fn determine(&mut self) -> Result<(), MatrixifyErr> {
        if self.size.rows() != self.size.cols() {
            return Err(NonSquareMatrix);
        }

        if self.determinant.is_none() {
            let mut rows = vec![true; self.size.rows()];
            let mut cols = vec![true; self.size.cols()];
            self.determinant = Some(self.minor(&mut rows, &mut cols));
        }

        Ok(())
    }

    /// Computes inversed matrix for not-transposed matrix and then transposes it and then returns it.
    /// Unless it's possible MatrixifyErr::NonSquareMatrix or MatrixifyErr::ZeroDeterminant is returned.
    pub fn inverse(&self) -> Result<Self, MatrixifyErr> {
        if self.size.rows() != self.size.cols() {
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

        let mut rows = vec![true; self.size.rows()];
        let mut cols = vec![true; self.size.cols()];

        let mut inversed = Matrix::zeros(self.size);
        for row in 0..self.size.rows() {
            cols[row] = false;
            for col in 0..self.size.cols() {
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

    /// Checks for whether Matrix can be used as RHS in Div operation.
    /// If not then MatrixifyErr::NonSquareMatrix or MatrixifyErr::ZeroDeterminant is returned.
    pub fn as_divider(&mut self) -> Result<(), MatrixifyErr> {
        if self.determinant.is_none() {
            self.determine()?;
        }
        let det = self.determinant.unwrap();
        if det == 0.0 {
            return Err(ZeroDeterminant);
        }
        Ok(())
    }

    /// The only private method, computes recursively minor based on ignored rows and columns.
    /// Does not pay any attention on whether the matrix is transposed or not.
    /// Panics if checks for square Matrix have been somehow ignored.
    fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> f64 {
        // just for ensurance
        assert_eq!(self.size.rows(), self.size.cols());

        // when this code is reached, matrix surely is square
        let mut row = 0;
        while row < self.size.rows() && rows[row] == false { row += 1 }

        // row == self.size.rows() or rows[row] = true

        if row == self.size.rows() {
            return 1.0;
        } else {
            rows[row] = false;
        }

        let mut minor = 0.0;
        let mut j = 0;
        for col in 0..self.size.cols() {
            if cols[col] {
                if self.inner[row][col].abs() >= EPSILON {
                    cols[col] = false;
                    minor += pow_minus(j) * self.inner[row][col] * self.minor(rows, cols);
                    cols[col] = true;
                }
                j += 1;
            }
        }
        rows[row] = true;

        minor
    }
}

impl Matrixify for Matrix {
    fn zeros(size: Size) -> Self {
        Self {
            inner: vec![vec![0.0; size.cols()]; size.rows()],
            transposed: false,
            determinant: None,
            size,
        }
    }

    fn fill_with(size: Size, with: f64) -> Self {
        Self {
            inner: vec![vec![with; size.cols()]; size.rows()],
            transposed: false,
            determinant: None,
            size,
        }
    }

    fn size(&self) -> Size {
        self.size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.size.transpose();
    }

    fn elem(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[row][col],
            true => &self.inner[col][row],
        }
    }

    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
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
                .map(|elem| *elem * *elem)
                .sum::<f64>())
            .sum::<f64>()
            .sqrt()
    }

    fn to_vector(mut self) -> Result<Vector, MatrixifyErr> {
        if self.size().rows() != 1 && self.size().cols() != 1 {
            return Err(NotAVector);
        }
        let is_vertical = self.size().is_vertical();
        if is_vertical {
            self.transpose();
        }

        let mut output = Vector::zeros(Size::Row(self.size.cols()));
        for col in 0..self.size().cols() {
            output[col] = self[(0, col)];
        }

        if is_vertical {
            output.transpose();
        }
        Ok(output)
    }
}

/// Computes actual size based on the given Vec, sets transposed to false, determinant to None.
impl From<Vec<Vec<f64>>> for Matrix {
    fn from(inner: Vec<Vec<f64>>) -> Self {
        let size = Size::Rect((inner.len(), inner[0].len()));
        Self {
            inner,
            transposed: false,
            determinant: None,
            size,
        }
    }
}


// unary operators

/// Works exactly the same as elem() method on Matrixify trait.
/// This two implementations exists simultaneously because of different implementations for Vector.
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

/// Works exactly the same as elem_mut() method on Matrixify trait.
/// This two implementations exists simultaneously because of different implementations for Vector.
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
        self.elem_mut(index)
    }
}

/// Applies MulAssign with -1 for each element.
impl Neg for Matrix {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.num_mul(-1.0);
        self
    }
}


// binary operators

/// Provides equality comparation for Matrix and arbitrary Matrixify implementor.
impl<M: Matrixify> PartialEq<M> for Matrix {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

/// Provides Add for Matrix and arbitrary Matrixify implementor. Matrix + impl Matrixify = Matrix.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Add<&M> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &M) -> Self::Output {
        self.mat_add(rhs, Sign::Plus)
    }
}

/// Provides Sub for Matrix and arbitrary Matrixify implementor. Matrix - impl Matrixify = Matrix.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Sub<&M> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &M) -> Self::Output {
        self.mat_add(rhs, Sign::Minus)
    }
}

/// Provides Mul for Matrix and arbitrary Matrixify implementor. Matrix * impl Matrixify = Matrix.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Mul<&M> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &M) -> Self::Output {
        self.mat_mul(rhs)
    }
}

/// Provides Div for two Matrices. Matrix / Matrix = Matrix.
/// Operation panics if operands have inappropriate sizes or RHS has no inversed.
impl Div for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: Self) -> Self::Output {
        self.mat_mul(&(rhs.inverse().unwrap()))
    }
}

// Matrix >>>


// <<< Vector

/// Vector of arbitrary size.
#[derive(Debug, Default, Clone)]
pub struct Vector {
    /// Values themselves.
    inner: Vec<f64>,
    /// Whether Vector should be represented as transposed or not.
    transposed: bool,
    /// As long as size can be computed from inner and transposed, it's redundant field, but it's used too often.
    /// Size may be of any variant (Rect, Row, Col), but it's intended to be a Row or Col.
    size: Size,
}

impl Vector {
    /// Provides access to the private field - transposed.
    pub fn is_transposed(&self) -> bool {
        self.transposed
    }

    /// Provides access to the private field property - inner::<Vec>.len()
    pub fn inner_len(&self) -> usize {
        self.inner.len()
    }
}

impl Matrixify for Vector {
    fn zeros(size: Size) -> Self {
        if let Size::Col(rows) = size {
            return Self {
                inner: vec![0.0; rows],
                transposed: true,
                size,
            };
        }

        assert_eq!(size.rows(), 1);

        Self {
            inner: vec![0.0; size.cols()],
            transposed: false,
            size,
        }
    }

    // size should looks like Pair { x: length, y: 1 }
    fn fill_with(size: Size, with: f64) -> Self {
        if let Size::Col(rows) = size {
            return Self {
                inner: vec![with; rows],
                transposed: true,
                size,
            };
        }

        assert_eq!(size.rows(), 1);

        Self {
            inner: vec![with; size.cols()],
            transposed: false,
            size,
        }
    }

    fn size(&self) -> Size {
        self.size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.size.transpose();
    }

    // use only after checking whether (row, col) is valid
    fn elem(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[col],
            true => &self.inner[row],
        }
    }

    // use only after checking whether (row, col) is valid
    fn elem_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &mut self.inner[col],
            true => &mut self.inner[row],
        }
    }

    fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|elem| *elem * *elem)
            .sum::<f64>()
            .sqrt()
    }

    fn to_vector(self) -> Result<Vector, MatrixifyErr> {
        Ok(self)
    }
}

/// Computes actual size based on the given Vec, by default it's Row, transposed is false.
impl From<Vec<f64>> for Vector {
    fn from(inner: Vec<f64>) -> Self {
        let size = Size::Row(inner.len());
        Self {
            inner,
            transposed: false,
            size,
        }
    }
}


// unary operators

/// Differs from elem() method from Matrixify trait in taking only one index.
impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match self.size.is_horizontal() {
            true => self.elem((0, index)),
            false => self.elem((index, 0)),
        }
    }
}

/// Differs from elem_mut() method from Matrixify trait in taking only one index.
impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match self.size.is_horizontal() {
            true => self.elem_mut((0, index)),
            false => self.elem_mut((index, 0)),
        }
    }
}

/// Applies MulAssign with -1 for each element.
impl Neg for Vector {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.num_mul(-1.0);
        self
    }
}


// binary operators

/// Provides equality comparation for Matrix and arbitrary Matrixify implementor.
impl<M: Matrixify> PartialEq<M> for Vector {
    fn eq(&self, other: &M) -> bool {
        self.partial_eq(other)
    }
}

/// Provides Add for Matrix and arbitrary Matrixify implementor. Vector + impl Matrixify = Vector.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Add<&M> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &M) -> Self::Output {
        self.mat_add(rhs, Sign::Plus).to_vector().unwrap()
    }
}

/// Provides Sub for Matrix and arbitrary Matrixify implementor. Vector - impl Matrixify = Vector.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Sub<&M> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &M) -> Self::Output {
        self.mat_add(rhs, Sign::Minus).to_vector().unwrap()
    }
}

/// Provides Mul for Matrix and arbitrary Matrixify implementor. Vector * impl Matrixify = Matrix.
/// Operation panics if operands have inappropriate sizes.
impl<M: Matrixify> Mul<&M> for &Vector {
    type Output = Matrix;

    fn mul(self, rhs: &M) -> Self::Output {
        self.mat_mul(rhs)
    }
}

/// Provides Div for Vector and Matrix. Vector / Matrix = Matrix.
/// Operation panics if operands have inappropriate sizes or RHS has no inversed.
impl Div<&Matrix> for &Vector {
    type Output = Matrix;

    fn div(self, rhs: &Matrix) -> Self::Output {
        self.mat_mul(&(rhs.inverse().unwrap()))
    }
}


/// Used in definitions of scalar product in basis or without it.
pub fn scalar_prod(lhs: &Vector, matrix: &Matrix, rhs: &Vector) -> f64 {
    assert_eq!(lhs.inner_len(), matrix.size().rows());
    assert_eq!(matrix.size().rows(), matrix.size().cols());
    assert_eq!(matrix.size().cols(), rhs.inner_len());

    (0..rhs.inner_len())
        .map(|i| rhs[i] *
            (0..lhs.inner_len())
                .map(|j| lhs[j] * matrix[(j, i)])
                .sum::<f64>())
        .sum::<f64>()
}

// Vector >>>
