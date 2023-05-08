mod grid_errs;
mod matr_errs;

pub use {
    grid_errs::GridErr,
    matr_errs::MatrErr,
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyRes<T> {
    Go(T),
    No(AnyErr),
}

impl<T> AnyRes<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Go(val) => val,
            Self::No(err) => panic!("{}", format!("Unwrap on AnyRes::No({:?})", err)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyErr {
    GridErr(GridErr),
    MatrErr(MatrErr),
}
