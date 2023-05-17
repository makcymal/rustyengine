use {
    super::super::{
        matrix::Matrix,
        set_biform_identity,
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
fn identity_matr() {
    let m = Matrix::from_double(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0]
    ]).to_square();
    assert_eq!(Matrix::identity(3), m);
}

#[test]
fn zero_matr() {
    let m = Matrix::from_double(vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0]
    ]);
    assert_eq!(Matrix::zero(3, 3), m);
}

#[test]
fn diag_determinant() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]
    ]);
    assert_eq!(m.det().unwrap(), 8.0);
}

#[test]
fn inv_diag_determinant() {
    let m = Matrix::from_double(vec![
        vec![0.0, 0.0, -5.0],
        vec![0.0, -5.0, 0.0],
        vec![-5.0, 7.0, 1.0]
    ]);
    assert_eq!(m.det().unwrap(), 125.0);
}

#[test]
fn filled_determinant() {
    let m = Matrix::from_double(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    assert_eq!(m.det().unwrap(), 473.0);
}

#[test]
fn determinant_of_collist() {
    let m = Matrix::from_double(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    ).to_multicol();
    assert_eq!(m.det().unwrap(), 473.0);
}

#[test]
fn transposed_determinant() {
    let mut m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]]
    );
    let before = m.det().unwrap();
    let after = m.transpose().det().unwrap();
    assert_eq!(before, after);
}

#[test]
fn determinant_of_failure() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0]]
    );
    assert_eq!(m.det(), Err(GridErr(UnhandledFailure)));
}

#[test]
fn determinant_of_non_square() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]);
    assert_eq!(m.det(), Err(GridErr(NotSquare((2, 3)))));
}

#[test]
fn determinant_of_non_square_trans() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]).transpose();
    assert_eq!(m.det(), Err(GridErr(NotSquare((3, 2)))));
}

#[test]
fn diag_inversed() {
    let primal = Matrix::from_double(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]]
    );
    let inversed = Matrix::from_double(vec![
        vec![0.5, 0.0, 0.0],
        vec![0.0, 0.5, 0.0],
        vec![0.0, 0.0, 0.5]]
    );
    assert_eq!(primal.inv().unwrap(), inversed);
}

#[test]
fn filled_inversed() {
    let mut primal = Matrix::from_double(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    let inversed = Matrix::from_double(vec![
        vec![2.0 / 43.0, 9.0 / 473.0, -7.0 / 43.0],
        vec![0.0, -1.0 / 11.0, 0.0],
        vec![-7.0 / 43.0, 76.0 / 473.0, 3.0 / 43.0]]
    );
    assert_eq!(primal.inv().unwrap(), inversed.round());
}

#[test]
fn transposed_inversed() {
    let primal = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]
    ]).transpose();
    let inversed = Matrix::from_double(vec![
        vec![-9.0 / 57.0, -35.0 / 57.0, -15.0 / 57.0],
        vec![0.0, -19.0 / 57.0, 0.0],
        vec![-15.0 / 57.0, -14.0 / 57.0, -6.0 / 57.0]
    ]).transpose();
    assert_eq!(primal.inv().unwrap(), inversed.round());
}

#[test]
fn inversed_of_failure() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0]]
    );
    assert_eq!(m.inv(), Err(GridErr(UnhandledFailure)));
}

#[test]
fn inversed_of_non_square() {
    let m = Matrix::from_double(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]);
    assert_eq!(m.det(), Err(GridErr(NotSquare((2, 3)))));
}

// #[test]
// fn primal_times_inversed() {
//     let primal = Matr::from_rec(vec![
//         vec![2.0, 0.0, -5.0],
//         vec![0.0, -3.0, 0.0],
//         vec![-5.0, 7.0, 3.0]
//     ]);
//     assert_eq!(primal.inversed().unwrap().mul(&primal, false), Matr::identity(3));
// }

#[test]
fn round_matr() {
    let m = Matrix::from_double(vec![
        vec![1.5625, 0.875],
        vec![-0.875, -0.75]
    ]);
    let m_round = Matrix::from_double(vec![
        vec![1.5, 0.5],
        vec![-0.5, -0.5]
    ]);
    assert_eq!(m.round_prec(1), m_round);
}

#[test]
fn add_assign_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]);
    let sum = Matrix::from_double(vec![
        vec![6.0, 8.0],
        vec![11.0, 11.0],
    ]);
    assert_eq!(lhs.add_assign(&rhs), sum);
}

#[test]
fn add_t_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]);
    let sum = Matrix::from_double(vec![
        vec![6.0, 10.0],
        vec![9.0, 11.0],
    ]);
    assert_eq!(lhs.add_t(&rhs), sum);
}

#[test]
fn add_assign_trans_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![7.0, 8.0],
        vec![0.0, 9.0],
    ]).transpose();
    let sum = Matrix::from_double(vec![
        vec![6.0, 9.0, 10.0],
        vec![9.0, 12.0, 20.0],
    ]);
    assert_eq!(lhs.add_assign(&rhs), sum);
}

#[test]
fn add_assign_t_trans_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0, 100.0],
        vec![7.0, 8.0, 200.0],
    ]).transpose();
    let sum = Matrix::from_double(vec![
        vec![6.0, 8.0, 110.0],
        vec![10.0, 12.0, 211.0],
    ]);
    assert_eq!(lhs.add_assign_t(&rhs), sum);
}

#[test]
fn sub_assign_diff_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]).to_multicol();
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]).to_multirow();
    let sum = Matrix::from_double(vec![
        vec![6.0, 8.0],
        vec![11.0, 11.0],
    ]).to_multicol();
    assert_eq!(sum.sub_assign(&rhs), lhs);
}

#[test]
fn sub_assign_t_trans_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0, 100.0],
        vec![7.0, 8.0, 200.0],
    ]).transpose();
    let sum = Matrix::from_double(vec![
        vec![6.0, 8.0, 110.0],
        vec![10.0, 12.0, 211.0],
    ]);
    assert_eq!(sum.sub_assign_t(&rhs), lhs);
}

#[test]
fn invalid_add_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
        vec![8.0, 7.0],
    ]);
    assert_eq!(lhs.add(&rhs), Matrix::Failure(MathErr(AddSizesMismatch { lhs: (2, 2), rhs: (3, 2) })));
}

#[test]
fn invalid_add_t_matr() {
    let lhs = Matrix::from_double(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
        vec![8.0, 7.0],
    ]).transpose();
    assert_eq!(lhs.add_t(&rhs), Matrix::Failure(MathErr(AddSizesMismatch { lhs: (2, 2), rhs: (3, 2) })));
}

#[test]
fn mul_square_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let prod = Matrix::from_double(vec![
        vec![2.0, 8.0, 0.0],
        vec![-1.0, 2.0, 5.0],
        vec![-8.0, 31.0, 35.0]
    ]);
    assert_eq!(lhs.mul(&rhs), prod);
}

#[test]
fn mul_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
    ]);
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0, -3.0],
    ]).transpose();
    let prod = Matrix::from_single(vec![5.0, -4.0]).transpose();
    assert_eq!(lhs.mul(&rhs), prod);
}

#[test]
fn mul_t_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 3.0],
    ]);
    let prod = Matrix::from_double(vec![
        vec![5.0, -4.0],
        vec![-1.0, 5.0],
        vec![1.0, 37.0]
    ]);
    assert_eq!(lhs.mul_t(&rhs), prod);
}

#[test]
fn invalid_mul_t_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0],
        vec![-2.0, 4.0],
    ]);
    assert_eq!(lhs.mul_t(&rhs), Matrix::Failure(MathErr(MulSizesMismatch { lhs: (3, 3), rhs: (2, 2) })));
}

#[test]
fn mul_left_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let prod = Matrix::from_double(vec![
        vec![2.0, 8.0, 0.0],
        vec![-1.0, 2.0, 5.0],
        vec![-8.0, 31.0, 35.0]
    ]);
    assert_eq!(rhs.mul_left(&lhs), prod);
}

#[test]
fn mul_left_t_matr() {
    let lhs = Matrix::from_double(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]).transpose();
    let rhs = Matrix::from_double(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let prod = Matrix::from_double(vec![
        vec![2.0, 8.0, 0.0],
        vec![-1.0, 2.0, 5.0],
        vec![-8.0, 31.0, 35.0]
    ]);
    assert_eq!(rhs.mul_left_t(&lhs), prod);
}

#[test]
fn matrix_norm() {
    let m = Matrix::from_double(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]]);
    assert_eq!(m.norm().unwrap(), 285.0f64.sqrt());
}

#[test]
fn col_norm() {
    let c = Matrix::from_single(vec![-1.0, -3.0, -7.0]).raw_transpose().to_col();
    assert_eq!(c.norm().unwrap(), 59.0f64.sqrt());
}

#[test]
fn row_len() {
    set_biform_identity();
    let r = Matrix::from_single(vec![-1.0, -3.0, -7.0]).to_row();
    assert_eq!(r.len().unwrap(), 59.0f64.sqrt());
}

// #[test]
// fn div_square_matr() {
//     let lhs = Matr::from_rec(vec![
//         vec![4.0, 1.0, 0.0],
//         vec![-1.0, 0.0, 1.0],
//         vec![-2.0, 3.0, 7.0]
//     ]);
//     let rhs = Matr::from_rec(vec![
//         vec![1.0, 1.0, 0.0],
//         vec![-2.0, 4.0, 0.0],
//         vec![0.0, 3.0, 5.0]
//     ]);
//     let prod = Matr::from_rec(vec![
//         vec![2.0, 8.0, 0.0],
//         vec![-1.0, 2.0, 5.0],
//         vec![-8.0, 31.0, 35.0]
//     ]);
//     assert_eq!(prod.div(&rhs), lhs);
// }

#[test]
fn neg_rowlist() {
    let rl = Matrix::from_double(vec![
        vec![0.0, 1.0, 2.0],
        vec![-1.0, -2.0, -3.0]
    ]);
    let neg = Matrix::from_double(vec![
        vec![0.0, -1.0, -2.0],
        vec![1.0, 2.0, 3.0]
    ]);
    assert_eq!(rl.neg(), neg);
}

#[test]
fn neg_col() {
    let rl = Matrix::from_single(vec![0.0, 1.0, 2.0]).raw_transpose().to_col();
    let neg = Matrix::from_double(vec![
        vec![0.0],
        vec![-1.0],
        vec![-2.0],
    ]).to_col();
    assert_eq!(rl.neg(), neg);
}
