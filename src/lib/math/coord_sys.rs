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
    once_cell::sync::{
        Lazy, OnceCell,
    },
};


#[derive(Debug, Clone, PartialEq)]
pub struct VectorSpace {
    /// Intended to be `Matr::ColList`
    basis: Matrix,
    gram: Matrix,
}

impl VectorSpace {
    pub fn new(mut basis: Matrix) -> ReRes<Self> {
        basis.ag_failed()?.ag_not_stratified()?.ag_not_square()?;
        if basis.is_multirow() {
            basis = basis.transpose();
        }
        Ok(Self {
            gram: basis.multi_scalar_prod(&basis).unwrap(),
            basis,
        })
    }

    pub fn gram(&self) -> &Matrix {
        &self.gram
    }

    pub fn scalar_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<f64> {
        lhs.approve_single_vector_ops(rhs)?;
        Ok(*lhs.raw_scalar_prod(rhs, &self.gram)?.att(0, 0))
    }

    pub fn vector_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<Matrix> {
        lhs.approve_multi_vector_ops(rhs)?;
        lhs.ag_not_3_dim()?;

        static DUAL: OnceCell<Matrix> = OnceCell::new();
        let dual = DUAL.get_or_init(|| {
            self.basis.vector_prod_at(1, &self.basis, 2).unwrap()
                .append_cols(self.basis.vector_prod_at(3, &self.basis, 1).unwrap()).unwrap()
                .append_cols(self.basis.vector_prod_at(1, &self.basis, 2).unwrap()).unwrap()
        });

        let coef = vec![
            lhs.att(0, 1) * rhs.att(0, 2) - lhs.att(0, 2) * rhs.att(0, 1),
            lhs.att(0, 2) * rhs.att(0, 0) - lhs.att(0, 0) * rhs.att(0, 2),
            lhs.att(0, 0) * rhs.att(0, 1) - lhs.att(0, 1) * rhs.att(0, 0),
        ];
        dual.combine(coef)
    }

    pub fn decompose_pt(&self, pt: &Point) -> Matrix {
        pt.radvec.mul_left(&self.basis.inv().expect("matrix of basis vector haven't inversed"))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    radvec: Matrix,
}

impl Point {
    pub fn new(mut radvec: Matrix) -> ReRes<Self> {
        radvec.ag_failed()?.ag_not_row_or_col()?;
        if let Repr::Row = radvec.repr() {
            radvec = radvec.transpose();
        }
        Ok(Self { radvec })
    }

    pub fn mv(&mut self, vec: &Matrix) -> ReRes<()> {
        vec.ag_failed()?.ag_not_row_or_col()?;
        self.radvec.approve_single_vector_ops(vec)?;
        self.radvec = match vec.repr() {
            Repr::Col => self.radvec.add(vec),
            Repr::Row => self.radvec.add_t(vec),
            _ => unreachable!(),
        };
        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct CoordSys {
    initpt: Point,
    space: VectorSpace,
}

impl CoordSys {
    pub fn new(initpt: Point, space: VectorSpace) -> Self {
        Self { initpt, space }
    }

    pub fn initpt(&self) -> &Point {
        &self.initpt
    }

    pub fn space(&self) -> &VectorSpace {
        &self.space
    }

    pub fn gram(&self) -> &Matrix {
        self.space.gram()
    }
}
