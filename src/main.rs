#![allow(warnings)]

mod linalg;
mod globals;
mod utils;
mod enums;

use globals::{
    init_linalg,
};

fn main() {
    init_linalg();
}
