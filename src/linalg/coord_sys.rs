use {
    crate::{
        globals::{
            DIM, BIFORM, COORDSYS,
        },
        utils::Size,
        linalg::matrixified::{
            Matrixified, Matrix, Vector,
        },
    },
    std::ops::{
        Add, Sub, Index, IndexMut,
    },
    once_cell::sync::OnceCell,
};


static GRAMM: OnceCell<Matrix> = OnceCell::new();


// <<< Vecspace

#[derive(Debug)]
pub struct Vecspace {
    pub basis: [Vector; DIM],
    // false if it's unknown that basis vectors are orthogonal pairwise
    surely_is_ortho: bool,
}

impl Vecspace {
    pub fn identity() -> Self {
        let mut basis: [Vector; DIM] = Default::default();
        for i in 0..DIM {
            basis[i] = Vector::from(vec![0.0; DIM]);
            basis[i][i] = 1.0;
        }
        Self { basis, surely_is_ortho: true }
    }

    fn gramm(&self) -> &'static Matrix {
        if GRAMM.get().is_none() {
            let mut gramm = Matrix::zeros(Size::Rect((DIM, DIM)));
            for row in 0..DIM {
                for col in 0..DIM {
                    gramm[(row, col)] = &self.basis[row] % &self.basis[col];
                }
            }
            GRAMM.set(gramm).expect("GRAMM initialization failed");
        }

        GRAMM.get().expect("GRAMM is not initialized")
    }
}

impl From<[Vector; DIM]> for Vecspace {
    fn from(basis: [Vector; DIM]) -> Self {
        Self { basis, surely_is_ortho: false }
    }
}

impl Index<usize> for Vecspace {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.basis[index])
    }
}

impl PartialEq for Vecspace {
    fn eq(&self, other: &Self) -> bool {
        self.basis == other.basis
    }
}

// Vecspace >>>


// <<< Point

#[derive(Debug, PartialEq)]
pub struct Point {
    radvec: Vector,
}

impl Point {
    pub fn zeros() -> Self {
        Self::from(Vector::zeros(Size::Row(DIM)))
    }

    // only in orthogonal basis
    pub fn as_vector(&self) -> Vector {
        if !CoordSys::global().surely_is_ortho() {
            panic!("Basis may be not orthogonal");
        }

        let vs: &Vecspace = CoordSys::vecspace();
        let mut inner = vec![];
        for i in 0..DIM {
            inner.push((&self.radvec ^ &vs[i]) / vs[i].length())
        }
        Vector::from(inner)
    }
}

impl From<Vector> for Point {
    fn from(radvec: Vector) -> Self {
        Self { radvec }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec + rhs)
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec - rhs)
    }
}

// Point >>>


// <<< CoordSys

#[derive(Debug, PartialEq)]
pub struct CoordSys {
    init_pt: Point,
    vecspace: Vecspace,
}

impl CoordSys {
    pub fn from(init_pt: Point, vecspace: Vecspace) -> Self {
        Self { init_pt, vecspace }
    }

    pub fn surely_is_ortho(&self) -> bool {
        self.vecspace.surely_is_ortho
    }

    pub fn global() -> &'static CoordSys {
        COORDSYS.get().expect("COORDSYS is not initialized")
    }

    pub fn vecspace() -> &'static Vecspace {
        &CoordSys::global().vecspace
    }

    pub fn gramm() -> &'static Matrix {
        CoordSys::global().vecspace.gramm()
    }
}

// CoordSys >>>
