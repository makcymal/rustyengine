#![allow(warnings)]

mod linalg;
mod globals;
mod utils;

use globals::{
    init_linalg,
    GRAMM,
};

fn main() {
    init_linalg();
    unsafe {
        dbg!(&GRAMM);
    }
}
