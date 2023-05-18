use {
    super::super::{
        matrix::Matrix,
        set_biform_identity,
        set_biform
    },
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
fn vector_combination() {
    let cols = Matrix::from_double(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    ).to_multicol();
    let coef = vec![1.0, 2.0, 3.0];
    let comb = Matrix::from_single(vec![-50.0, -22.0, -23.0]).raw_transpose().to_col();
    assert_eq!(cols.combine(coef).unwrap(), comb);
}

#[test]
fn vector_multicol_dim() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 3.0],
        vec![3.0, 4.0, 5.0],
    ]).to_multicol();
    assert_eq!(lhs.dim().unwrap(), 2);
}

#[test]
fn vector_multirow_dim() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 3.0],
        vec![3.0, 4.0, 5.0],
    ]).to_multirow();
    assert_eq!(lhs.dim().unwrap(), 3);
}

#[test]
fn scalar_prod() {
    set_biform(vec![
        vec![2.0, 3.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0],
    ]);
    let lhs = Matrix::from_single(vec![1.0, 2.0, 3.0]).to_row();
    let rhs = Matrix::from_double(vec![
        vec![3.0, 4.0, 5.0],
    ]).to_row();
    assert_eq!(lhs.scalar_prod(&rhs).unwrap(), 64.0);
}

#[test]
fn scalar_prod_at() {
    set_biform(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0],
    ]);
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 3.0],
        vec![3.0, 4.0, 5.0],
    ]).to_multirow();
    assert_eq!(lhs.scalar_prod_at(0, &lhs, 1).unwrap(), 52.0);
}
