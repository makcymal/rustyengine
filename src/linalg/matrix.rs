use {
    std::{
        ops::{Index, IndexMut,
              Neg, Add, Sub, Mul, Div,
        },
        iter::{zip, Sum},
    },
    crate::linalg::enums::{
        MatrixError::{self, *},
    },
    num_traits as nt,
};

pub trait Num: nt::Num + nt::NumAssign + nt::NumCast + Into<f64> + Copy + Sum + Neg<Output=Self> {}
impl<T: nt::Num + nt::NumAssign + nt::NumCast + Into<f64> + Copy + Sum + Neg<Output=Self>> Num for T {}

// (rows, cols)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    pub x: usize,
    pub y: usize,
}

impl Pair {
    pub fn transpose(&mut self) {
        (self.x, self.y) = (self.y, self.x);
    }
}

impl From<(usize, usize)> for Pair {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

type Size = Pair;
type Pos = Pair;

#[inline(always)]
pub fn pow_minus<T: Num>(deg: usize) -> T {
    match deg % 2 == 0 {
        true => T::one(),
        false => -T::one(),
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    inner: Vec<Vec<T>>,
    transposed: bool,
    pub determinant: Option<T>,
    initial_size: Size,
    pub actual_size: Size,
}

impl<T: Num> Matrix<T> {
    pub fn zeros(initial_size: Size) -> Self {
        Self {
            inner: vec![vec![T::zero(); initial_size.x]; initial_size.y],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    pub fn ones(initial_size: Size) -> Result<Self, MatrixError> {
        if initial_size.x != initial_size.y {
            return Err(NonSquareMatrix);
        }
        let mut inner = vec![vec![T::zero(); initial_size.x]; initial_size.y];
        for d in 0..initial_size.y {
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

    pub fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.actual_size.transpose();
    }

    pub fn is_transposed(&self) -> bool {
        return self.transposed;
    }

    pub fn determinant(&mut self) -> Result<(), MatrixError> {
        if self.initial_size.x != self.initial_size.y {
            return Err(NonSquareMatrix);
        }

        if self.determinant.is_none() {
            let mut rows = vec![true; self.initial_size.y];
            let mut cols = vec![true; self.initial_size.x];
            self.determinant = Some(self.minor(&mut rows, &mut cols));
        }

        Ok(())
    }

    // computes inversed matrix for not-transposed matrix and then transposes it
    pub fn inverse(&self) -> Result<Self, MatrixError> {
        if self.initial_size.x != self.initial_size.y {
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

        let mut rows = vec![true; self.initial_size.y];
        let mut cols = vec![true; self.initial_size.x];

        let mut inversed = Matrix::<T>::zeros(self.initial_size);
        for row in 0..self.initial_size.y {
            cols[row] = false;
            for col in 0..self.initial_size.x {
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
        // when this code is reached, matrix surely is square
        let mut row = 0;
        while row < self.initial_size.y && rows[row] == false { row += 1 }

        if row == self.initial_size.y {
            return T::one()
        } else {
            rows[row] = false;
        }

        let mut minor = T::zero();
        for col in 0..self.initial_size.x {
            let mut j = 0;
            if cols[col] {
                cols[col] = false;
                minor +=  pow_minus::<T>(j) * self.inner[row][col] * self.minor(rows, cols);
                cols[col] = true;
            }
        }
        rows[row] = true;

        minor
    }

    pub fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|row| row.iter()
                .map(|elem| (*elem * *elem).into())
                .sum::<f64>())
            .sum::<f64>()
            .sqrt()
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        match self.transposed {
            false => &self.inner[r][c],
            true => &self.inner[c][r],
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
        match self.transposed {
            false => &mut self.inner[r][c],
            true => &mut self.inner[c][r],
        }
    }
}


impl<T: Num> Neg for Matrix<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for row in 0..self.initial_size.y {
            for col in 0..self.initial_size.x {
                self.inner[row][col] *= -T::one();
            }
        }
        self
    }
}

impl<T: Num> Add for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.actual_size != rhs.actual_size {
            return Err(InappropriateSizes);
        }

        let size = self.actual_size;

        let mut output = Matrix::<T>::zeros(size);
        for row in 0..size.y {
            for col in 0..size.x {
                output[(row, col)] = self[(row, col)] + rhs[(row, col)];
            }
        }
        Ok(output)
    }
}

impl<T: Num> Sub for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.actual_size != rhs.actual_size {
            return Err(InappropriateSizes);
        }

        let size = self.actual_size;

        let mut output = Matrix::<T>::zeros(size);
        for row in 0..size.y {
            for col in 0..size.x {
                output[(row, col)] = self[(row, col)] - rhs[(row, col)];
            }
        }
        Ok(output)
    }
}

impl<T: Num> Mul for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let output_size;
        if self.actual_size.x == rhs.actual_size.y {
            output_size = (self.actual_size.y, rhs.actual_size.x).into();
        } else {
            return Err(InappropriateSizes);
        }

        let mut output = Matrix::<T>::zeros(output_size);

        for row in 0..output_size.y {
            for col in 0..output_size.x {
                let mut elem = T::zero();
                for i in 0..self.actual_size.x {
                    elem += self[(row, i)] * rhs[(i, col)];
                }
                output[(row, col)] = elem;
            }
        }

        Ok(output)
    }
}

impl<T: Num> Div for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn div(self, rhs: Self) -> Self::Output {
        let inversed_rhs = rhs.inverse()?;
        self * &inversed_rhs
    }
}
