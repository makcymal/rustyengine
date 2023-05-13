use {
    crate::{
        grid::{RawGrid, Grid},
        errs::{
            AnyRes,
            AnyErr::{self, *},
            GridErr::{self, *},
            MatrErr::{self, *},
        },
    },
};

pub type Matr = Grid<f64>;

impl Matr {
    pub fn identity(side: usize) -> Self {
        let mut id = Self::zero(side, side);
        let _ = (0..side).map(|d| *id.att_mut(d, d).unwrap() = 1.0);
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
                *self.rawgrid_mut().att_mut(r, c, false).unwrap() += rhs.rawgrid().att(r, c, t).unwrap();
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
                *self.rawgrid_mut().att_mut(r, c, false).unwrap() -= rhs.rawgrid().att(r, c, t).unwrap();
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
                *output.att_mut(r, c).unwrap() =
                    (0..self.rawgrid().cols(false))
                        .map(|i| self.rawgrid().att(r, i, false).unwrap() * rhs.rawgrid().att(i, c, t).unwrap())
                        .sum();
            }
        }
        output
    }
}
