use {
    crate::vecspace::{
        matrixified::Matrix,

    },
};

pub type Flt = f64;

pub const EPSILON: Flt = Flt::EPSILON * 10.0;

pub static mut BIFORM: Option<Matrix> = None;
