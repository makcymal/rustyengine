use {
    crate::{
        globals::EPSILON,
        utils::Size,
    },
    super::matrixify::*,
};

#[test]
fn test_add() {
    let mut lhs = Matrix::from(vec![
        vec![2.0, 0.0],
        vec![0.0, 2.0],
        vec![7.0, 0.0]
    ]);
    let mut rhs = Matrix::from(vec![
        vec![4.0, 1.0],
        vec![3.0, -6.0],
        vec![11.0, 3.0]
    ]);
    let mut out = Matrix::from(vec![
        vec![6.0, 1.0],
        vec![3.0, -4.0],
        vec![18.0, 3.0]
    ]);
    assert_eq!(&lhs + &rhs, out);

    lhs.transpose();
    rhs.transpose();
    out.transpose();
    assert_eq!(&lhs + &rhs, out);

    rhs = &rhs + &lhs;
    out = &out + &lhs;
    assert_eq!(&lhs + &rhs, out);

    assert_eq!(&out - &lhs, rhs);

    let mut lhs = Matrix::from(vec![
        vec![2.0],
        vec![0.0],
        vec![7.0]
    ]);
    let mut rhs = Vector::from(vec![-4.0, -5.0, 6.0]);
    let m_out = Matrix::from(vec![
        vec![-2.0],
        vec![-5.0],
        vec![13.0]
    ]);
    let v_out = Vector::from(vec![-2.0, -5.0, 13.0]);

    rhs.transpose();
    assert_eq!(&lhs + &rhs, m_out);

    lhs.transpose();
    rhs.transpose();
    assert_eq!(&rhs + &lhs, v_out);
}

#[test]
fn test_mul() {
    let lhs = Matrix::from(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let rhs = Matrix::from(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let out = Matrix::from(vec![
        vec![2.0, 8.0, 0.0],
        vec![-1.0, 2.0, 5.0],
        vec![-8.0, 31.0, 35.0]
    ]);
    assert_eq!(&lhs * &rhs, out);

    let lhs = Vector::from(vec![3.0, 5.0, 6.0]);
    let out = Matrix::from(vec![
        vec![-7.0, 41.0, 30.0]
    ]);
    assert_eq!(&lhs * &rhs, out);

    let lhs = Matrix::from(vec![
        vec![4.0, 1.0, 0.0],
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 3.0, 7.0]
    ]);
    let mut rhs = Vector::from(vec![3.0, 5.0, 6.0]);
    rhs.transpose();
    let out = Matrix::from(vec![
        vec![17.0],
        vec![3.0],
        vec![51.0]
    ]);
    assert_eq!(&lhs * &rhs, out);
}


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
    assert_eq!(sqm[2].inverse().unwrap(), sqm[1]);
    assert_eq!(sqm[3].inverse().unwrap(), sqm[4]);
    assert_eq!(sqm[5].inverse().unwrap(), sqm[6]);
    assert_eq!(sqm[7].inverse().unwrap(), sqm[8]);
}

#[test]
fn test_inverse_of_transposed() {
    let mut sqm = sq_matrices();
    for m in sqm.iter_mut() {
        m.transpose();
        m.determine().unwrap();
    }
    assert_eq!(sqm[1].inverse().unwrap(), sqm[2]);
    assert_eq!(sqm[2].inverse().unwrap(), sqm[1]);
    assert_eq!(sqm[3].inverse().unwrap(), sqm[4]);
    assert_eq!(sqm[5].inverse().unwrap(), sqm[6]);
    assert_eq!(sqm[7].inverse().unwrap(), sqm[8]);
}

#[test]
fn test_norm() {
    let m = Matrix::from(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]]);
    assert_eq!(m.norm(), (285.0 as f64).sqrt());

    let v = Vector::from(vec![-1.0, -3.0, -7.0]);
    assert_eq!(v.norm(), (59.0 as f64).sqrt());
}

#[test]
fn test_transpose() {
    let mut m = Matrix::from(vec![
        vec![4.0, 1.0],
        vec![3.0, -6.0],
        vec![11.0, 3.0]]);

    let elem = m[(2, 0)];
    m.transpose();
    assert_eq!(elem, m[(0, 2)]);

    let mut v = Vector::from(vec![-1.0, -3.0, -7.0]);

    let elem = v[2];
    v.transpose();
    assert_eq!(elem, v[2]);
}

#[test]
fn test_to_vector() {
    let mut m = Matrix::from(vec![
        vec![3.0],
        vec![0.0],
        vec![-5.0]]);
    let v = Vector::from(vec![3.0, 0.0, -5.0]);
    m.transpose();

    assert_eq!(m.to_vector().unwrap(), v);

    let m = Matrix::from(vec![
        vec![3.0],
        vec![0.0],
        vec![-5.0]]);
    let mut v = Vector::from(vec![3.0, 0.0, -5.0]);
    v.transpose();

    assert_eq!(m.to_vector().unwrap(), v);
}

#[test]
fn test_scalar_prod() {
    let lhs = Vector::from(vec![1.0, 3.0, 4.0]);
    let rhs = Vector::from(vec![5.0, -4.0, -7.0]);
    let matrix = Matrix::from(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]);

    assert_eq!(scalar_prod(&lhs, &matrix, &rhs), 214.0);
}
