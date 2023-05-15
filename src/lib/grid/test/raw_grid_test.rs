use {
    super::super::{
        raw_grid::{
            VecWrapper::{self, *},
            RawGrid,
        },
    },
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
        },
    },
};


// <<< RawGrid tests
#[test]
fn is_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Single(vec![]);
    assert_eq!(vw.is_single(), true);
}

#[test]
fn is_not_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Double(vec![]);
    assert_eq!(vw.is_single(), false);
}


#[test]
fn empty_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Single(vec![]);
    assert_eq!(vw.is_valid(), Err(GridErr(Emptiness)));
}

#[test]
fn curve_rec_vec_wrapper() {
    let vw = Double(vec![vec![1, 2], vec![1]]);
    assert_eq!(vw.is_valid(), Err(GridErr(CurveSides(1))));
}

#[test]
fn valid_rec_vec_wrapper() {
    let vw = Double(vec![vec![1, 2], vec![1, 2]]);
    assert!(vw.is_valid().is_ok());
}

#[test]
fn rows_rec_vec_wrapper() {
    let vw = Double(vec![vec![1, 2, 3], vec![1, 2, 3]]);
    assert_eq!(vw.rows(), 2);
}

#[test]
fn cols_rec_vec_wrapper() {
    let vw = Double(vec![vec![1, 2, 3], vec![1, 2, 3]]);
    assert_eq!(vw.cols(), 3);
}

#[test]
fn att_rec_vec_wrapper() {
    let vw = Double(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    assert_eq!(*vw.att(1, 2), 6);
}

#[test]
fn curve_from_rec_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]);
    assert_eq!(rg, Err(GridErr(CurveSides(2))));
}

#[test]
fn rows_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(rg.rows(false), 2);
}

#[test]
fn trans_cols_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.cols(false), 2);
}

#[test]
fn raw_grid_is_not_transposed() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(rg.is_transposed(), false);
}

#[test]
fn raw_grid_is_transposed() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.is_transposed(), true);
}

#[test]
fn t_trans_cols_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.cols(true), 3);
}

#[test]
fn att_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(*rg.att(0, 1, false), 2);
}

#[test]
fn att_trans_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(*rg.att(0, 1, false), 4);
}

#[test]
fn att_t_trans_raw_grid() {
    let rg = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(*rg.att(0, 1, true), 2);
}

#[test]
fn equal_raw_grids() {
    let lhs = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    let rhs = RawGrid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn equal_rec_lin_raw_grids() {
    let lhs = RawGrid::from_double(vec![vec![1, 2, 3]]).unwrap();
    let rhs = RawGrid::from_single(vec![1, 2, 3]).unwrap();
    assert_eq!(lhs, rhs);
}

#[test]
fn equal_rec_lin_transposed_raw_grids() {
    let lhs = RawGrid::from_double(vec![vec![1], vec![2], vec![3]]).unwrap();
    let rhs = RawGrid::from_single(vec![1, 2, 3]).unwrap().transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn inequal_raw_grids() {
    let lhs = RawGrid::from_double(vec![vec![1], vec![2], vec![3]]).unwrap();
    let rhs = RawGrid::from_single(vec![1, 2, 3]).unwrap();
    assert_ne!(lhs, rhs);
}
