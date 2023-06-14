use {
    std::{
        ops::{Mul, Index, IndexMut}
    },
};


#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    matrix: [[f32; 3]; 3],
}

impl Matrix {
    pub fn rotation(mut from: usize, mut to: usize, mut angle: f32) -> Self {
        // TODO: remove when it works
        assert_ne!(from, to);
        if from > to {
            (from, to) = (to, from);
            angle = -angle;
        }
        let (s, c) = (angle.sin(), angle.cos());
        let mut matrix = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        matrix[from][from] = c;
        matrix[from][to] = -s;
        matrix[to][from] = s;
        matrix[to][to] = c;

        Self { matrix }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub(crate) coord: [f32; 3],
}

impl Vector {
    pub fn new(coord: [f32; 3]) -> Self {
        Self { coord }
    }

    pub fn scalar_prod(&self, rhs: &Self) -> f32 {
        (0..3).map(|i| self[i] * rhs[i]).sum()
    }

    pub fn len(&self) -> f32 {
        self.scalar_prod(self).sqrt()
    }

    pub fn resize(&mut self, coef: f32) {
        for i in 0..3 {
            self[i] *= coef
        }
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector {
            coord: [
                (0..3).map(|i| self.matrix[0][i] * rhs[i]).sum(),
                (0..3).map(|i| self.matrix[1][i] * rhs[i]).sum(),
                (0..3).map(|i| self.matrix[2][i] * rhs[i]).sum()
            ]
        }
    }
}

impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coord[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coord[index]
    }
}


pub type Point = Vector;

impl Point {
    pub fn mv(&mut self, vector: &Vector) {
        for i in 0..3 {
            self[i] += vector[i]
        }
    }

    pub fn df(&self, other: &Self) -> Vector {
        Vector {
            coord: [
                self[0] - other[0],
                self[1] - other[1],
                self[2] - other[2]
            ]
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new([0.0; 3])
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Basis {
    pub(crate) elems: [Vector; 3],
    pub(crate) inv: Option<Matrix>,
}

impl Basis {
    pub fn new() -> Self {
        Self {
            elems: [
                Vector::new([1.0, 0.0, 0.0]),
                Vector::new([0.0, 1.0, 0.0]),
                Vector::new([0.0, 0.0, 1.0])
            ],
            inv: Some(Matrix {
                matrix: [
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]
                ]
            }),
        }
    }

    pub fn inv(&mut self) {
        let e = &self.elems;
        self.inv = Some(Matrix {
            matrix: [
                [e[1][1] * e[2][2] - e[2][1] * e[1][2], e[2][1] * e[0][2] - e[0][1] * e[2][2], e[0][1] * e[1][2] - e[1][1] * e[0][2]],
                [e[2][0] * e[1][2] - e[1][0] * e[2][2], e[0][0] * e[2][2] - e[2][0] * e[0][2], e[1][0] * e[0][2] - e[0][0] * e[1][2]],
                [e[1][0] * e[2][1] - e[2][0] * e[1][1], e[2][0] * e[0][1] - e[0][0] * e[2][1], e[0][0] * e[1][1] - e[1][0] * e[0][1]],
            ]
        })
    }

    pub fn decompose(&self, pt: &Point) -> Vector {
        self.inv.as_ref().unwrap() * pt
    }
}

impl Mul<&Basis> for &Matrix {
    type Output = Basis;

    fn mul(self, rhs: &Basis) -> Self::Output {
        Basis {
            elems: [
                self * &rhs[0],
                self * &rhs[1],
                self * &rhs[2],
            ],
            inv: None,
        }
    }
}

impl Index<usize> for Basis {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Basis {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}
