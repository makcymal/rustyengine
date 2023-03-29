#![allow(warnings)]

mod vecspace;
mod globals;
mod utils;

use {
    globals::{CNT, BIFORM},
    vecspace::{
        matrixified::{Matrix, Vector},
        biform,
        enums::MatrixType,
    },
};

fn main() {
    biform::set_common_biform(MatrixType::Identity);
}
