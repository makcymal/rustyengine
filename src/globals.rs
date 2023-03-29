use {
    crate::{
        linalg::{
            common_matrix,
            matrixified::{Matrix, Vector},
            coord_sys::Vecspace,
            enums::MatrixType,
        },
        utils::{
            Size,
        },
    },
};

pub type Flt = f64;

pub const EPSILON: Flt = Flt::EPSILON * 10.0;

pub const DIM: usize = 3;

pub static mut BIFORM: Matrix = Matrix::empty(Size::Rect((DIM, DIM)));

pub static mut VECSPACE: Vecspace = Vecspace::empty();

pub static mut GRAMM: Matrix = Matrix::empty(Size::Rect((DIM, DIM)));


pub fn init_linalg() {
    unsafe {
        BIFORM = common_matrix(MatrixType::Identity);
        VECSPACE = Vecspace::identity();
        GRAMM = Matrix::gramm(&(VECSPACE.basis));
    }
}