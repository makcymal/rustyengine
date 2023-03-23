#![allow(warnings)]

#[cfg(test)]
mod tests;
mod linalg;
mod globals;

use linalg::matrix::Matrix;

fn main() {
    let mut m = Matrix::<i8>::ones((3, 3).into()).unwrap();
    dbg!(m.norm());
}
