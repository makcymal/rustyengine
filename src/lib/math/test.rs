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
use crate::errs::MatrErr::DeterminantOfNonSquare;

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
