#![allow(warnings)]

/// Types related to linear algebra, such as Matrix, Vector, Point, VectorSpace, CoordinateSystem.
mod linalg;
/// Global configuration parameters and state variables.
mod globals;
/// Types that may be useful in any module.
mod utils;
/// Enums defining errors and some useful things.
mod enums;

use globals::{
    init_linalg,
};

fn main() {
    init_linalg();
}
