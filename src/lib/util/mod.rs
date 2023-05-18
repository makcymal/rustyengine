//! Some useful things

use {
    strum_macros::Display,
};


#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum Sign {
    Plus, Minus,
}


#[inline(always)]
pub fn pow_minus(x: usize) -> f64 {
    match x % 2 {
        0 => 1.0,
        1 => -1.0,
        _ => unreachable!(),
    }
}


#[derive(Debug, Display, Clone, PartialEq)]
pub enum Idx {
    Single(usize),
    Double((usize, usize)),
}
