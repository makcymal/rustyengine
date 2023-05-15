use {
    super::{
        matrix::Matrix,
        get_biform,
    },
    crate::{
        grid::Repr,
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MatrixErr::{self, *},
            CoordSysErr::{self, *},
        },
    },
    once_cell::sync::Lazy,
};


#[derive(Debug, Clone, PartialEq)]
pub struct VectorSpace {
    /// Intended to be `Matr::ColList`
    basis: Matrix,
    gram: Matrix,
}

impl VectorSpace {
    pub fn new(mut basis: Matrix) -> ReRes<Self> {
        if basis.failed() {
            return Err(GridErr(UnhandledFailure));
        } else if !basis.is_multirow() && !basis.is_multicol() {
            return Err(MatrixErr(TooArbitrary));
        } else if basis.rows() != basis.cols() {
            return Err(CoordSysErr(CurveBasis((basis.rows(), basis.cols()))));
        } else if basis.is_multicol() {
            basis = basis.transpose();
        }
        let gram = basis.mul(get_biform()).mul_t(&basis);
        Ok(Self { basis, gram })
    }

    pub fn scalar_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<f64> {
        lhs.approve_vector_ops(rhs)?;
        let first = match lhs {
            Matrix::Row(_) => lhs.mul(get_biform()),
            Matrix::Col(_) => lhs.mul_left_t(get_biform()).transpose(),
            _ => unreachable!(),
        };
        Ok(match rhs {
            Matrix::Row(_) => *first.mul_t(rhs).att(0, 0),
            Matrix::Col(_) => *first.mul(rhs).att(0, 0),
            _ => unreachable!()
        })
    }

    // pub fn vector_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<Matrix> {
    //     lhs.approve_vector_ops(rhs);
    //     if lhs.dim() != Ok(3) {
    //         return Err(MatrixErr(VectorProdDimMismatch { lhs: lhs.dim().unwrap(), rhs: rhs.dim().unwrap() }))
    //     }
    //
    //     static DUAL: Lazy<Vec<Matrix>> = Lazy::new(|| {
    //         let mut dual = vec![];
    //         dual.push(self.basis.vector_prod_of(1, &self.basis, 2).unwrap());
    //         dual.push(self.basis.vector_prod_of(3, &self.basis, 1).unwrap());
    //         dual.push(self.basis.vector_prod_of(1, &self.basis, 2).unwrap());
    //         dual
    //     });
    // }
}
