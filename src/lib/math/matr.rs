use {
    crate::{
        grid::{RawGrid, Grid},
        errs::{
            AnyRes::{self, *},
            AnyErr::{self, *},
            GridErr::{self, *},
            MatrErr::{self, *},
        }
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
        let lhs_rows = self.grid().rows(false);
        let lhs_cols = self.grid().cols(false);

        if lhs_rows != rhs.grid().rows(t) || lhs_cols != rhs.grid().cols(t) {
            return Self::Failure(MatrErr(AddSizesMismatch(
                (lhs_rows, lhs_cols),(rhs.grid().rows(t), rhs.grid().cols(t)))));
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.grid_mut().att_mut(r, c, false).unwrap() += rhs.grid().att(r, c, t).unwrap();
            }
        }
        self
    }

    pub fn sub(&self, rhs: &Self, t: bool) -> Self {
        let mut output = self.clone();
        output.sub_assign(rhs, t)
    }

    pub fn sub_assign(mut self, rhs: &Self, t: bool) -> Self {
        let lhs_rows = self.grid().rows(false);
        let lhs_cols = self.grid().cols(false);

        if lhs_rows != rhs.grid().rows(t) || lhs_cols != rhs.grid().cols(t) {
            return Self::Failure(MatrErr(AddSizesMismatch(
                (lhs_rows, lhs_cols),(rhs.grid().rows(t), rhs.grid().cols(t)))));
        }
        for r in 0..lhs_rows {
            for c in 0..lhs_cols {
                *self.grid_mut().att_mut(r, c, false).unwrap() -= rhs.grid().att(r, c, t).unwrap();
            }
        }
        self
    }

    pub fn mul(&self, rhs: &Self, t: bool) -> Self {
        let out_rows = self.grid().rows(false);
        let out_cols = rhs.grid().cols(t);

        if self.grid().cols(false) != rhs.grid().rows(t) {
            return Self::Failure(MatrErr(MulSizesMismatch(
                (out_rows, self.grid().cols(false)),(rhs.grid().rows(t), out_cols))));
        }

        let mut output = Self::zero(out_rows, out_cols);
        for r in 0..out_rows {
            for c in 0..out_cols {
                *output.att_mut(r, c).unwrap() =
                    (0..self.grid().cols(false))
                        .map(|i| self.att(r, i).unwrap() * rhs.att(i, c).unwrap())
                        .sum();
            }
        }
        output
    }
}