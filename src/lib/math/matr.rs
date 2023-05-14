use {
    crate::{
        grid::{RawGrid, Grid},
        errs::{
            AnyRes,
            AnyErr::{self, *},
            GridErr::{self, *},
            MatrErr::{self, *},
        },
        util::{
            pow_minus,
            Sign::{self, *},
        },
    },
    super::prec::{round, round_prec},
};

pub type Matr = Grid<f64>;

impl Matr {
    pub fn identity(side: usize) -> Self {
        let mut id = Self::zero(side, side);
        for d in 0..side {
            *id.att_mut(d, d) = 1.0;
        }
        id
    }

    pub fn zero(r: usize, c: usize) -> Self {
        Self::fill_with(r, c, 0.0)
    }

    /// Determinant of square `Matr`. If not square, `MatrErr(DeterminantOfNonSquare)` is returned.
    /// It doesn't matter whether `Matr` is transposed or not
    pub fn determinant(&self) -> AnyRes<f64> {
        if self.is_failure() {
            return Err(GridErr(UnhandledFailure));
        }
        if self.rows() != self.cols() {
            return Err(MatrErr(DeterminantOfNonSquare((self.rows(), self.cols()))));
        }
        let mut rows = vec![true; self.rows()];
        let mut cols = vec![true; self.cols()];
        Ok(round(self.minor(&mut rows, &mut cols)))
    }

    /// Inversed `Matr::Matrix` for square `Matr` with non-null determinant.
    /// Unless it exists `MatrErr(DeterminantOfNonSquare)` or `MatrErr(NullDeterminant)` is returned
    pub fn inversed(&self) -> AnyRes<Self> {
        let det = self.determinant()?;
        if det == 0.0 {
            return Err(MatrErr(NullDeterminant));
        }

        let mut rows = vec![true; self.rows()];
        let mut cols = vec![true; self.cols()];

        let mut inversed = Matr::zero(self.rows(), self.cols());
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

    pub fn round(mut self) -> Self {
        for r in 0..self.rawgrid().rows(false) {
            for c in 0..self.rawgrid().cols(false) {
                let elem = *self.rawgrid().att(r, c, false);
                *self.rawgrid_mut().att_mut(r, c, false) = round(elem);
            }
        }
        self
    }

    pub fn round_prec(mut self, prec: u16) -> Self {
        for r in 0..self.rawgrid().rows(false) {
            for c in 0..self.rawgrid().cols(false) {
                let elem = *self.rawgrid().att(r, c, false);
                *self.rawgrid_mut().att_mut(r, c, false) = round_prec(elem, prec);
            }
        }
        self
    }

    pub fn add(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Plus, false)
    }

    pub fn add_t(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Plus, true)
    }

    pub fn add_assign(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Plus, false)
    }

    pub fn add_assign_t(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Plus, true)
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Minus, false)
    }

    pub fn sub_t(&self, rhs: &Self) -> Self {
        self.clone().raw_add(rhs, Minus, true)
    }

    pub fn sub_assign(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Minus, false)
    }

    pub fn sub_assign_t(mut self, rhs: &Self) -> Self {
        self.raw_add(rhs, Minus, true)
    }

    fn raw_add(mut self, rhs: &Self, sign: Sign, t: bool) -> Self {
        if let Err(err) = self.approve_add(rhs, t) {
            return Self::Failure(err);
        }

        for r in 0..self.rows() {
            for c in 0..self.cols() {
                match sign {
                    Plus => *self.rawgrid_mut().att_mut(r, c, false) += rhs.rawgrid().att(r, c, t),
                    Minus => *self.rawgrid_mut().att_mut(r, c, false) -= rhs.rawgrid().att(r, c, t)
                }
            }
        }
        self
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        self.raw_mul(rhs, false)
    }

    pub fn mul_t(&self, rhs: &Self) -> Self {
        self.raw_mul(rhs, true)
    }

    pub fn div(&self, rhs: &Self) -> Self {
        match rhs.inversed() {
            Ok(inv) => self.raw_mul(&inv, false),
            _ => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    pub fn div_t(&self, rhs: &Self) -> Self {
        match rhs.inversed() {
            Ok(inv) => self.raw_mul(&inv, true),
            _ => Self::Failure(GridErr(UnhandledFailure)),
        }
    }

    fn raw_mul(&self, rhs: &Self, t: bool) -> Self {
        if let Err(err) = self.approve_mul(rhs, t) {
            return Self::Failure(err);
        }

        let rows = self.rows();
        let cols = rhs.rawgrid().cols(t);
        let mut prod = Self::zero(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
                *prod.rawgrid_mut().att_mut(r, c, false) =
                    (0..self.cols())
                        .map(|i| self.rawgrid().att(r, i, false) * rhs.rawgrid().att(i, c, t))
                        .sum()
            }
        }
        prod
    }

    pub fn approve_ops(&self, rhs: &Self) -> AnyRes<()> {
        if self.is_failure() || rhs.is_failure() {
            return Err(GridErr(UnhandledFailure));
        }
        Ok(())
    }

    pub fn approve_add(&self, rhs: &Self, t: bool) -> AnyRes<()> {
        self.approve_ops(rhs)?;
        if self.rows() != rhs.rawgrid().rows(t) || self.cols() != rhs.rawgrid().cols(t) {
            return Err(MatrErr(AddSizesMismatch {
                lhs: (self.rows(), self.cols()),
                rhs: (rhs.rawgrid().rows(t), rhs.rawgrid().cols(t)),
            }));
        }
        Ok(())
    }

    pub fn approve_mul(&self, rhs: &Self, t: bool) -> AnyRes<()> {
        self.approve_ops(rhs)?;
        if self.cols() != rhs.rawgrid().rows(t) {
            return Err(MatrErr(MulSizesMismatch {
                lhs: (self.rows(), self.cols()),
                rhs: (rhs.rawgrid().rows(t), rhs.rawgrid().cols(t)),
            }));
        }
        Ok(())
    }

    pub fn num_mul(&self, num: f64) -> Self {
        self.clone().raw_num_mul(num)
    }

    pub fn num_mul_assign(mut self, num: f64) -> Self {
        self.raw_num_mul(num)
    }

    pub fn num_div(&self, num: f64) -> Self {
        if num == 0.0 {
            return Self::Failure(MatrErr(ZeroDivision))
        }
        self.clone().raw_num_mul(1.0 / num)
    }

    pub fn num_div_assign(mut self, num: f64) -> Self {
        if num == 0.0 {
            return Self::Failure(MatrErr(ZeroDivision))
        }
        self.raw_num_mul(1.0 / num)
    }

    pub fn neg(mut self) -> Self {
        self.raw_num_mul(-1.0)
    }

    fn raw_num_mul(mut self, num: f64) -> Self {
        if self.is_failure() {
            return Self::Failure(GridErr(UnhandledFailure))
        }
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                *self.rawgrid_mut().att_mut(r, c, false) *= num;
            }
        }
        self
    }
}
