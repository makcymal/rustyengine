#![allow(warnings)]

/// Types related to linear algebra;
mod linal;

use {
    linal::{
        matrixify::{
            Matrixify,
        },
        enums::{
            Repr,
        }
    }
};

fn main() {
    let m = Matrixify::identity(3, Repr::Square);
    let n = Matrixify::fill_with((3, 3), Repr::RowList, 5.0);
    m.mul_num(10.0).add_assign(&n);
}
