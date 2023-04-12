use {
    once_cell::sync::OnceCell,
    crate::{
        linalg::{
            coord_sys::CoordSys,
            matrixify::Matrix,
            init_biform, init_coordsys,
        },
    }
};

/// Used in equality comparasions with f64;
pub const EPSILON: f64 = f64::EPSILON * 10.0;

// <<< linalg

/// Actual dimension of vector space.
pub const DIM: usize = 3;
/// Actual bilinear form. Must be iniitialized in main() function.
pub static BIFORM: OnceCell<Matrix> = OnceCell::new();
/// Actual coordinate system. Must be initialized in main() function.
pub static COORDSYS: OnceCell<CoordSys> = OnceCell::new();

/// Easy way to initialize global bilinear form and coordinate system with default values.
pub fn init_linalg() {
    init_biform();
    init_coordsys();
}

// linalg >>>
