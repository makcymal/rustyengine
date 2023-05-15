use {
    crate::{
        grid::{
            RawGrid,
            Repr,
            Grid,
            Line,
            Elem,
        },
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MatrixErr::{self, *},
        },
        util::{
            pow_minus,
            Sign::{self, *},
            LineTp
        },
    },
    super::{
        BIFORM, get_biform,
        prec::{round, round_prec},
    },
    std::ops::{
        Add, Sub, Mul, Div, Neg,
    },
};

/// Grid with `f64` numbers
pub type Matrix = Grid<f64>;

impl Matrix {
    /// Square `Matrix` with 1 on the diagonal and 0 elsewhere
    pub fn identity(side: usize) -> Self {
        let mut id = Self::zero(side, side).to_square();
        for d in 0..side {
            *id.att_mut(d, d) = 1.0;
        }
        id
    }

    /// Arbitrary `Matrix`, full of zeros of `r` rows and `c` cols
    pub fn zero(r: usize, c: usize) -> Self {
        Self::fill(r, c, 0.0)
    }

    /// Determinant of square `Matrix`. If not square, `GridErr(IsNotSquare)` is returned.
    /// It doesn't matter whether `Matrix` is transposed or not
    pub fn det(&self) -> ReRes<f64> {
        if self.failed() {
            return Err(GridErr(UnhandledFailure));
        }
        if self.rows() != self.cols() {
            return Err(GridErr(NotSquare((self.rows(), self.cols()))));
        }
        let mut rows = vec![true; self.rows()];
        let mut cols = vec![true; self.cols()];
        Ok(round(self.minor(&mut rows, &mut cols)))
    }

    /// Inversed `Matrix::Square` for square `Matrix` with non-null determinant.
    /// Unless it exists `GridErr(IsNotSquare)` or `MatrErr(NullDeterminant)` is returned
    pub fn inv(&self) -> ReRes<Self> {
        let det = self.det()?;
        if det == 0.0 {
            return Err(MatrixErr(NullDeterminant));
        }

        let mut rows = vec![true; self.rows()];
        let mut cols = vec![true; self.cols()];

        let mut inversed = Matrix::zero(self.rows(), self.cols());
        for row in 0..self.rows() {
            cols[row] = false;
            for col in 0..self.cols() {
                rows[col] = false;
                *inversed.att_mut(row, col) =
                    round(pow_minus(row + col) * self.minor(&mut rows, &mut cols) / det);
                rows[col] = true;
            }
            cols[row] = true;
        }
        Ok(inversed)
    }

    /// Minor based on ignored rows and columns, computed recursively.
    /// `rows` and `cols` must contain equal number of `true`s
    pub fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> f64 {
        let row = rows.iter().position(|&x| x);
        if row.is_none() {
            return 1.0;
        }
        let row = row.unwrap();
        rows[row] = false;

        let mut minor = 0.0;
        let mut j = 0;
        for col in 0..self.cols() {
            if cols[col] {
                let elem = *self.att(row, col);
                if elem != 0.0 {
                    cols[col] = false;
                    minor += pow_minus(j) * elem * self.minor(rows, cols);
                    cols[col] = true;
                }
                j += 1;
            }
        }
        rows[row] = true;
        minor
    }

    /// Rounds all the elements with precision specified in `math::prec`
    pub fn round(mut self) -> Self {
        for r in 0..self.rawgrid_ref().rows(false) {
            for c in 0..self.rawgrid_ref().cols(false) {
                let elem = *self.rawgrid_ref().att(r, c, false);
                *self.rawgrid_mut().att_mut(r, c, false) = round(elem);
            }
        }
        self
    }

    /// Rounds all the elements with the given precision
    pub fn round_prec(mut self, prec: u16) -> Self {
        for r in 0..self.rawgrid_ref().rows(false) {
            for c in 0..self.rawgrid_ref().cols(false) {
                let elem = *self.rawgrid_ref().att(r, c, false);
                *self.rawgrid_mut().att_mut(r, c, false) = round_prec(elem, prec);
            }
        }
        self
    }

    /// Element-wise sum of two `Matrix`'s
    pub fn add(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Plus, false)
    }

    /// Element-wise sum of two `Matrix`'s as if RHS operands was transposed
    pub fn add_t(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Plus, true)
    }

    /// Element-wise addition to the given `Matrix` of another `Matrix`
    pub fn add_assign(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Plus, false)
    }

    /// Element-wise addition to the given `Matrix` of another `Matrix` as if RHS operands was transposed
    pub fn add_assign_t(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Plus, true)
    }

    /// Element-wise difference of two `Matrix`'s
    pub fn sub(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Minus, false)
    }

    /// Element-wise difference of two `Matrix`'s as if RHS operands was transposed
    pub fn sub_t(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Minus, true)
    }

    /// Element-wise subtraction to the given `Matrix` of another `Matrix`
    pub fn sub_assign(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Minus, false)
    }

    /// Element-wise subtracion to the given `Matrix` of another `Matrix` as if RHS operands was transposed
    pub fn sub_assign_t(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Minus, true)
    }

    /// Element-wise add or sub for transposed and not RHS operands
    fn raw_add(mut self, rhs: &Self, sign: Sign, t: bool) -> Self {
        if let Err(err) = self.approve_add(rhs, t) {
            return Self::Failure(err);
        }

        for r in 0..self.rows() {
            for c in 0..self.cols() {
                match sign {
                    Plus => *self.rawgrid_mut().att_mut(r, c, false) += rhs.rawgrid_ref().att(r, c, t),
                    Minus => *self.rawgrid_mut().att_mut(r, c, false) -= rhs.rawgrid_ref().att(r, c, t)
                }
            }
        }
        self
    }

    /// `Matrix` multiplication
    pub fn mul(&self, rhs: &Self) -> Self {
        self.raw_mul(rhs, false)
    }

    /// `Matrix` multiplication as if RHS operand was transposed
    pub fn mul_t(&self, rhs: &Self) -> Self {
        self.raw_mul(rhs, true)
    }

    /// `Matrix` multiplication to left from the `self`
    pub fn mul_left(&self, lhs: &Self) -> Self {
        self.raw_mul_left(lhs, false)
    }

    /// `Matrix` multiplication to left from the `self` as if RHS operand was transposed
    pub fn mul_left_t(&self, lhs: &Self) -> Self {
        self.raw_mul_left(lhs, true)
    }

    /// `Matrix` division by square `Matrix` of non-null determinant
    pub fn div(&self, rhs: &Self) -> Self {
        match rhs.inv() {
            Ok(inv) => self.raw_mul(&inv, false),
            _ => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    /// `Matrix` division by square `Matrix` of non-null determinant as if RHS operand was transposed
    pub fn div_t(&self, rhs: &Self) -> Self {
        match rhs.inv() {
            Ok(inv) => self.raw_mul(&inv, true),
            _ => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    /// `Matrix` multiplication with RHS operand transposed or not
    fn raw_mul(&self, rhs: &Self, t: bool) -> Self {
        if let Err(err) = self.approve_mul(rhs, t) {
            return Self::Failure(err);
        }

        let rows = self.rows();
        let cols = rhs.rawgrid_ref().cols(t);
        let mut prod = Self::zero(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
                *prod.rawgrid_mut().att_mut(r, c, false) =
                    (0..self.cols())
                        .map(|i| self.rawgrid_ref().att(r, i, false) * rhs.rawgrid_ref().att(i, c, t))
                        .sum()
            }
        }
        prod
    }

    /// `Matrix` multiplication to the left with RHS operand transposed or not
    fn raw_mul_left(&self, lhs: &Self, t: bool) -> Self {
        if let Err(err) = self.approve_mul_left(lhs, t) {
            return Self::Failure(err);
        }

        let rows = lhs.rawgrid_ref().rows(t);
        let cols = self.cols();
        let mut prod = Self::zero(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
                *prod.rawgrid_mut().att_mut(r, c, false) =
                    (0..lhs.rawgrid_ref().cols(t))
                        .map(|i| lhs.rawgrid_ref().att(r, i, t) * self.rawgrid_ref().att(i, c, false))
                        .sum()
            }
        }
        prod
    }

    /// Whether both operands aren't `Matrix::Failure`'s and both operands have exactly the same sizes
    pub fn approve_add(&self, rhs: &Self, t: bool) -> ReRes<()> {
        self.approve_ops(rhs)?;
        if self.rows() != rhs.rawgrid_ref().rows(t) || self.cols() != rhs.rawgrid_ref().cols(t) {
            return Err(MatrixErr(AddSizesMismatch {
                lhs: (self.rows(), self.cols()),
                rhs: (rhs.rawgrid_ref().rows(t), rhs.rawgrid_ref().cols(t)),
            }));
        }
        Ok(())
    }

    /// Whether both operands aren't `Matrix::Failure`'s and `self.cols() == rhs.rows()`
    pub fn approve_mul(&self, rhs: &Self, t: bool) -> ReRes<()> {
        self.approve_ops(rhs)?;
        if self.cols() != rhs.rawgrid_ref().rows(t) {
            return Err(MatrixErr(MulSizesMismatch {
                lhs: (self.rows(), self.cols()),
                rhs: (rhs.rawgrid_ref().rows(t), rhs.rawgrid_ref().cols(t)),
            }));
        }
        Ok(())
    }

    /// Whether both operands aren't `Matrix::Failure`'s and `self.rows() == lhs.cols()`
    pub fn approve_mul_left(&self, lhs: &Self, t: bool) -> ReRes<()> {
        self.approve_ops(lhs)?;
        if lhs.rawgrid_ref().cols(t) != self.rows() {
            return Err(MatrixErr(MulSizesMismatch {
                lhs: (lhs.rawgrid_ref().rows(t), lhs.rawgrid_ref().cols(t)),
                rhs: (self.rows(), self.cols()),
            }));
        }
        Ok(())
    }

    /// Multiplies all the elements by the given number on the cloned `self`
    pub fn num_mul(&self, num: f64) -> Self {
        self.clone().raw_num_mul(num)
    }

    /// Multiplies all the elements by the given number on place
    pub fn num_mul_assign(mut self, num: f64) -> Self {
        self.raw_num_mul(num)
    }

    /// Divides all the elements by the given number on the cloned `self`
    pub fn num_div(&self, num: f64) -> Self {
        if num == 0.0 {
            return Self::Failure(MatrixErr(ZeroDivision));
        }
        self.clone().raw_num_mul(1.0 / num)
    }

    /// Divides all the elements by the given number on place
    pub fn num_div_assign(mut self, num: f64) -> Self {
        if num == 0.0 {
            return Self::Failure(MatrixErr(ZeroDivision));
        }
        self.raw_num_mul(1.0 / num)
    }

    /// Applies negation of all the elements
    pub fn neg(mut self) -> Self {
        self.raw_num_mul(-1.0)
    }

    /// Multiplies by the given number
    fn raw_num_mul(mut self, num: f64) -> Self {
        if self.failed() {
            return Self::Failure(GridErr(UnhandledFailure));
        }
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                *self.rawgrid_mut().att_mut(r, c, false) *= num;
            }
        }
        self
    }

    /// Norm of the `Matrix` as sqrt of sum of square of elements
    pub fn norm(&self) -> ReRes<f64> {
        match self {
            Self::Arbitrary(grid) | Self::Square(grid) | Self::MultiRow(grid) | Self::MultiCol(grid) => {
                Ok((0..grid.rows(false))
                    .map(|r| (0..grid.cols(false))
                        .map(|c| grid.att(r, c, false).powi(2))
                        .sum::<f64>())
                    .sum::<f64>()
                    .sqrt())
            }
            Self::Row(grid) => {
                Ok((0..grid.cols(false))
                    .map(|c| grid.att(0, c, false).powi(2))
                    .sum::<f64>()
                    .sqrt())
            }
            Self::Col(grid) => {
                Ok((0..grid.rows(false))
                    .map(|r| grid.att(r, 0, false).powi(2))
                    .sum::<f64>()
                    .sqrt())
            }
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
        }
    }

    // /// Scalar
    // pub fn scalar_prod(&self, rhs: &Self) -> ReRes<f64> {
    //     self.approve_vector_ops(rhs)?;
    //     Ok((0..self.dim().unwrap())
    //         .map(|i| self.at(i) * rhs.at(i))
    //         .sum())
    // }
    //
    // pub fn vector_prod(&self, rhs: &Self) -> ReRes<Self> {
    //     self.approve_vector_ops(rhs)?;
    //     if self.dim() != Ok(3) {
    //         return Err(MatrixErr(VectorProdDimMismatch { lhs: self.dim().unwrap(), rhs: rhs.dim().unwrap() }));
    //     }
    //     Ok(Self::from_single(vec![
    //         self.at(1) * rhs.at(2) - self.at(2) * rhs.at(1),
    //         self.at(2) * rhs.at(0) - self.at(0) * rhs.at(2),
    //         self.at(0) * rhs.at(1) - self.at(1) * rhs.at(0),
    //     ]).raw_transpose().to_col())
    // }
    //
    // pub fn vector_prod_of(&self, i: usize, rhs: &Self, j: usize) -> ReRes<Self> {
    //     self.approve_ops(other)?;
    //     if !self.is_multirow() && !self.is_multicol() || !other.is_rowlist() && !other.is_collist() {
    //         return Err(GridErr(NotMultiRowOrCol));
    //     } else if self.dim() != Ok(3) || other.dim() != Ok(3) {
    //         return Err(MatrixErr(VectorProdDimMismatch { lhs: self.dim().unwrap(), rhs: rhs.dim().unwrap() }));
    //     }
    //     Ok(Self::from_single(vec![
    //         self.att(i, 1) * rhs.att(j, 2) - self.att(i, 2) * rhs.att(j, 1),
    //         self.att(i, 2) * rhs.att(j, 0) - self.att(i, 0) * rhs.att(j, 2),
    //         self.att(i, 0) * rhs.att(j, 1) - self.att(i, 1) * rhs.att(j, 0),
    //     ]).raw_transpose().to_col())
    // }
}

impl<'g> Matrix {
    /// `Vector` instance pointing to the `Row` or `Col` at the given idx
    pub fn vector(&'g self, idx: usize) -> Vector<'g> {
        Vector::new(self, idx)
    }

    /// Whether both operands aren't `Matrix::Failure`'s and have the same dim
    pub fn approve_vector_ops(&self, other: &Self) -> ReRes<()> {
        self.approve_ops(other)?;
        if !self.is_row() && !self.is_col() || !other.is_row() && !other.is_col() {
            return Err(GridErr(NotRowOrCol));
        }
        else if self.dim() != other.dim() {
            return Err(MatrixErr(ScalarProdDimMismatch { lhs: self.dim().unwrap(), rhs: other.dim().unwrap() }));
        }
        Ok(())
    }

    /// Whether `self` contains only one `Row` or `Col`
    pub fn approve_single_vector(&self) -> ReRes<()> {
        match self.repr() {
            Repr::Row | Repr::MultiRow => {
                if self.rows() != 1 {
                    return Err(GridErr(TooManyRows(self.rows())))
                }
            },
            Repr::Col | Repr::MultiCol => {
                if self.rows() != 1 {
                    return Err(GridErr(TooManyCols(self.cols())))
                }
            },
            _ => return Err(GridErr(NotRowOrCol))
        }
        Ok(())
    }

    /// How many elements contains such `Vector` as `Row`, `Col` or in `MultiRow`, `MultiCol`
    pub fn dim(&self) -> ReRes<usize> {
        match self {
            Self::Row(_) | Self::MultiRow(_) => {
                Ok(self.cols())
            }
            Self::Col(_) | Self::MultiCol(_) => {
                Ok(self.rows())
            }
            Self::Arbitrary(_) | Self::Square(_) => Err(GridErr(NotRowOrCol)),
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
        }
    }

    /// Orthonorm length of `Vector` without basis according only to `BIFORM` matrix
    pub fn len(&self) -> ReRes<f64> {
        match self {
            Self::Row(_) => Ok(self.mul(get_biform()).mul_t(self).att(0, 0).sqrt()),
            Self::Col(_) => Ok(self.mul_left_t(get_biform()).transpose().mul(self).att(0, 0).sqrt()),
            Self::Failure(_) => Err(GridErr(UnhandledFailure)),
            _ => Err(GridErr(NotRowOrCol)),
        }
    }

    /// Scalar product without basis according only to `BIFORM` matrix.
    /// Operands must have single `Row` or `Col` having the same dim. Produces `f64`
    pub fn scalar_prod(&self, rhs: &Self) -> ReRes<f64> {
        self.approve_single_vector()?;
        rhs.approve_single_vector()?;
        Ok(*self.raw_scalar_prod(rhs, get_biform())?.att(0, 0))
    }

    /// Scalar product without basis according only to `BIFORM` matrix.
    /// Operands at the given indices must have the same dim.
    /// Between `Row`'s or `Col`'s produces `Col`
    pub fn scalar_prod_at(&self, i: usize, rhs: &Self, j: usize) -> ReRes<f64> {
        self.raw_scalar_prod_at(i, rhs, j, get_biform())
    }

    /// Scalar product without basis according only to `BIFORM` matrix.
    /// Operands must have `Row` or `Col` of the same dim.
    /// Produces `Arbitrary` matrix of `f64`, that is pair-wise scalar products
    pub fn multi_scalar_prod(&self, rhs: &Self) -> ReRes<Self> {
        self.raw_scalar_prod(rhs, get_biform())
    }

    /// Scalar product without basis according to given `core` matrix.
    /// Operands must have `Row` or `Col` of the same dim.
    /// Produces `Arbitrary` matrix of `f64`, that is pair-wise scalar products
    pub(in super) fn raw_scalar_prod(&self, rhs: &Self, core: &Self) -> ReRes<Self> {
        self.approve_vector_ops(rhs)?;
        let lhs = match self.repr() {
            Repr::Row | Repr::MultiRow => self.mul(core),
            Repr::Col | Repr::MultiCol => self.mul_left_t(core).transpose(),
            _ => unreachable!()
        };
        Ok(match rhs.repr() {
            Repr::Col | Repr::MultiCol => lhs.mul(rhs),
            Repr::Row | Repr::MultiRow => lhs.mul_t(rhs),
            _ => unreachable!()
        })
    }

    /// Scalar product without basis according to given `core` matrix.
    /// Operands at the given indices must have the same dim.
    /// Between `Row`'s or `Col`'s produces `f64`
    pub(in super) fn raw_scalar_prod_at(&self, s: usize, rhs: &Self, r: usize, core: &Self) -> ReRes<f64> {
        self.approve_vector_ops(rhs)?;
        Ok((0..self.dim().unwrap())
            .map(|i| {
                (0..rhs.dim().unwrap())
                    .map(|j| core.att(i, j) * rhs.att(r, j))
                    .sum::<f64>()})
            .sum::<f64>()
        )
    }


}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}


pub type Vector<'g> = Line<'g, f64>;
