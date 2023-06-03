use {
    super::*,
    crate::{
        grid::Repr::{self, *},
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
    },
};


/// Vector that is `Matrix::Col`
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub(crate) coord: Matrix,
}

impl Vector {
    /// Vector as column with the given coordinates
    pub fn new(coord: Vec<f64>) -> Self {
        Self {
            coord: Matrix::from_single(coord).raw_transpose().to_col()
        }
    }

    /// Ref to `Matrix` represents coordinates
    pub fn coord(&self) -> &Matrix {
        &self.coord
    }

    /// Element in `i` position
    pub fn at(&self, i: usize) -> f64 {
        *self.coord.at(i)
    }

    /// Mut ref to element in `i` position
    pub fn at_mut(&mut self, i: usize) -> &mut f64 {
        self.coord.at_mut(i)
    }

    /// Switches representation to row keeping the same coordinates
    pub fn to_row(mut self) -> Self {
        if self.coord.is_col() {
            Self {
                coord: self.coord.raw_transpose().to_row()
            }
        } else {
            self
        }
    }

    /// Switches representation to column keeping the same coordinates
    pub fn to_col(self) -> Self {
        if self.coord.is_row() {
            Self {
                coord: self.coord.raw_transpose().to_col()
            }
        } else {
            self
        }
    }

    pub fn scalar_prod(&self, rhs: &Vector) -> ReRes<f64> {
        self.coord.scalar_prod(&rhs.coord)
    }

    pub fn vector_prod(&self, rhs: &Vector) -> ReRes<Self> {
        Ok(Self { coord: self.coord.vector_prod(rhs.coord())? })
    }

    pub fn normalize(mut self) -> Self {
        Self { coord: self.coord.normalize() }
    }

    pub fn resize(mut self, coef: f64) -> Self {
        for i in 0..self.coord.dim().unwrap() {
            *self.coord.at_mut(i) *= coef;
        }
        self
    }

    /// Dimension of vector space where radius vector of point lays
    pub fn dim(&self) -> usize {
        self.coord.dim().unwrap()
    }
}


/// Point that defined by `Matrix::Col` as radius vector
pub type Point = Vector;

impl Point {
    /// Moves point on the given vector taking point by value and returning it
    pub fn mv(mut self, vec: &Vector) -> ReRes<Self> {
        self.mv_assign(vec)?;
        Ok(self)
    }

    /// Moves point on the given vector on place
    pub fn mv_assign(&mut self, vec: &Vector) -> ReRes<()> {
        self.coord.approve_single_vector_ops(vec.coord())?;
        self.coord = match vec.coord().repr() {
            Col => self.coord.add(vec.coord()),
            Row => self.coord.add_t(vec.coord()),
            _ => unreachable!(),
        };
        Ok(())
    }

    /// Vector that can be applied to move `other` to get into `self`
    pub fn df(&self, other: &Self) -> ReRes<Vector> {
        match self.coord.clone().sub(&other.coord) {
            Matrix::Failure(err) => Err(err),
            col => Ok(Vector { coord: col }),
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(vec![0.0; 3])
    }
}


/// Basis that is `Matrix::MultiCol`, square, linear independence
#[derive(Debug, Clone, PartialEq)]
pub struct Basis {
    pub(crate) basis: Matrix,
}

impl Basis {
    /// Constructor for basis, validates it and makes it `Multicol`
    pub fn new(mut basis: Matrix) -> ReRes<Self> {
        basis.ag_failed()?.ag_not_stratified()?.ag_not_square()?.ag_linear_dependence()?;
        if basis.is_multirow() {
            basis = basis.transpose();
        }
        Ok(Self { basis })
    }
}

impl Default for Basis {
    fn default() -> Self {
        Basis {
            basis: Matrix::identity(3).to_multicol(),
        }
    }
}


/// Coordinate system as combination of initial point, basis of given vector space
#[derive(Debug, Clone, PartialEq)]
pub struct CoordSys {
    pub(crate) initpt: Point,
    pub(crate) space: Basis,
    pub(crate) gram: Matrix,
    pub(crate) dual: Option<Matrix>,
    pub(crate) is_ortho: bool,
}

impl CoordSys {
    /// Constructor for coordinate system, takes initial point, vector space defined with it's basis.
    /// They must have 3 dimensions.
    /// Furthermore Gram matrix, dual basis is computed and whether basis is orthogonal.
    pub fn new(initpt: Point, space: Basis) -> ReRes<Self> {
        initpt.coord.ag_not_3_dim()?;
        space.basis.ag_not_3_dim()?;
        let gram = space.basis.multi_scalar_prod(&space.basis).unwrap();
        let dual = None;
        let is_ortho = aeq(&space.basis.scalar_prod_at(0, &space.basis, 1).unwrap(), &0.0) &&
            aeq(&space.basis.scalar_prod_at(0, &space.basis, 2).unwrap(), &0.0) &&
            aeq(&space.basis.scalar_prod_at(1, &space.basis, 2).unwrap(), &0.0);
        Ok(Self { initpt, space, gram, dual, is_ortho })
    }

    /// Public access to field `initpt`, returns `&Point`
    pub fn initpt(&self) -> &Point {
        &self.initpt
    }

    /// Public access to field `space`, returns `&Basis`
    pub fn space(&self) -> &Basis {
        &self.space
    }

    /// Public access to field `gram`, returns `&Matrix`
    pub fn gram(&self) -> &Matrix {
        &self.gram
    }

    /// Public access to field `dual`, returns `&Matrix`
    pub fn dual(&self) -> &Matrix {
        self.dual.as_ref().unwrap()
    }

    /// Public access to field `is_ortho`, returns `bool`
    pub fn is_ortho(&self) -> bool {
        self.is_ortho
    }

    /// Length of `Row` or `Col` in basis according to `gram` matrix
    pub fn len(&self, vec: &Matrix) -> ReRes<f64> {
        Ok(self.scalar_prod(vec, vec)?.sqrt())
    }

    /// Length of vector in the given index `i` in `Row`, `MultiRow`, `Col` or `MultiCol` in basis
    /// according to `gram` matrix
    pub fn len_at(&self, vec: &Matrix, i: usize) -> ReRes<f64> {
        Ok(self.scalar_prod_at(vec, i, vec, i)?.sqrt())
    }

    /// Resizing each vector to length 1 using scalar product in basis
    pub fn normalize(&self, vec: &mut Matrix) {
        match vec.repr() {
            Row | MultiRow => {
                for r in 0..vec.rows() {
                    let len = self.len_at(vec, r).unwrap();
                    for c in 0..vec.cols() {
                        let elem = *vec.att(r, c);
                        *vec.att_mut(r, c) = round(elem / len);
                    }
                }
            }
            Col | MultiCol => {

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
        Ok(*lhs.raw_scalar_prod(rhs, &self.gram)?.att(0, 0))
    }

    /// Scalar product in basis according to `self.gram()` matrix.
    /// Operands at the given indices must have the same dim. Produces `f64`
    pub fn scalar_prod_at(&self, lhs: &Matrix, l: usize, rhs: &Matrix, r: usize) -> ReRes<f64> {
        Ok(lhs.raw_scalar_prod_at(l, rhs, r, &self.gram)?)
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
        self.dual.as_ref().unwrap().combine(coef)
    }



    /// Decompose point in current basis
    pub fn decompose_pt(&self, pt: &Point) -> Matrix {
        pt.coord.mul_left(&self.space.basis.inv().expect("matrix of basis vector haven't inversed"))
    }
}

impl Default for CoordSys {
    fn default() -> Self {
        Self::new(Point::default(), Basis::default()).unwrap()
    }
}
