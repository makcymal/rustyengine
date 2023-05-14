use {
    crate::{
        grid::{RawGrid, Grid},
        errs::{
            AnyRes,
            AnyErr::{self, *},
            GridErr::{self, *},
            MatrErr::{self, *},
        },
        util::pow_minus,
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

    pub fn add(&self, rhs: &Self, t: bool) -> Self {
        let mut output = self.clone();
        output.add_assign(rhs, t)
    }

    pub fn add_assign(mut self, rhs: &Self, t: bool) -> Self {
        let lhs_rows = self.rawgrid().rows(false);
        let lhs_cols = self.rawgrid().cols(false);

        if lhs_rows != rhs.rawgrid().rows(t) || lhs_cols != rhs.rawgrid().cols(t) {
            return Self::Failure(MatrErr(AddSizesMismatch {
                lhs: (lhs_rows, lhs_cols),
                rhs: (rhs.rawgrid().rows(t), rhs.rawgrid().cols(t)),
            }))
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.rawgrid_mut().att_mut(r, c, false) =
                    round(*self.rawgrid_mut().att_mut(r, c, false) + rhs.rawgrid().att(r, c, t))
            }
        }
        self
    }

    pub fn sub(&self, rhs: &Self, t: bool) -> Self {
        let mut output = self.clone();
        output.sub_assign(rhs, t)
    }

    pub fn sub_assign(mut self, rhs: &Self, t: bool) -> Self {
        let lhs_rows = self.rawgrid().rows(false);
        let lhs_cols = self.rawgrid().cols(false);

        if lhs_rows != rhs.rawgrid().rows(t) || lhs_cols != rhs.rawgrid().cols(t) {
            return Self::Failure(MatrErr(AddSizesMismatch {
                lhs: (lhs_rows, lhs_cols),
                rhs: (rhs.rawgrid().rows(t), rhs.rawgrid().cols(t)),
            }))
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.rawgrid_mut().att_mut(r, c, false) =
                    round(*self.rawgrid_mut().att_mut(r, c, false) - rhs.rawgrid().att(r, c, t))
            }
        }
        self
    }

    pub fn mul(&self, rhs: &Self, t: bool) -> Self {
        let out_rows = self.rawgrid().rows(false);
        let out_cols = rhs.rawgrid().cols(t);

        if self.rawgrid().cols(false) != rhs.rawgrid().rows(t) {
            return Self::Failure(MatrErr(MulSizesMismatch {
                lhs: (out_rows, self.rawgrid().cols(false)),
                rhs: (rhs.rawgrid().rows(t), out_cols),
            }))
        }

        let mut output = Self::zero(out_rows, out_cols);
        for r in 0..out_rows {
            for c in 0..out_cols {
                *output.att_mut(r, c) =
                    round((0..self.rawgrid().cols(false))
                        .map(|i| self.rawgrid().att(r, i, false) * rhs.rawgrid().att(i, c, t))
                        .sum())
            }
        }
        output
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
}
