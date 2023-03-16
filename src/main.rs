#![allow(warnings)]

#[cfg(test)]
mod tests;
mod primitives;

use crate::{
    primitives::*
};

pub const EPSILON: f64 = f64::EPSILON * 10.0;
pub const SCREEN_WIDTH: u16 = 500;
pub const SCREEN_HEIGHT: u16 = 300;

pub static VECTOR_SPACE: VectorSpace = VectorSpace::default();

fn main() {

}
