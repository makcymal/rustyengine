use {
    super::{
        matrixify::{*},
        enums::{
            Repr::{self, *},
            MatrErr::{self, *},
        },
    },
};

#[test]
fn test_identity() {
    let id = Matrixify::identity(3, Square);
    assert_eq!(*id.att((0, 0)).unwrap(), 1.0);
}

#[test]
fn test_fillwith() {
    let m = Matrixify::fill_with((2, 3), RowList, 5.0);
    assert_eq!(*m.att((1, 2)).unwrap(), 5.0);
}

#[test]
fn test_validate_emptiness() {
    let m = Matrixify::from_solo(vec![], Matrix);
    assert_eq!(m.repr(), Failure(EmptyAtAll));
}

#[test]
fn test_validate_curveness() {
    let m = Matrixify::from_duet(vec![
        vec![1.0, 2.0],
        vec![3.0],
    ], Matrix);
    assert_eq!(m.repr(), Failure(CurveSides));
}

#[test]
fn test_transpose() {
    let m = Matrixify::from_duet(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ], Matrix).transpose();
    assert_eq!(*m.att((0, 1)).unwrap(), 3.0);
}

#[test]
fn test_switch_rowlist_to_collist() {
    let m = Matrixify::from_duet(vec![
        vec![1.0, 2.0, 5.0],
        vec![3.0, 4.0, 6.0],
    ], RowList).switch(ColList);
    assert_eq!(m.repr(), ColList);
}

#[test]
fn test_att_collist() {
    let m = Matrixify::from_duet(vec![
        vec![1.0, 2.0, 5.0],
        vec![3.0, 4.0, 6.0],
    ], ColList);
    assert_eq!(*m.att((0, 1)).unwrap(), 3.0);
}

#[test]
fn test_rows() {
    let m = Matrixify::from_duet(vec![
        vec![1.0, 2.0, 5.0],
        vec![3.0, 4.0, 6.0],
    ], ColList).switch(RowList);
    assert_eq!(m.rows().unwrap(), 3);
}
