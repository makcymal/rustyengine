use {
    super::matrixified::*,
    crate::{
        utils::Size,
        globals::EPSILON,
    },
};

fn sq_matrices() -> [Matrix; 10] {
    [
        // det = 1
        Matrix::identity(Size::Rect((3, 3))).unwrap(),

        // det = 8
        Matrix::from(vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0]]),
        // det = 0.125
        Matrix::from(vec![
            vec![0.5, 0.0, 0.0],
            vec![0.0, 0.5, 0.0],
            vec![0.0, 0.0, 0.5]]),

        // det = 125
        Matrix::from(vec![
            vec![0.0, 0.0, -5.0],
            vec![0.0, -5.0, 0.0],
            vec![-5.0, 7.0, 1.0]]),
        // det = 1 / 125
        Matrix::from(vec![
            vec![-0.04, -7.0 / 25.0, -0.2],
            vec![0.0, -0.2, 0.0],
            vec![-0.2, 0.0, 0.0]]),

        // det = 57
        Matrix::from(vec![
            vec![2.0, 0.0, -5.0],
            vec![0.0, -3.0, 0.0],
            vec![-5.0, 7.0, 3.0]]),
        // det = 1 / 57
        Matrix::from(vec![
            vec![-9.0 / 57.0, -35.0 / 57.0, -15.0 / 57.0],
            vec![0.0, -19.0 / 57.0, 0.0],
            vec![-15.0 / 57.0, -14.0 / 57.0, -6.0 / 57.0]]),

        // det = 473
        Matrix::from(vec![
            vec![-3.0, -13.0, -7.0],
            vec![0.0, -11.0, 0.0],
            vec![-7.0, -5.0, -2.0]]),
        // det = 1 / 473
        Matrix::from(vec![
            vec![2.0 / 43.0, 9.0 / 473.0, -7.0 / 43.0],
            vec![0.0, -1.0 / 11.0, 0.0],
            vec![-7.0 / 43.0, 76.0 / 473.0, 3.0 / 43.0]]),

        // det = 0
        Matrix::from(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]]),
    ]
}

fn vt_matrices() -> [Matrix; 3] {
    [
        Matrix::from(vec![
            vec![2.0, 0.0],
            vec![0.0, 2.0],
            vec![7.0, 0.0]]),
        Matrix::from(vec![
            vec![4.0, 1.0],
            vec![3.0, -6.0],
            vec![11.0, 3.0]]),
        Matrix::from(vec![
            vec![3.0],
            vec![0.0],
            vec![-5.0]]),
    ]
}

fn hr_matrices() -> [Matrix; 3] {
    [
        Matrix::from(vec![
            vec![2.0, 3.0, 4.0],
            vec![1.0,-8.0, 3.0]]),
        Matrix::from(vec![
            vec![-3.0, -2.0, -1.0]]),
        Matrix::from(vec![
            vec![0.0, -2.0, -6.0]]),
    ]
}

fn vectors() -> [Vector; 4] {
    [
        Vector::fill_with(Size::Row(3), 4.0),
        Vector::fill_with(Size::Col(3), -1.0),
        Vector::from(vec![-1.0, -3.0, -7.0]),
        Vector::from(vec![-7.0, -6.0, -5.0]),
    ]
}

#[test]
fn test_add() {
    let mut sqm = sq_matrices();
    assert_eq!((&sqm[0] + &sqm[0]).unwrap(), sqm[1]);
    assert_eq!((&sqm[1] + &sqm[3]).unwrap(), sqm[5]);
    assert_eq!((&sqm[5] - &sqm[3]).unwrap(), sqm[1]);

    let mut vtm = vt_matrices();
    let mut hrm = hr_matrices();

    hrm[0].transpose();
    assert_eq!((&vtm[0] + &hrm[0]).unwrap(), vtm[1]);
    vtm[2].transpose();
    assert_eq!((&vtm[2] + &hrm[1]).unwrap(), hrm[2]);

    let mut v = vectors();

    hrm[2].transpose();
    v[2].transpose();
    assert_eq!((&v[1] + &hrm[2]).unwrap(), v[2]);
    assert_eq!((&hrm[1] - &v[0]).unwrap(), v[3]);
}

#[test]
fn test_determinant() {
    let mut sqm = sq_matrices();
    for m in sqm.iter_mut() {
        m.determine().unwrap();
    }

    assert_eq!(sqm[0].determinant.unwrap(), 1.0);

    assert_eq!(sqm[1].determinant.unwrap(), 8.0);
    assert_eq!(sqm[2].determinant.unwrap(), 0.125);

    assert_eq!(sqm[3].determinant.unwrap(), 125.0);
    assert!((sqm[4].determinant.unwrap() - 0.008).abs() < EPSILON);

    assert_eq!(sqm[5].determinant.unwrap(), 57.0);
    assert!((sqm[6].determinant.unwrap() - 1.0 / 57.0).abs() < EPSILON);

    assert_eq!(sqm[7].determinant.unwrap(), 473.0);
    assert!((sqm[8].determinant.unwrap() - 1.0 / 473.0).abs() < EPSILON);

    assert_eq!(sqm[9].determinant.unwrap(), 0.0);
}

#[test]
fn test_determinant_of_transposed() {
    let mut sqm = sq_matrices();
    for m in sqm.iter_mut() {
        m.transpose();
        m.determine().unwrap();
    }

    assert_eq!(sqm[0].determinant.unwrap(), 1.0);

    assert_eq!(sqm[1].determinant.unwrap(), 8.0);
    assert_eq!(sqm[2].determinant.unwrap(), 0.125);

    assert_eq!(sqm[3].determinant.unwrap(), 125.0);
    assert!((sqm[4].determinant.unwrap() - 0.008).abs() < EPSILON);

    assert_eq!(sqm[5].determinant.unwrap(), 57.0);
    assert!((sqm[6].determinant.unwrap() - 1.0 / 57.0).abs() < EPSILON);

    assert_eq!(sqm[7].determinant.unwrap(), 473.0);
    assert!((sqm[8].determinant.unwrap() - 1.0 / 473.0).abs() < EPSILON);

    assert_eq!(sqm[9].determinant.unwrap(), 0.0);
}

#[test]
fn test_inverse() {
    let mut sqm = sq_matrices();
    for m in sqm.iter_mut() {
        m.determine().unwrap();
    }
    assert_eq!(sqm[1].inverse().unwrap(), sqm[2]);
    assert_eq!(sqm[3].inverse().unwrap(), sqm[4]);
    assert_eq!(sqm[5].inverse().unwrap(), sqm[6]);
    assert_eq!(sqm[7].inverse().unwrap(), sqm[8]);
}
