//! Types related to analytical geometry like `VectorSpace`, `Point`, `CoordSys`.
//! Within vector space it becomes possible to define scalar and vector product in some basis.
//! `Col` and `MultiCol` is preferrable representation for vectors

use {
    super::{
        matrix::Matrix,
        get_biform,
        precision::{round, eq},
    },
    crate::{
        grid::Repr,
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
    },
    once_cell::sync::OnceCell,
};


/// Vector space that defined by basis that is `Matrix::MultiCol`, square, linear independence
#[derive(Debug, Clone, PartialEq)]
pub struct VectorSpace {
    basis: Matrix,
    ortho: bool,
}

impl VectorSpace {
    /// Constructor for vector space, validates basis and checks whether it's orthogonal
    pub fn new(mut basis: Matrix) -> ReRes<Self> {
        basis.ag_failed()?.ag_not_stratified()?.ag_not_square()?.ag_linear_dependence()?;
        if basis.is_multirow() {
            basis = basis.transpose();
        }
        let ortho =
            eq(basis.scalar_prod_at(0, &basis, 1).unwrap(), 0.0) &&
            eq(basis.scalar_prod_at(0, &basis, 2).unwrap(), 0.0) &&
            eq(basis.scalar_prod_at(1, &basis, 2).unwrap(), 0.0);

        Ok(Self {
            basis,
            ortho,
        })
    }

    /// Gram matrix of current basis
    pub fn gram(&self) -> &Matrix {
        static GRAM: OnceCell<Matrix> = OnceCell::new();
        GRAM.get_or_init(|| {
            self.basis.multi_scalar_prod(&self.basis).unwrap()
        })
    }

    /// Dual basis with repr `Matrix::MultiCol`, not accurating to their length
    pub fn dual(&self) -> &Matrix {
        static DUAL: OnceCell<Matrix> = OnceCell::new();
        DUAL.get_or_init(|| {
            self.basis
                .vector_prod_at(1, &self.basis, 2).unwrap()
                .append_cols(self.basis
                    .vector_prod_at(3, &self.basis, 1).unwrap()).unwrap()
                .append_cols(self.basis
                    .vector_prod_at(1, &self.basis, 2).unwrap()).unwrap()
        })
    }

    /// Length of `Row` or `Col` in basis according to `self.gram()` matrix
    pub fn len(&self, vec: &Matrix) -> ReRes<f64> {
        Ok(self.scalar_prod(vec, vec)?.sqrt())
    }

     /// Length of vector in the given index `i` in `Row`, `MultiRow`, `Col` or `MultiCol` in basis
     /// according to `self.gram` matrix
    pub fn len_at(&self, vec: &Matrix, i: usize) -> ReRes<f64> {
        Ok(self.scalar_prod_at(vec, i, vec, i)?.sqrt())
    }

    /// Resizing each vector to length 1 using scalar product in basis
    pub fn normalize(&self, vec: &mut Matrix) {
        match vec.repr() {
            Repr::Row | Repr::MultiRow => {
                for r in 0..vec.rows() {
                    let len = self.len_at(vec, r).unwrap();
                    for c in 0..vec.cols() {
                        let elem = *vec.att(r, c);
                        *vec.att_mut(r, c) = round(elem / len);
                    }
                }
            }
            Repr::Col | Repr::MultiCol => {
                for c in 0..vec.cols() {
                    let len = self.len_at(vec, c).unwrap();
                    for r in 0..vec.rows() {
                        let elem = *vec.att(c, r);
                        *vec.att_mut(c, r) = round(elem / len);
                    }
                }
            }
            _ => {}
        }
    }

    /// Scalar product in basis according to `self.gram()` matrix.
    /// Operands must have single `Row` or `Col` having the same dim. Produces `f64`
    pub fn scalar_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<f64> {
        lhs.approve_single_vector_ops(rhs)?;
        Ok(*lhs.raw_scalar_prod(rhs, self.gram())?.att(0, 0))
    }

    /// Scalar product in basis according to `self.gram()` matrix.
    /// Operands at the given indices must have the same dim. Produces `f64`
    pub fn scalar_prod_at(&self, lhs: &Matrix, l: usize, rhs: &Matrix, r: usize) -> ReRes<f64> {
        Ok(lhs.raw_scalar_prod_at(l, rhs, r, self.gram())?)
    }

    /// Scalar product in basis according to `self.gram()` matrix.
    /// Operands must have `Row` or `Col` of the same dim.
    /// Produces `Arbitrary` matrix of `f64`, that is pair-wise scalar products
    pub fn multi_scalar_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<Matrix> {
        Ok(lhs.raw_scalar_prod(rhs, self.gram())?)
    }

    /// Vector product
    pub fn vector_prod(&self, lhs: &Matrix, rhs: &Matrix) -> ReRes<Matrix> {
        lhs.approve_single_vector_ops(rhs)?;
        self.vector_prod_at(lhs, 0, rhs, 0)
    }

    /// Vector product between vectors on given indices
    pub fn vector_prod_at(&self, lhs: &Matrix, l: usize, rhs: &Matrix, r: usize) -> ReRes<Matrix> {
        lhs.approve_multi_vector_ops(rhs)?;
        lhs.ag_not_3_dim()?;

        let coef = vec![
            round(lhs.att(l, 1) * rhs.att(r, 2) - lhs.att(l, 2) * rhs.att(r, 1)),
            round(lhs.att(l, 2) * rhs.att(r, 0) - lhs.att(l, 0) * rhs.att(r, 2)),
            round(lhs.att(l, 0) * rhs.att(r, 1) - lhs.att(l, 1) * rhs.att(r, 0)),
        ];
        self.dual().combine(coef)
    }

    /// Decompose point in current basis
    pub fn decompose_pt(&self, pt: &Point) -> Matrix {
        pt.radvec.mul_left(&self.basis.inv().expect("matrix of basis vector haven't inversed"))
    }
}


/// Point that defined by `Matrix::Col` as radius vector
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    radvec: Matrix,
}

impl Point {
    /// Constructs point validating given radius vector
    pub fn new(mut radvec: Matrix) -> ReRes<Self> {
        radvec.ag_failed()?.ag_not_row_or_col()?;
        if let Repr::Row = radvec.repr() {
            radvec = radvec.transpose();
        }
        Ok(Self { radvec })
    }

    /// Moves point on the given vector taking point by value and returning it
    pub fn mv(mut self, vec: &Matrix) -> ReRes<Self> {
        self.mv_assign(vec)?;
        Ok(self)
    }

    /// Moves point on the given vector on place
    pub fn mv_assign(&mut self, vec: &Matrix) -> ReRes<()> {
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


/// Coordinate system as combination of initial point and basis of given vector space
#[derive(Debug, Clone, PartialEq)]
pub struct CoordSys {
    initpt: Point,
    space: VectorSpace,
}

impl CoordSys {
    /// Constructs `CoordSys` taking initial point and vector space
    pub fn new(initpt: Point, space: VectorSpace) -> Self {
        Self { initpt, space }
    }

    /// Access to `initpt` field
    pub fn initpt(&self) -> &Point {
        &self.initpt
    }

    /// Access to `space` field
    pub fn space(&self) -> &VectorSpace {
        &self.space
    }

    /// Ref to actual gram matrix
    pub fn gram(&self) -> &Matrix {
        self.space.gram()
    }
}
