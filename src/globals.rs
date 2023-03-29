use {
    crate::{
        utils::Size,
        vecspace::{
            matrixified::Matrix,
        },
    },
};

pub type Flt = f64;

pub const EPSILON: Flt = Flt::EPSILON * 10.0;

pub static mut BIFORM: Matrix = Matrix::empty(Size::Rect((3, 3)));
