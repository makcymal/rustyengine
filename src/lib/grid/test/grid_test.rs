use {
    super::super::{
        grid::{
            Repr,
            Grid::{self, *},
            Line,
            Elem,
        },
    },
    crate::{
        util::LineTp,
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
        },
    },
};


#[test]
fn fill_grid_with() {
    let grid = Grid::fill(0, 1, 3);
    assert_eq!(grid, Failure(GridErr(IsEmpty)));
}

#[test]
fn grid_from_curve_rec() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]);
    assert_eq!(grid, Failure(GridErr(CurveSides(2))));
}

#[test]
fn grid_is_matrix() {
    let grid = Grid::fill(2, 1, 3);
    assert_eq!(grid.is_arbitrary(), true);
}

#[test]
fn failure_to_matrix() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]).to_arbitrary();
    assert_eq!(grid, Failure(GridErr(UnhandledFailure)));
}

#[test]
fn grid_to_row() {
    let grid = Grid::from_single(vec![1, 2, 3]).to_row();
    assert!(grid.is_row());
}

#[test]
fn many_rows_to_row() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5, 7]]).to_row();
    assert_eq!(grid, Failure(GridErr(TooManyRows(3))));
}

#[test]
fn grid_to_col() {
    let grid = Grid::from_double(vec![vec![1], vec![4], vec![4]]).to_col();
    assert!(grid.is_col());
}

#[test]
fn many_cols_to_col() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_col();
    assert_eq!(grid, Failure(GridErr(TooManyCols(2))));
}

#[test]
fn raw_transpose_many_cols_to_col() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose().to_col();
    assert_eq!(grid, Failure(GridErr(TooManyCols(3))));
}

#[test]
fn grid_rows() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.rows(), 3);
}

#[test]
fn grid_cols() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.cols(), 2);
}

#[test]
fn raw_transposed_grid_cols() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(grid.cols(), 3);
}

#[test]
fn raw_transpose_matrix() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(*grid.att(0, 1), 4);
}

#[test]
fn transposed_grid_col_is_row() {
    let grid = Grid::from_single(vec![0, 1, 2, 3, 4]).raw_transpose().to_col().transpose();
    assert!(grid.is_row());
}

#[test]
fn transposed_grid_row_is_transposed() {
    let grid = Grid::from_single(vec![0, 1, 2, 3, 4]).to_row().transpose();
    assert!(grid.is_transposed());
}

#[test]
fn transposed_grid_rowlist_is_collist() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_multirow().transpose();
    assert!(grid.is_multicol());
}

#[test]
fn transposed_grid_rowlist_col_sum() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_multirow().transpose();
    assert_eq!(
        grid.col_iter().unwrap()
            .next().unwrap().sum::<i32>(),
        3
    );
}

#[test]
fn grid_is_not_transposed() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.is_transposed(), false);
}

#[test]
fn grid_is_transposed() {
    let grid = Grid::from_double(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(grid.is_transposed(), true);
}

#[test]
fn at_lin_row_grid() {
    let grid = Grid::from_single(vec![0, 1, 2, 3]).to_row();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn at_rec_row_grid() {
    let grid = Grid::from_double(vec![vec![0], vec![1], vec![2], vec![3]]).raw_transpose().to_row();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn at_rec_col_grid() {
    let grid = Grid::from_double(vec![vec![0], vec![1], vec![2], vec![3]]).to_col();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn att_rec_matrix_grid() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    assert_eq!(*grid.att(2, 1), 8);
}

#[test]
fn att_rec_rowlist_grid() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_multirow();
    assert_eq!(*grid.att(2, 1), 8);
}

#[test]
fn att_rec_collist_grid() {
    let grid = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_multicol();
    assert_eq!(*grid.att(2, 1), 6);
}

#[test]
fn att_lin_matrix_grid() {
    let grid = Grid::from_single(vec![0, 1, 2, 3]).to_arbitrary();
    assert_eq!(*grid.att(0, 2), 2);
}

#[test]
fn att_lin_collist_grid() {
    let grid = Grid::from_single(vec![1]).to_multicol();
    assert_eq!(*grid.att(0, 0), 1);
}

#[test]
fn equal_grid() {
    let lhs = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_multicol();
    let rhs = Grid::from_double(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).to_multicol().raw_transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn inequal_grid() {
    let lhs = Grid::from_double(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_multicol();
    let rhs = Grid::from_double(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).to_multicol().transpose();
    assert_ne!(lhs, rhs);
}


#[test]
fn rec_grid_row_iter() {
    let grid = Grid::from_double(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_multirow();
    assert_eq!(
        *grid.row_iter().unwrap()
            .next().unwrap()
            .next().unwrap(),
        0
    );
}

#[test]
fn rec_grid_col_iter_sum() {
    let grid = Grid::from_double(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_multicol();
    let mut col_iter = grid.col_iter().unwrap();
    assert_eq!(col_iter.next().unwrap().sum::<i32>(), 12);
}

#[test]
fn rec_grid_next_col_iter_sum() {
    let grid = Grid::from_double(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_multicol();
    let mut col_iter = grid.col_iter().unwrap();
    col_iter.next();
    assert_eq!(col_iter.next().unwrap().sum::<i32>(), 15);
}

#[test]
fn lin_grid_row_iter() {
    let grid = Grid::from_single(vec![0, 1, 2, 3]).to_multirow();
    assert_eq!(
        *grid.row_iter().unwrap()
            .last().unwrap()
            .last().unwrap(),
        3
    );
}
