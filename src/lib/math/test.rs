use {
    super::{
        prec::{
            float_exponent,
            round_prec,
        },
        matr::Matr,
    }
};
use crate::errs::AnyErr::{GridErr, MatrErr};
use crate::errs::GridErr::{CurveSides, UnhandledFailure};
use crate::errs::MatrErr::{AddSizesMismatch, DeterminantOfNonSquare, MulSizesMismatch};
use crate::grid::Repr::Failure;

#[test]
fn float_exponent_of_1() {
    assert_eq!(float_exponent(1.0), 1023);
}

#[test]
fn float_exponent_of_2() {
    assert_eq!(float_exponent(2.0), 1024);
}

#[test]
fn float_exponent_of_10() {
    assert_eq!(float_exponent(10.0), 1026);
}

#[test]
fn float_exponent_of_0_5() {
    assert_eq!(float_exponent(0.5), 1022);
}

#[test]
fn float_exponent_of_0_3() {
    assert_eq!(float_exponent(0.3), 1021);
}

#[test]
fn round_0_5_with_prec_0() {
    assert_eq!(round_prec(0.5, 0), 0.5);
}

#[test]
fn round_0_75_with_prec_1() {
    assert_eq!(round_prec(0.75, 1), 0.5);
}

#[test]
fn round_0_875_with_prec_1() {
    assert_eq!(round_prec(0.875, 1), 0.5);
}

#[test]
fn round_0_875_with_prec_2() {
    assert_eq!(round_prec(-0.875, 2), -0.75);
}

#[test]
fn round_1_5625_with_prec_3() {
    assert_eq!(round_prec(1.5625, 3), 1.5);
}

#[test]
fn round_0_5625_with_prec_3() {
    assert_eq!(round_prec(0.5625, 3), 0.5);
}


#[test]
fn identity_matr() {
    let m = Matr::from_rec(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0]
    ]);
    assert_eq!(Matr::identity(3), m);
}

#[test]
fn zero_matr() {
    let m = Matr::from_rec(vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0]
    ]);
    assert_eq!(Matr::zero(3, 3), m);
}

#[test]
fn diag_determinant() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]
    ]);
    assert_eq!(m.determinant().unwrap(), 8.0);
}

#[test]
fn inv_diag_determinant() {
    let m = Matr::from_rec(vec![
        vec![0.0, 0.0, -5.0],
        vec![0.0, -5.0, 0.0],
        vec![-5.0, 7.0, 1.0]
    ]);
    assert_eq!(m.determinant().unwrap(), 125.0);
}

#[test]
fn filled_determinant() {
    let m = Matr::from_rec(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    assert_eq!(m.determinant().unwrap(), 473.0);
}

#[test]
fn determinant_of_collist() {
    let m = Matr::from_rec(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    ).to_collist();
    assert_eq!(m.determinant().unwrap(), 473.0);
}

#[test]
fn transposed_determinant() {
    let mut m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]]
    );
    let before = m.determinant().unwrap();
    let after = m.transpose().determinant().unwrap();
    assert_eq!(before, after);
}

#[test]
fn determinant_of_failure() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0]]
    );
    assert_eq!(m.determinant(), Err(GridErr(UnhandledFailure)));
}

#[test]
fn determinant_of_non_square() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]);
    assert_eq!(m.determinant(), Err(MatrErr(DeterminantOfNonSquare((2, 3)))));
}

#[test]
fn determinant_of_non_square_trans() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]).transpose();
    assert_eq!(m.determinant(), Err(MatrErr(DeterminantOfNonSquare((3, 2)))));
}

#[test]
fn diag_inversed() {
    let primal = Matr::from_rec(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]]
    );
    let inversed = Matr::from_rec(vec![
        vec![0.5, 0.0, 0.0],
        vec![0.0, 0.5, 0.0],
        vec![0.0, 0.0, 0.5]]
    );
    assert_eq!(primal.inversed().unwrap(), inversed);
}

#[test]
fn filled_inversed() {
    let mut primal = Matr::from_rec(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    let inversed = Matr::from_rec(vec![
        vec![2.0 / 43.0, 9.0 / 473.0, -7.0 / 43.0],
        vec![0.0, -1.0 / 11.0, 0.0],
        vec![-7.0 / 43.0, 76.0 / 473.0, 3.0 / 43.0]]
    );
    assert_eq!(primal.inversed().unwrap(), inversed.round());
}

#[test]
fn transposed_inversed() {
    let primal = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]
    ]).transpose();
    let inversed = Matr::from_rec(vec![
        vec![-9.0 / 57.0, -35.0 / 57.0, -15.0 / 57.0],
        vec![0.0, -19.0 / 57.0, 0.0],
        vec![-15.0 / 57.0, -14.0 / 57.0, -6.0 / 57.0]
    ]).transpose();
    assert_eq!(primal.inversed().unwrap(), inversed.round());
}

#[test]
fn inversed_of_failure() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0]]
    );
    assert_eq!(m.inversed(), Err(GridErr(UnhandledFailure)));
}

#[test]
fn inversed_of_non_square() {
    let m = Matr::from_rec(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0]
    ]);
    assert_eq!(m.determinant(), Err(MatrErr(DeterminantOfNonSquare((2, 3)))));
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
    let m = Matr::from_rec(vec![
        vec![1.5625, 0.875],
        vec![-0.875, -0.75]
    ]);
    let m_round = Matr::from_rec(vec![
        vec![1.5, 0.5],
        vec![-0.5, -0.5]
    ]);
    assert_eq!(m.round_prec(1), m_round);
}

#[test]
fn add_assign_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]);
    let sum = Matr::from_rec(vec![
        vec![6.0, 8.0],
        vec![11.0, 11.0],
    ]);
    assert_eq!(lhs.add_assign(&rhs), sum);
}

#[test]
fn add_t_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]);
    let sum = Matr::from_rec(vec![
        vec![6.0, 10.0],
        vec![9.0, 11.0],
    ]);
    assert_eq!(lhs.add_t(&rhs), sum);
}

#[test]
fn add_assign_trans_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![7.0, 8.0],
        vec![0.0, 9.0],
    ]).transpose();
    let sum = Matr::from_rec(vec![
        vec![6.0, 9.0, 10.0],
        vec![9.0, 12.0, 20.0],
    ]);
    assert_eq!(lhs.add_assign(&rhs), sum);
}

#[test]
fn add_assign_t_trans_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0, 100.0],
        vec![7.0, 8.0, 200.0],
    ]).transpose();
    let sum = Matr::from_rec(vec![
        vec![6.0, 8.0, 110.0],
        vec![10.0, 12.0, 211.0],
    ]);
    assert_eq!(lhs.add_assign_t(&rhs), sum);
}

#[test]
fn sub_assign_diff_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]).to_collist();
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
    ]).to_rowlist();
    let sum = Matr::from_rec(vec![
        vec![6.0, 8.0],
        vec![11.0, 11.0],
    ]).to_collist();
    assert_eq!(sum.sub_assign(&rhs), lhs);
}

#[test]
fn sub_assign_t_trans_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0, 10.0],
        vec![3.0, 4.0, 11.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0, 100.0],
        vec![7.0, 8.0, 200.0],
    ]).transpose();
    let sum = Matr::from_rec(vec![
        vec![6.0, 8.0, 110.0],
        vec![10.0, 12.0, 211.0],
    ]);
    assert_eq!(sum.sub_assign_t(&rhs), lhs);
}

#[test]
fn invalid_add_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
        vec![8.0, 7.0],
    ]);
    assert_eq!(lhs.add(&rhs), Matr::Failure(MatrErr(AddSizesMismatch { lhs: (2, 2), rhs: (3, 2) })));
}

#[test]
fn invalid_add_t_matr() {
    let lhs = Matr::from_rec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![5.0, 6.0],
        vec![8.0, 7.0],
        vec![8.0, 7.0],
    ]).transpose();
    assert_eq!(lhs.add_t(&rhs), Matr::Failure(MatrErr(AddSizesMismatch { lhs: (2, 2), rhs: (3, 2) })));
}

#[test]
fn mul_square_matr() {
    let lhs = Matr::from_rec(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matr::from_rec(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let prod = Matr::from_rec(vec![
        vec![2.0, 8.0, 0.0],
        vec![-1.0, 2.0, 5.0],
        vec![-8.0, 31.0, 35.0]
    ]);
    assert_eq!(lhs.mul(&rhs), prod);
}

#[test]
fn mul_matr() {
    let lhs = Matr::from_rec(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
    ]);
    let rhs = Matr::from_rec(vec![
        vec![1.0, 1.0, -3.0],
    ]).transpose();
    let prod = Matr::from_lin(vec![5.0, -4.0]).transpose();
    assert_eq!(lhs.mul(&rhs), prod);
}

#[test]
fn mul_t_matr() {
    let lhs = Matr::from_rec(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matr::from_rec(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 3.0],
    ]);
    let prod = Matr::from_rec(vec![
        vec![5.0, -4.0],
        vec![-1.0, 5.0],
        vec![1.0, 37.0]
    ]);
    assert_eq!(lhs.mul_t(&rhs), prod);
}

#[test]
fn invalid_mul_t_matr() {
    let lhs = Matr::from_rec(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matr::from_rec(vec![
        vec![1.0, 1.0],
        vec![-2.0, 4.0],
    ]);
    let prod = Matr::from_rec(vec![
        vec![5.0, -4.0],
        vec![-1.0, 5.0],
        vec![1.0, 37.0]
    ]);
    assert_eq!(lhs.mul_t(&rhs), Matr::Failure(MatrErr(MulSizesMismatch { lhs: (3, 3), rhs: (2, 2) })));
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
    let rl = Matr::from_rec(vec![
        vec![0.0, 1.0, 2.0],
        vec![-1.0, -2.0, -3.0]
    ]);
    let neg = Matr::from_rec(vec![
        vec![0.0, -1.0, -2.0],
        vec![1.0, 2.0, 3.0]
    ]);
    assert_eq!(rl.neg(), neg);
}

#[test]
fn neg_col() {
    let rl = Matr::from_lin(vec![0.0, 1.0, 2.0]).raw_transpose().to_col();
    let neg = Matr::from_rec(vec![
        vec![0.0],
        vec![-1.0],
        vec![-2.0],
    ]).to_col();
    assert_eq!(rl.neg(), neg);
}
