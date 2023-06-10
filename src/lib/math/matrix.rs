//! `Matrix` is defines as `Grid<f32>`.
//! Besides inherited methods `Matrix` also has matrices-specific methods like determinant,
//! inversed, minor, additions, subtractions, multiplications, divisions, norms. On matrices that
//! stratified into `Row` and `Col` scalar and vector products can be applied, therefore also computing
//! length and linear combinations. Provided constructors for rotation matrices

use {
    super::{
        *,
        Sign::{self, *},
    },
    crate::{
        errs::{
            GridErr::{self, *},
            MathErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        grid::*,
    },
    once_cell::sync::OnceCell,
    std::ops::{Add, Div, Mul, Neg, Sub},
};

/// Grid with `f32` numbers
pub type Matrix = Grid<f32>;

/// Common matrix methods
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
        Self::new(r, c, 0.0)
    }

    /// Determinant of square `Matrix`. If not square, `GridErr(IsNotSquare)` is returned.
    /// It doesn't matter whether `Matrix` is transposed or not
    pub fn det(&self) -> ReRes<f32> {
        self.ag_failed()?.ag_not_square()?;
        let mut rows = vec![true; self.rows()];
        let mut cols = vec![true; self.cols()];
        Ok(round(self.minor(&mut rows, &mut cols)))
    }

    /// Inversed `Matrix::Square` for square `Matrix` with non-null determinant.
    /// Unless it exists `GridErr(IsNotSquare)` or `MatrErr(NullDeterminant)` is returned
    pub fn inv(&self) -> ReRes<Self> {
        let det = self.det()?;
        if aeq(det, 0.0) {
            return Err(MathErr(NullDeterminant));
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
    pub fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> f32 {
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
                if !aeq(elem, 0.0) {
                    cols[col] = false;
                    minor += pow_minus(j) * elem * self.minor(rows, cols);
                    cols[col] = true;
                }
                j += 1;
            }
        }
        rows[row] = true;
        round(minor)
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
                let lhs = *self.rawgrid_mut().att(r, c, false);
                match sign {
                    Plus => {
                        *self.rawgrid_mut().att_mut(r, c, false) =
                            round(lhs + rhs.rawgrid_ref().att(r, c, t))
                    }
                    Minus => {
                        *self.rawgrid_mut().att_mut(r, c, false) =
                            round(lhs - rhs.rawgrid_ref().att(r, c, t))
                    }
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
                *prod.rawgrid_mut().att_mut(r, c, false) = round(
                    (0..self.cols())
                        .map(|i| {
                            self.rawgrid_ref().att(r, i, false) * rhs.rawgrid_ref().att(i, c, t)
                        })
                        .sum(),
                )
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
                *prod.rawgrid_mut().att_mut(r, c, false) = round(
                    (0..lhs.rawgrid_ref().cols(t))
                        .map(|i| {
                            lhs.rawgrid_ref().att(r, i, t) * self.rawgrid_ref().att(i, c, false)
                        })
                        .sum(),
                )
            }
        }
        prod
    }

    /// Whether both operands aren't `Matrix::Failure`'s and both operands have exactly the same sizes
    pub fn approve_add(&self, rhs: &Self, t: bool) -> ReRes<()> {
        self.approve_ops(rhs)?;
        if self.rows() != rhs.rawgrid_ref().rows(t) || self.cols() != rhs.rawgrid_ref().cols(t) {
            return Err(MathErr(AddSizesMismatch {
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
            return Err(MathErr(MulSizesMismatch {
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
            return Err(MathErr(MulSizesMismatch {
                lhs: (lhs.rawgrid_ref().rows(t), lhs.rawgrid_ref().cols(t)),
                rhs: (self.rows(), self.cols()),
            }));
        }
        Ok(())
    }

    /// Multiplies all the elements by the given number on the cloned `self`
    pub fn num_mul(&self, num: f32) -> Self {
        self.clone().raw_num_mul(num)
    }

    /// Multiplies all the elements by the given number on place
    pub fn num_mul_assign(mut self, num: f32) -> Self {
        self.raw_num_mul(num)
    }

    /// Divides all the elements by the given number on the cloned `self`
    pub fn num_div(&self, num: f32) -> Self {
        if aeq(num, 0.0) {
            return Self::Failure(MathErr(ZeroDivision));
        }
        self.clone().raw_num_mul(1.0 / num)
    }

    /// Divides all the elements by the given number on place
    pub fn num_div_assign(mut self, num: f32) -> Self {
        if aeq(num, 0.0) {
            return Self::Failure(MathErr(ZeroDivision));
        }
        self.raw_num_mul(1.0 / num)
    }

    /// Applies negation of all the elements
    pub fn neg(mut self) -> Self {
        self.raw_num_mul(-1.0)
    }

    /// Multiplies by the given number
    fn raw_num_mul(mut self, num: f32) -> Self {
        if self.is_failure() {
            return Self::Failure(GridErr(UnhandledFailure));
        }
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let elem = *self.rawgrid_mut().att(r, c, false);
                *self.rawgrid_mut().att_mut(r, c, false) = round(elem * num);
            }
        }
        self
    }

    /// Norm of the `Matrix` as sqrt of sum of square of elements
    pub fn norm(&self) -> ReRes<f32> {
        self.ag_failed()?;
        match self {
            Self::Arbitrary(grid)
            | Self::Square(grid)
            | Self::MultiRow(grid)
            | Self::MultiCol(grid) => Ok(round(
                (0..grid.rows(false))
                    .map(|r| {
                        (0..grid.cols(false))
                            .map(|c| grid.att(r, c, false).powi(2))
                            .sum::<f32>()
                    })
                    .sum::<f32>()
                    .sqrt(),
            )),
            Self::Row(grid) => Ok(round(
                (0..grid.cols(false))
                    .map(|c| grid.att(0, c, false).powi(2))
                    .sum::<f32>()
                    .sqrt(),
            )),
            Self::Col(grid) => Ok(round(
                (0..grid.rows(false))
                    .map(|r| grid.att(r, 0, false).powi(2))
                    .sum::<f32>()
                    .sqrt(),
            )),
            _ => unreachable!(),
        }
    }

    /// Whether `self` element-wise approximate equals to `other` treating the repr
    pub fn aeq(&self, other: &Self) -> bool {
        let p = |lhs: f32, rhs: f32| aeq(lhs, rhs);
        if self.repr() == other.repr() {
            self.rawgrid_ref().eqp(other.rawgrid_ref(), p)
        } else {
            false
        }
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

/// Rotations
impl Matrix {
    /// Rotation matrix in n-dim space on the given angle
    pub fn rotation(mut from: usize, mut to: usize, mut angle: f32, dim: usize) -> Self {
        let mut matr = Self::identity(dim);
        if from == to {
            return Self::Failure(MathErr(RotationInOneAxis(from)));
        } else if from > to {
            (from, to) = (to, from);
            angle = -angle;
        }
        let (sin, cos) = (angle.sin(), angle.cos());
        *matr.att_mut(from, from) = cos;
        *matr.att_mut(from, to) = -sin;
        *matr.att_mut(to, from) = sin;
        *matr.att_mut(to, to) = cos;
        matr
    }

    /// Suppose it's needed to rotate vector on `from` axis and then extend it so theq would compose
    /// right triangle. This function constructs such rotation matrix
    pub fn triag_rotation(mut from: usize, mut to: usize, mut angle: f32, dim: usize) -> Self {
        let mut matr = Self::identity(dim);
        if from == to {
            return Self::Failure(MathErr(RotationInOneAxis(from)));
        } else if from > to {
            (from, to) = (to, from);
            angle = -angle;
        }
        let (sin, cos) = (angle.sin(), angle.cos());
        *matr.att_mut(from, from) = cos.signum();
        *matr.att_mut(from, to) = -sin / cos;
        *matr.att_mut(to, from) = sin.signum();
        *matr.att_mut(to, to) = cos / sin;
        matr
    }

    /// Rotation matrix in 3-dim space on the 3 given angles around cardinal axes
    pub fn teit_bryan_rotation(x: f32, y: f32, z: f32) -> Self {
        Self::rotation(1, 2, x, 3)
            .mul(&Self::rotation(0, 2, -y, 3))
            .mul(&Self::rotation(0, 1, z, 3))
    }
}

/// All methods related to representation `Row`, `Col`, `MultiRow` or `MultiCol`
impl Matrix {
    /// Constructor for column
    pub fn col(comp: Vec<f32>) -> Self {
        Self::from_single(comp).raw_transpose().to_col()
    }

    /// Linear combination of rows or cols producing single row or col
    pub fn combine(&self, coef: Vec<f32>) -> ReRes<Self> {
        self.ag_failed()?.ag_not_stratified()?;
        match self.repr() {
            Repr::Row | Repr::Col => Ok(self.num_mul(coef[0])),
            Repr::MultiRow => {
                let mut comb = Self::zero(1, self.cols()).to_row();
                for c in 0..self.cols() {
                    for r in 0..self.rows() {
                        *comb.at_mut(c) += *self.att(r, c) * coef[r];
                    }
                }
                Ok(comb.round())
            }
            Repr::MultiCol => {
                let mut comb = Self::zero(1, self.rows()).transpose().to_col();
                for r in 0..self.rows() {
                    for c in 0..self.cols() {
                        *comb.at_mut(r) += *self.att(c, r) * coef[c];
                    }
                }
                Ok(comb)
            }
            _ => unreachable!(),
        }
    }

    /// Whether `self` contains only one `Row` or `Col`
    pub fn approve_single_vector(&self) -> ReRes<()> {
        self.ag_failed()?.ag_not_stratified()?;
        match self.repr() {
            Repr::Row | Repr::MultiRow => self.ag_too_many_rows()?,
            Repr::Col | Repr::MultiCol => self.ag_too_many_cols()?,
            _ => unreachable!(),
        };
        Ok(())
    }

    /// Whether both operands aren't `Matrix::Failure`'s, have the same dim and contains only one `Row` or `Col`
    pub fn approve_single_vector_ops(&self, rhs: &Self) -> ReRes<()> {
        self.approve_ops(rhs)?;
        self.approve_single_vector()?;
        rhs.approve_single_vector()?;
        if self.dim() != rhs.dim() {
            return Err(MathErr(DimMismatch {
                lhs: self.dim().unwrap(),
                rhs: rhs.dim().unwrap(),
            }));
        }
        Ok(())
    }

    /// Whether both operands aren't `Matrix::Failure`'s, have the same dim and stratified to rows or cols
    pub fn approve_multi_vector_ops(&self, rhs: &Self) -> ReRes<()> {
        self.approve_ops(rhs)?;
        self.ag_not_stratified()?;
        rhs.ag_not_stratified()?;
        if self.dim() != rhs.dim() {
            return Err(MathErr(DimMismatch {
                lhs: self.dim().unwrap(),
                rhs: rhs.dim().unwrap(),
            }));
        }
        Ok(())
    }

    /// How many elements contains such `Vector` as `Row`, `Col` or in `MultiRow`, `MultiCol`
    pub fn dim(&self) -> ReRes<usize> {
        self.ag_failed()?.ag_not_stratified()?;
        match self {
            Self::Row(_) | Self::MultiRow(_) => Ok(self.cols()),
            Self::Col(_) | Self::MultiCol(_) => Ok(self.rows()),
            _ => unreachable!(),
        }
    }

    /// Orthonorm length of `Row` or `Col` without basis according only to `BIFORM` matrix
    pub fn len(&self) -> ReRes<f32> {
        Ok(round(self.scalar_prod(self)?.sqrt()))
    }

    /// Orthonorm length of vector in the given index `s` in `Row`, `MultiRow`, `Col` or `MultiCol`
    /// without basis according only to `BIFORM` matrix
    pub fn len_at(&self, s: usize) -> ReRes<f32> {
        Ok(round(self.scalar_prod_at(s, self, s)?.sqrt()))
    }

    /// Resizing each vector to length 1 using scalar product without basis
    pub fn normalize(mut self) -> Self {
        match self.repr() {
            Repr::Row | Repr::MultiRow => {
                for r in 0..self.rows() {
                    let len = self.len_at(r).unwrap();
                    for c in 0..self.cols() {
                        let elem = *self.att(r, c);
                        *self.att_mut(r, c) = round(elem / len);
                    }
                }
            }
            Repr::Col | Repr::MultiCol => {
                for c in 0..self.cols() {
                    let len = self.len_at(c).unwrap();
                    for r in 0..self.rows() {
                        let elem = *self.att(c, r);
                        *self.att_mut(c, r) = round(elem / len);
                    }
                }
            }
            _ => {}
        }
        self
    }

    /// Orthonorm scalar product without basis according only to `BIFORM` matrix.
    /// Operands must have single `Row` or `Col` having the same dim. Produces `f32`
    pub fn scalar_prod(&self, rhs: &Self) -> ReRes<f32> {
        self.approve_single_vector_ops(rhs)?;
        Ok(*self.raw_scalar_prod(rhs, biform())?.att(0, 0))
    }

    /// Orthonorm scalar product without basis according only to `BIFORM` matrix.
    /// Operands at the given indices must have the same dim.
    pub fn scalar_prod_at(&self, i: usize, rhs: &Self, j: usize) -> ReRes<f32> {
        self.raw_scalar_prod_at(i, rhs, j, biform())
    }

    /// Orthonorm scalar product without basis according only to `BIFORM` matrix.
    /// Operands must have `Row` or `Col` of the same dim.
    /// Produces `Arbitrary` matrix of `f32`, that is pair-wise scalar products
    pub fn multi_scalar_prod(&self, rhs: &Self) -> ReRes<Self> {
        self.raw_scalar_prod(rhs, biform())
    }

    /// Scalar product according to given `core` matrix.
    /// Operands must have `Row` or `Col` of the same dim.
    /// Produces `Arbitrary` matrix of `f32`, that is pair-wise scalar products
    pub(super) fn raw_scalar_prod(&self, rhs: &Self, core: &Self) -> ReRes<Self> {
        self.approve_multi_vector_ops(rhs)?;
        let lhs = match self.repr() {
            Repr::Row | Repr::MultiRow => self.mul(core),
            Repr::Col | Repr::MultiCol => self.mul_left_t(core).transpose(),
            _ => unreachable!(),
        };
        Ok(match rhs.repr() {
            Repr::Col | Repr::MultiCol => lhs.mul(rhs),
            Repr::Row | Repr::MultiRow => lhs.mul_t(rhs),
            _ => unreachable!(),
        })
    }

    /// Scalar product according to given `core` matrix.
    /// Operands at the given indices must have the same dim.
    /// Between `Row`'s or `Col`'s produces `f32`
    pub(super) fn raw_scalar_prod_at(
        &self,
        s: usize,
        rhs: &Self,
        r: usize,
        core: &Self,
    ) -> ReRes<f32> {
        self.approve_multi_vector_ops(rhs)?;
        Ok(round(
            (0..self.dim().unwrap())
                .map(|i| {
                    self.att(s, i)
                        * (0..rhs.dim().unwrap())
                            .map(|j| core.att(i, j) * rhs.att(r, j))
                            .sum::<f32>()
                })
                .sum::<f32>(),
        ))
    }

    /// Orthonorm vector product
    pub fn vector_prod(&self, rhs: &Self) -> ReRes<Self> {
        self.approve_single_vector_ops(rhs)?;
        self.vector_prod_at(0, rhs, 0)
    }

    /// Orthonorm vector product without basis according only to `BIFORM` matrix between vectors on given indices
    pub fn vector_prod_at(&self, s: usize, rhs: &Self, r: usize) -> ReRes<Self> {
        self.approve_multi_vector_ops(rhs)?;
        self.ag_not_3_dim()?;
        Ok(Self::from_single(vec![
            round(self.att(s, 1) * rhs.att(r, 2) - self.att(s, 2) * rhs.att(r, 1)),
            round(self.att(s, 2) * rhs.att(r, 0) - self.att(s, 0) * rhs.att(r, 2)),
            round(self.att(s, 0) * rhs.att(r, 1) - self.att(s, 1) * rhs.att(r, 0)),
        ])
        .raw_transpose()
        .to_col())
    }

    /// Doesn't pass any matrix that are not `Row`, `Col`, `MultiRow` or `MultiCol` or have dimension inequal to 3
    pub fn ag_not_3_dim(&self) -> ReRes<&Self> {
        match self.dim() {
            Ok(3) => Ok(self),
            Err(err) => Err(err),
            _ => Err(MathErr(NotIn3Dim)),
        }
    }

    /// Doesn't pass matrices of null determinant
    pub fn ag_linear_dependence(&self) -> ReRes<&Self> {
        if let Ok(det) = self.det() {
            if det == 0.0 {
                return Err(MathErr(NullDeterminant));
            }
        }
        Ok(self)
    }
}

static mut BIFORM: OnceCell<Matrix> = OnceCell::new();

pub fn set_biform(biform: Matrix) {
    unsafe {
        BIFORM.take();
        BIFORM.set(biform).expect("BIFORM init failed");
    }
}

pub fn set_biform_vec(double: Vec<Vec<f32>>) {
    unsafe {
        BIFORM.take();
        BIFORM
            .set(Matrix::from_double(double))
            .expect("BIFORM init failed");
    }
}

pub fn set_biform_identity() {
    unsafe {
        BIFORM.take();
        BIFORM.set(Matrix::identity(3)).expect("BIFORM init failed");
    }
}

fn biform() -> &'static Matrix {
    unsafe { BIFORM.get().expect("BIFORM isn't initialized") }
}
