use {
    crate::{
        globals::EPSILON,
        utils::Size,
    },
    super::matrixify::*,
    std::f64::consts::PI,
};

#[test]
fn test_matrix_plus_matrix() {
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
}

#[test]
fn test_transposed_matrix_plus_matrix() {
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
    lhs.transpose();
    rhs.transpose();
    out.transpose();
    assert_eq!(&lhs + &rhs, out);
}

#[test]
fn test_matrix_minus_matrix() {
    let mut lhs = Matrix::from(vec![
        vec![6.0, 1.0],
        vec![3.0, -4.0],
        vec![18.0, 3.0]
    ]);
    let mut rhs = Matrix::from(vec![
        vec![4.0, 1.0],
        vec![3.0, -6.0],
        vec![11.0, 3.0]
    ]);
    let mut out = Matrix::from(vec![
        vec![2.0, 0.0],
        vec![0.0, 2.0],
        vec![7.0, 0.0]
    ]);
    assert_eq!(&lhs - &rhs, out);
}

#[test]
fn test_matrix_plus_vector() {
    let mut lhs = Matrix::from(vec![
        vec![2.0],
        vec![0.0],
        vec![7.0]
    ]);
    let mut rhs = Vector::from(vec![-4.0, -5.0, 6.0]);
    rhs.transpose();
    let out = Matrix::from(vec![
        vec![-2.0],
        vec![-5.0],
        vec![13.0]
    ]);
    assert_eq!(&lhs + &rhs, out);
}

#[test]
fn test_vector_plus_matrix() {
    let mut lhs = Vector::from(vec![-4.0, -5.0, 6.0]);
    let mut rhs = Matrix::from(vec![
        vec![2.0],
        vec![0.0],
        vec![7.0]
    ]);
    rhs.transpose();
    let out = Vector::from(vec![-2.0, -5.0, 13.0]);
    assert_eq!(&lhs + &rhs, out);
}

#[test]
fn test_squared_matrix_mul() {
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
}

#[test]
fn test_vector_mul_matrix() {
    let lhs = Vector::from(vec![3.0, 5.0, 6.0]);
    let rhs = Matrix::from(vec![
        vec![1.0, 1.0, 0.0],
        vec![-2.0, 4.0, 0.0],
        vec![0.0, 3.0, 5.0]
    ]);
    let out = Matrix::from(vec![
        vec![-7.0, 41.0, 30.0]
    ]);
    assert_eq!(&lhs * &rhs, out);
}

#[test]
fn test_matrix_mul_vector() {
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

#[test]
fn test_diag_det() {
    let mut m = Matrix::from(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]]
    );
    m.determine().unwrap();
    assert_eq!(m.determinant.unwrap(), 8.0);
}

#[test]
fn test_inv_diag_det() {
    let mut m = Matrix::from(vec![
        vec![0.0, 0.0, -5.0],
        vec![0.0, -5.0, 0.0],
        vec![-5.0, 7.0, 1.0]]
    );
    m.determine().unwrap();
    assert_eq!(m.determinant.unwrap(), 125.0);
}

#[test]
fn test_filled_det() {
    let mut m = Matrix::from(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    m.determine().unwrap();
    assert_eq!(m.determinant.unwrap(), 473.0);
}

#[test]
fn test_transposed_det() {
    let mut m = Matrix::from(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]]
    );
    m.determine().unwrap();
    assert_eq!(m.determinant.unwrap(), 57.0);
}

#[test]
fn test_diag_inverse() {
    let mut primal = Matrix::from(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 2.0]]
    );
    primal.determine().unwrap();
    let inversed = Matrix::from(vec![
        vec![0.5, 0.0, 0.0],
        vec![0.0, 0.5, 0.0],
        vec![0.0, 0.0, 0.5]]
    );
    assert_eq!(primal.inverse().unwrap(), inversed);
}

#[test]
fn test_filled_inverse() {
    let mut primal = Matrix::from(vec![
        vec![-3.0, -13.0, -7.0],
        vec![0.0, -11.0, 0.0],
        vec![-7.0, -5.0, -2.0]]
    );
    primal.determine().unwrap();
    let inversed = Matrix::from(vec![
        vec![2.0 / 43.0, 9.0 / 473.0, -7.0 / 43.0],
        vec![0.0, -1.0 / 11.0, 0.0],
        vec![-7.0 / 43.0, 76.0 / 473.0, 3.0 / 43.0]]
    );
    assert_eq!(primal.inverse().unwrap(), inversed);
}

#[test]
fn test_transposed_inverse() {
    let mut primal = Matrix::from(vec![
        vec![2.0, 0.0, -5.0],
        vec![0.0, -3.0, 0.0],
        vec![-5.0, 7.0, 3.0]
    ]);
    primal.determine().unwrap();
    let mut inversed = Matrix::from(vec![
        vec![-9.0 / 57.0, -35.0 / 57.0, -15.0 / 57.0],
        vec![0.0, -19.0 / 57.0, 0.0],
        vec![-15.0 / 57.0, -14.0 / 57.0, -6.0 / 57.0]
    ]);
    primal.transpose();
    inversed.transpose();
    assert_eq!(primal.inverse().unwrap(), inversed);
}

#[test]
fn test_matrix_norm() {
    let m = Matrix::from(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]]);
    assert_eq!(m.norm(), (285.0 as f64).sqrt());
}

#[test]
fn test_vector_norm() {
    let v = Vector::from(vec![-1.0, -3.0, -7.0]);
    assert_eq!(v.norm(), (59.0 as f64).sqrt());
}

#[test]
fn test_matrix_transpose() {
    let mut m = Matrix::from(vec![
        vec![4.0, 1.0],
        vec![3.0, -6.0],
        vec![11.0, 3.0]
    ]);
    let elem = m[(2, 0)];
    m.transpose();
    assert_eq!(elem, m[(0, 2)]);
}

#[test]
fn test_vector_transpose() {
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

#[test]
fn test_vector_prod() {
    let lhs = Vector::from(vec![1.0, 3.0, 4.0]);
    let rhs = Vector::from(vec![5.0, -4.0, -7.0]);
    let out = Vector::from(vec![-5.0, 27.0, -19.0]);
    assert_eq!(&lhs | &rhs, out);
}

#[test]
fn test_rotational_matrix() {
    let m = Matrix::from(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, -1.0, 0.0],
    ]);
    assert_eq!(Matrix::rot(1, 2, 1.5 * PI).unwrap(), m);
}
