use {
    super::{
        raw_grid::{
            VecWrapper::{self, *},
            RawGrid,
        },
        grid::{
            Repr,
            Grid::{self, *},
            LineIter,
            ElemIter,
        },
    },
    crate::{
        util::Line,
        errs::{
            AnyRes,
            AnyErr::{self, *},
            GridErr::{self, *},
        },
    },
};


// <<< RawGrid tests
#[test]
fn is_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Lin(vec![]);
    assert_eq!(vw.is_lin(), true);
}

#[test]
fn is_not_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Rec(vec![]);
    assert_eq!(vw.is_lin(), false);
}


#[test]
fn empty_lin_vec_wrapper() {
    let vw: VecWrapper<f64> = Lin(vec![]);
    assert_eq!(vw.is_valid(), Err(GridErr(IsEmpty)));
}

#[test]
fn curve_rec_vec_wrapper() {
    let vw = Rec(vec![vec![1, 2], vec![1]]);
    assert_eq!(vw.is_valid(), Err(GridErr(CurveSides(1))));
}

#[test]
fn valid_rec_vec_wrapper() {
    let vw = Rec(vec![vec![1, 2], vec![1, 2]]);
    assert!(vw.is_valid().is_ok());
}

#[test]
fn rows_rec_vec_wrapper() {
    let vw = Rec(vec![vec![1, 2, 3], vec![1, 2, 3]]);
    assert_eq!(vw.rows(), 2);
}

#[test]
fn cols_rec_vec_wrapper() {
    let vw = Rec(vec![vec![1, 2, 3], vec![1, 2, 3]]);
    assert_eq!(vw.cols(), 3);
}

#[test]
fn att_rec_vec_wrapper() {
    let vw = Rec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    assert_eq!(*vw.att(1, 2), 6);
}

#[test]
fn curve_from_rec_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]);
    assert_eq!(rg, Err(GridErr(CurveSides(2))));
}

#[test]
fn rows_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(rg.rows(false), 2);
}

#[test]
fn trans_cols_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.cols(false), 2);
}

#[test]
fn raw_grid_is_not_transposed() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(rg.is_transposed(), false);
}

#[test]
fn raw_grid_is_transposed() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.is_transposed(), true);
}

#[test]
fn t_trans_cols_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(rg.cols(true), 3);
}

#[test]
fn att_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(*rg.att(0, 1, false), 2);
}

#[test]
fn att_trans_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(*rg.att(0, 1, false), 4);
}

#[test]
fn att_t_trans_raw_grid() {
    let rg = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(*rg.att(0, 1, true), 2);
}

#[test]
fn equal_raw_grids() {
    let lhs = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    let rhs = RawGrid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap().transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn equal_rec_lin_raw_grids() {
    let lhs = RawGrid::from_rec(vec![vec![1, 2, 3]]).unwrap();
    let rhs = RawGrid::from_lin(vec![1, 2, 3]).unwrap();
    assert_eq!(lhs, rhs);
}

#[test]
fn equal_rec_lin_transposed_raw_grids() {
    let lhs = RawGrid::from_rec(vec![vec![1], vec![2], vec![3]]).unwrap();
    let rhs = RawGrid::from_lin(vec![1, 2, 3]).unwrap().transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn inequal_raw_grids() {
    let lhs = RawGrid::from_rec(vec![vec![1], vec![2], vec![3]]).unwrap();
    let rhs = RawGrid::from_lin(vec![1, 2, 3]).unwrap();
    assert_ne!(lhs, rhs);
}

// RawGrid tests >>>


// <<< Grid tests

#[test]
fn fill_grid_with() {
    let grid = Grid::fill_with(0, 1, 3);
    assert_eq!(grid, Failure(GridErr(IsEmpty)));
}

#[test]
fn grid_from_curve_rec() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]);
    assert_eq!(grid, Failure(GridErr(CurveSides(2))));
}

#[test]
fn grid_is_matrix() {
    let grid = Grid::fill_with(2, 1, 3);
    assert_eq!(grid.is_matrix(), true);
}

#[test]
fn failure_to_matrix() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5]]).to_matrix();
    assert_eq!(grid, Failure(GridErr(UnhandledFailure)));
}

#[test]
fn grid_to_row() {
    let grid = Grid::from_lin(vec![1, 2, 3]).to_row();
    assert!(grid.is_row());
}

#[test]
fn many_rows_to_row() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5, 7]]).to_row();
    assert_eq!(grid, Failure(GridErr(TooManyRows(3))));
}

#[test]
fn grid_to_col() {
    let grid = Grid::from_rec(vec![vec![1], vec![4], vec![4]]).to_col();
    assert!(grid.is_col());
}

#[test]
fn many_cols_to_col() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_col();
    assert_eq!(grid, Failure(GridErr(TooManyCols(2))));
}

#[test]
fn raw_transpose_many_cols_to_col() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose().to_col();
    assert_eq!(grid, Failure(GridErr(TooManyCols(3))));
}

#[test]
fn grid_rows() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.rows(), 3);
}

#[test]
fn grid_cols() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.cols(), 2);
}

#[test]
fn raw_transposed_grid_cols() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(grid.cols(), 3);
}

#[test]
fn raw_transpose_matrix() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(*grid.att(0, 1), 4);
}

#[test]
fn transposed_grid_col_is_row() {
    let grid = Grid::from_lin(vec![0, 1, 2, 3, 4]).raw_transpose().to_col().transpose();
    assert!(grid.is_row());
}

#[test]
fn transposed_grid_row_is_transposed() {
    let grid = Grid::from_lin(vec![0, 1, 2, 3, 4]).to_row().transpose();
    assert!(grid.is_transposed());
}

#[test]
fn transposed_grid_rowlist_is_collist() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_rowlist().transpose();
    assert!(grid.is_collist());
}

#[test]
fn transposed_grid_rowlist_col_sum() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).to_rowlist().transpose();
    assert_eq!(
        grid.col_iter().unwrap()
            .next().unwrap().sum::<i32>(),
        3
    );
}

#[test]
fn grid_is_not_transposed() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]);
    assert_eq!(grid.is_transposed(), false);
}

#[test]
fn grid_is_transposed() {
    let grid = Grid::from_rec(vec![vec![1, 2], vec![4, 5], vec![4, 5]]).raw_transpose();
    assert_eq!(grid.is_transposed(), true);
}

#[test]
fn at_lin_row_grid() {
    let grid = Grid::from_lin(vec![0, 1, 2, 3]).to_row();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn at_rec_row_grid() {
    let grid = Grid::from_rec(vec![vec![0], vec![1], vec![2], vec![3]]).raw_transpose().to_row();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn at_rec_col_grid() {
    let grid = Grid::from_rec(vec![vec![0], vec![1], vec![2], vec![3]]).to_col();
    assert_eq!(*grid.at(2), 2);
}

#[test]
fn att_rec_matrix_grid() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    assert_eq!(*grid.att(2, 1), 8);
}

#[test]
fn att_rec_rowlist_grid() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_rowlist();
    assert_eq!(*grid.att(2, 1), 8);
}

#[test]
fn att_rec_collist_grid() {
    let grid = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_collist();
    assert_eq!(*grid.att(2, 1), 6);
}

#[test]
fn att_lin_matrix_grid() {
    let grid = Grid::from_lin(vec![0, 1, 2, 3]).to_matrix();
    assert_eq!(*grid.att(0, 2), 2);
}

#[test]
fn att_lin_collist_grid() {
    let grid = Grid::from_lin(vec![1]).to_collist();
    assert_eq!(*grid.att(0, 0), 1);
}

#[test]
fn equal_grid() {
    let lhs = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_collist();
    let rhs = Grid::from_rec(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).to_collist().raw_transpose();
    assert_eq!(lhs, rhs);
}

#[test]
fn inequal_grid() {
    let lhs = Grid::from_rec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).to_collist();
    let rhs = Grid::from_rec(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).to_collist().transpose();
    assert_ne!(lhs, rhs);
}

// Grid tests >>>


// <<< Grid Iterators

#[test]
fn rec_grid_row_iter() {
    let grid = Grid::from_rec(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_rowlist();
    assert_eq!(
        *grid.row_iter().unwrap()
            .next().unwrap()
            .next().unwrap(),
        0
    );
}

#[test]
fn rec_grid_col_iter_sum() {
    let grid = Grid::from_rec(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_collist();
    let mut col_iter = grid.col_iter().unwrap();
    assert_eq!(col_iter.next().unwrap().sum::<i32>(), 12);
}

#[test]
fn rec_grid_next_col_iter_sum() {
    let grid = Grid::from_rec(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).to_collist();
    let mut col_iter = grid.col_iter().unwrap();
    col_iter.next();
    assert_eq!(col_iter.next().unwrap().sum::<i32>(), 15);
}

#[test]
fn lin_grid_row_iter() {
    let grid = Grid::from_lin(vec![0, 1, 2, 3]).to_rowlist();
    assert_eq!(
        *grid.row_iter().unwrap()
            .last().unwrap()
            .last().unwrap(),
        3
    );
}

// Grid Iterators >>>
