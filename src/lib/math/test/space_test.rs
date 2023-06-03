use {
    super::super::*,
    crate::{
        errs::{
            ReErr::*,
            GridErr::*,
            MathErr::*,
        },
        grid::{
            Repr::*,
        }
    }
};


#[test]
fn normalize_vector_in_basis() {
    set_biform(Matrix::identity(3).num_mul_assign(2.0));
    let cs = CoordSys::default();
    dbg!("vector.dim()");
    let mut vector = Matrix::from_single(vec![2.0, -1.0, 0.0]).raw_transpose().to_col();
    cs.normalize(&mut vector);
    let len = 10.0_f64.sqrt();
    let norm = Matrix::from_single(vec![2.0, -1.0, 0.0]).raw_transpose().to_col().num_div_assign(len);
    assert_eq!(vector, norm);
}
