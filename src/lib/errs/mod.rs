mod grid_errs;
mod matr_errs;

pub use {
    grid_errs::GridErr,
    matr_errs::MatrErr,
};

pub type AnyRes<T> = Result<T, AnyErr>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyErr {
    GridErr(GridErr),
    MatrErr(MatrErr),
}
