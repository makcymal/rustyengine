use {
    once_cell::sync::OnceCell,
    crate::{
        linal::{
            coord_sys::CoordSys,
            matrixify::Matrix,
            init_biform, init_coordsys,
        },
    }
};

/// Used in equality comparasions with f64;
pub const EPSILON: f64 = f64::EPSILON * 10.0;
