use {
    super::super::*,
    crate::{
        errs::{GridErr::*, MathErr::*, ReErr::*},
        grid::Repr::*,
    },
};

#[test]
fn normalize_vector_without_basis() {
    set_biform(Matrix::identity(3).num_mul_assign(3.0));
    let mut vector = Vector::new(vec![1.0, -3.0, 4.0]);
    vector = vector.normalize();
    let mut nvector = Vector::new(vec![1.0, -3.0, 4.0]);
    nvector = nvector.resize(1.0 / 78.0_f64.sqrt());
    assert_eq!(vector, nvector);
}

#[test]
fn normalize_vector_in_basis() {
    set_biform(Matrix::identity(3).num_mul_assign(2.0));
    let cs = CoordSys::default();
    let mut vector = Matrix::from_single(vec![2.0, -1.0, 0.0])
        .raw_transpose()
        .to_col();
    cs.normalize(&mut vector);
    let len = 10.0_f64.sqrt();
    let norm = Matrix::from_single(vec![2.0, -1.0, 0.0])
        .raw_transpose()
        .to_col()
        .num_div_assign(len);
    assert_eq!(vector, norm);
}

#[test]
fn mv_point() {
    let mut point = Point::new(vec![1.0, 2.0, 3.0]);
    point = point.mv(&Vector::new(vec![3.0, 2.0, 1.0])).unwrap();
    assert_eq!(point, Point::new(vec![4.0; 3]));
}

#[test]
fn fail_to_mv_point() {
    let mut point = Point::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(
        point.mv(&Vector::new(vec![3.0, 2.0, 1.0, 0.0])),
        Err(MathErr(DimMismatch { lhs: 3, rhs: 4 }))
    );
}

#[test]
fn mv_point_and_back() {
    let mut point = Point::default();
    point = point
        .mv(&Vector::new(vec![1.0, 2.0, 3.0]))
        .unwrap()
        .mv(&Vector::new(vec![1.0, 2.0, 3.0]).resize(-1.0))
        .unwrap();
    assert_eq!(point, Point::default());
}

#[test]
fn df_points() {
    let p1 = Point::new(vec![1.0, 2.0, 3.0]);
    let p2 = Point::new(vec![-2.0, -3.0, 0.0]);
    assert_eq!(p1.df(&p2).unwrap(), Vector::new(vec![3.0, 5.0, 3.0]))
}

#[test]
fn basis_not_stratified() {
    let basis = Basis::new(Matrix::identity(3));
    assert_eq!(basis, Err(GridErr(NotMultiRowOrCol)));
}

#[test]
fn linear_dep_basis() {
    let basis = Basis::new(Matrix::new(3, 3, 0.0).to_multicol());
    assert_eq!(basis, Err(MathErr(NullDeterminant)));
}

#[test]
fn transposed_basis() {
    let basis = Basis::new(
        Matrix::from_double(vec![
            vec![1.0, 1.0, 1.0],
            vec![0.0, 1.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ])
        .to_multirow(),
    )
    .unwrap();
    assert_eq!(basis.basis.att(1, 2), &1.0);
}

#[test]
fn point_decomposition_in_basis() {
    set_biform_identity();
    let basis = Basis::new(Matrix::identity(3).num_mul_assign(2.0).to_multicol()).unwrap();
    let point = Point::new(vec![5.0, 5.0, 5.0]);
    assert_eq!(basis.decompose(&point), Vector::new(vec![2.5, 2.5, 2.5]));
}
