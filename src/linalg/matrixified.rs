// Matrix and Vector structs of arbitrary size.
// Depends only on global BIFORM matrix used in definition of scalar product without basis.
// Does not depends on global VECSPACE, GRAMM matrix or COORDSYS
// so related vector, scalar product defined in coord_sys module.

use {
    crate::{
        globals::{EPSILON, BIFORM},
        linalg::coord_sys::Vecspace,
        utils::{
            pow_minus, Sign, Size,
        },
    },
    std::{
        ops::{
            Add, Div, Index, IndexMut, Mul, Neg, Sub,
        },
    },
};
use crate::enums::{
    MatrixifiedError::{self, *},
    MatrixType,
};


// <<< Matrixified

pub trait Matrixified {
    fn zeros(_: Size) -> Self;
    fn fill_with(_: Size, _: f64) -> Self;
    fn size(&self) -> Size;
    fn transpose(&mut self);
    fn elem(&self, _: (usize, usize)) -> &f64;
    fn elem_mut(&mut self, _: (usize, usize)) -> &mut f64;
    fn norm(&self) -> f64;
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

    fn allow_add(&self, rhs: &impl Matrixified) -> Result<(), String> {
        match self.size() == rhs.size() {
            true => Ok(()),
            false => Err(format!("LHS size is: {:#?}, RHS size is: {:#?}",
                                 self.size(), rhs.size())),
        }
    }

    fn allow_mul(&self, rhs: &impl Matrixified) -> Result<(), String> {
        match self.size().cols() == rhs.size().rows() {
            true => Ok(()),
            false => Err(format!("LHS size is: {:?}, RHS size is: {:?}",
                                 self.size(), rhs.size())),
        }
    }

    fn m_add(&self, rhs: &impl Matrixified, sign: Sign) -> Matrix {
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

    fn m_mul(&self, rhs: &impl Matrixified) -> Matrix {
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

    fn a(&mut self, num: f64) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) += num;
            }
        }
    }

    fn s(&mut self, num: f64) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) -= num;
            }
        }
    }

    fn m(&mut self, num: f64) {
        for row in 0..self.size().rows() {
            for col in 0..self.size().cols() {
                *self.elem_mut((row, col)) *= num;
            }
        }
    }

    fn d(&mut self, num: f64) {
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
    inner: Vec<Vec<f64>>,
    transposed: bool,
    pub determinant: Option<f64>,
    initial_size: Size,
    pub actual_size: Size,
}

impl Matrix {
    pub const fn empty(initial_size: Size) -> Self {
        Self {
            inner: vec![],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

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
    fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> f64 {
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

    fn fill_with(initial_size: Size, with: f64) -> Self {
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
    fn elem(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(self.size().contains(row, col));

        match self.transposed {
            false => &self.inner[row][col],
            true => &self.inner[col][row],
        }
    }

    // use only after checking whether (r, c) is valid
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

    fn to_vector(mut self) -> Result<Vector, MatrixifiedError> {
        if self.size().rows() != 1 && self.size().cols() != 1 {
            return Err(NotAVector);
        }
        let is_vertical = self.size().is_vertical();
        if is_vertical {
            self.transpose();
        }

        let mut output = Vector::zeros(Size::Row(self.actual_size.cols()));
        for col in 0..self.size().cols() {
            output[col] = self[(0, col)];
        }

        if is_vertical {
            output.transpose();
        }
        Ok(output)
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(inner: Vec<Vec<f64>>) -> Self {
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
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
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
    type Output = Matrix;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)
    }
}

// Matrix - [Matrix | Vector] = Matrix
impl<M: Matrixified> Sub<&M> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)
    }
}

// Matrix * [Matrix | Vector] = Matrix
impl<M: Matrixified> Mul<&M> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Matrix / Matrix = Matrix
impl Div for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: Self) -> Self::Output {
        if let Ok(rhs) = rhs.inverse() {
            self.m_mul(&rhs)
        } else {
            panic!("Division with null determinant");
        }
    }
}

// Matrix >>>


// <<< Vector

#[derive(Debug, Default, Clone)]
pub struct Vector {
    inner: Vec<f64>,
    transposed: bool,
    pub length: usize,
    actual_size: Size,
}

impl Vector {
    pub const fn empty() -> Self {
        Self {
            inner: vec![],
            transposed: false,
            length: 0,
            actual_size: Size::Row(0),
        }
    }

    pub fn is_transposed(&self) -> bool {
        self.transposed
    }

    pub fn length(&self) -> f64 {
        self ^ self
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
    fn fill_with(initial_size: Size, with: f64) -> Self {
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

    fn to_vector(self) -> Result<Vector, MatrixifiedError> {
        Ok(self)
    }
}

impl From<Vec<f64>> for Vector {
    fn from(inner: Vec<f64>) -> Self {
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

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match self.actual_size.is_horizontal() {
            true => self.elem((0, index)),
            false => self.elem((index, 0)),
        }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match self.actual_size.is_horizontal() {
            true => self.elem_mut((0, index)),
            false => self.elem_mut((index, 0)),
        }
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
    type Output = Vector;

    fn add(self, rhs: &M) -> Self::Output {
        let output = self.m_add(rhs, Sign::Plus);
        if let Ok(output) = output.to_vector() {
            output
        } else {
            panic!("An error while converting 1-dim Matrix into Vector");
        }
    }
}

// Vector - [Matrix | Vector] = Vector
impl<M: Matrixified> Sub<&M> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &M) -> Self::Output {
        let output = self.m_add(rhs, Sign::Minus);
        if let Ok(output) = output.to_vector() {
            output
        } else {
            panic!("An error while converting 1-dim Matrix into Vector");
        }
    }
}

// Vector * [Matrix | Vector] = Matrix
impl<M: Matrixified> Mul<&M> for &Vector {
    type Output = Matrix;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}


pub fn scalar_prod(lhs: &Vector, matrix: &Matrix, rhs: &Vector) -> f64 {
    let mut output;
    if lhs.size().is_vertical() {
        // if self is Col(n)
        unsafe {
            output = matrix * lhs;
            // now output is Col(n)
        }
        output.transpose();
        // now output is Row(n)
    } else {
        // if self is Row(n)
        unsafe {
            output = lhs * matrix;
            // now output is Row(n)
        }
    }
    if rhs.size().is_horizontal() {
        // if rhs is Row(n)
        output.transpose();
        (rhs * &output)[(0, 0)]
    } else {
        // if rhs is Col(n)
        (&output * rhs)[(0, 0)]
    }
}

// Vector >>>
