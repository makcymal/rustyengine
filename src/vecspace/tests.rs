use {
    super::matrixified::*,
    crate::utils::Size,
};

fn sq_matrices() -> [Matrix<i8>; 5] {
    [
        // det = 1
        Matrix::<i8>::identity(Size::Rect((3, 3))).unwrap(),
        // det = 8
        Matrix::<i8>::from(vec![
            vec![2, 0, 0],
            vec![0, 2, 0],
            vec![0, 0, 2]]),
        // det = -125
        Matrix::<i8>::from(vec![
            vec![0, 0, -5],
            vec![0, -5, 0],
            vec![-5, 7, 1]]),
        // det = 57
        Matrix::<i8>::from(vec![
            vec![2, 0, -5],
            vec![0, -3, 0],
            vec![-5, 7, 3]]),
        // det = 0
        Matrix::<i8>::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]])
    ]
}

fn vt_matrices() -> [Matrix<i8>; 3] {
    [
        Matrix::<i8>::from(vec![
            vec![2, 0],
            vec![0, 2],
            vec![7, 0]]),
        Matrix::<i8>::from(vec![
            vec![4, 1],
            vec![3, -6],
            vec![11, 3]]),
        Matrix::<i8>::from(vec![
            vec![3],
            vec![0],
            vec![-5]]),
    ]
}

fn hr_matrices() -> [Matrix<i8>; 3] {
    [
        Matrix::<i8>::from(vec![
            vec![2, 3, 4],
            vec![1,-8, 3]]),
        Matrix::<i8>::from(vec![
            vec![-3, -2, -1]]),
        Matrix::<i8>::from(vec![
            vec![0, -2, -6]]),
    ]
}

fn vectors() -> [Vector<i8>; 4] {
    [
        Vector::<i8>::fill_with(Size::Row(3), 4),
        Vector::<i8>::fill_with(Size::Col(3), -1),
        Vector::<i8>::from(vec![-1, -3, -7]),
        Vector::<i8>::from(vec![-7, -6, -5]),
    ]
}

#[test]
fn test_add() {
    let mut sqm = sq_matrices();
    assert_eq!((&sqm[0] + &sqm[0]).unwrap(), sqm[1]);
    assert_eq!((&sqm[1] + &sqm[2]).unwrap(), sqm[3]);
    assert_eq!((&sqm[3] - &sqm[2]).unwrap(), sqm[1]);

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
fn test_minor() {
    let mut sqm = sq_matrices();
    for m in sqm.iter_mut() {
        m.determine().unwrap();
        dbg!(m.determinant.unwrap());
    }

    assert_eq!(sqm[0].determinant.unwrap(), 1);
    assert_eq!(sqm[1].determinant.unwrap(), 8);
    assert_eq!(sqm[2].determinant.unwrap(), -125);
    assert_eq!(sqm[3].determinant.unwrap(), 57);
    assert_eq!(sqm[4].determinant.unwrap(), 0);
}
