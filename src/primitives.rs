use {
    std::{
        ops::{Neg, Add, AddAssign, Sub, Mul, Div, BitXor},
        iter::zip,
    },
    crate::{EPSILON, SCREEN_HEIGHT, SCREEN_WIDTH, VECTOR_SPACE},
};


#[derive(Debug, Clone)]
pub struct Point {
    coord: [f64; 3],
}

impl From<[f64; 3]> for Point {
    fn from(coord: [f64; 3]) -> Self {
        Self { coord }
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for Point
where f64: From<T1> + From<T2> + From<T3>
{
    fn from((c1, c2, c3): (T1, T2, T3)) -> Self {
        Self {
            coord: [c1.into(), c2.into(), c3.into()]
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        for (lhs, rhs) in zip(self.coord, rhs.coord) {
            if (lhs - rhs).abs() > EPSILON {
                return false
            }
        }
        true
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..3 {
            self.coord[i] *= -1.0;
        }
        self
    }
}

impl Add for Point {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.coord[i] += rhs.coord[i]
        }
        self
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.coord[i] -= rhs.coord[i]
        }
        self
    }
}
impl Mul<f64> for Point {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in 0..3 {
            self.coord[i] *= rhs
        }
        self
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        for i in 0..3 {
            self.coord[i] /= rhs
        }
        self
    }
}

impl Point {
    pub const fn default() -> Self {
        Self { coord: [0.0f64; 3] }
    }

    pub fn distance(&self, pt: &Self) -> f64 {
         zip(self.coord, pt.coord)
             .map(|(self_c, pt_c)| (self_c - pt_c).powi(2))
             .sum::<f64>()
             .sqrt()
    }
}


#[derive(Debug, Default, Clone)]
pub struct Vector {
    dest: [f64; 3],
}

impl From<[f64; 3]> for Vector {
    fn from(dest: [f64; 3]) -> Self {
        Self { dest }
    }
}
impl<T1, T2, T3> From<(T1, T2, T3)> for Vector
where f64: From<T1> + From<T2> + From<T3>
{
    fn from((c1, c2, c3): (T1, T2, T3)) -> Self {
        Self {
            dest: [c1.into(), c2.into(), c3.into()]
        }
    }
}

impl From<Point> for Vector {
    fn from(pt: Point) -> Self {
        Self {
            dest: pt.coord
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Self) -> bool {
        for (lhs, rhs) in zip(self.dest, rhs.dest) {
            if (lhs - rhs).abs() > EPSILON {
                return false
            }
        }
        true
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..3 {
            self.dest[i] *= -1.0;
        }
        self
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.dest[i] += rhs.dest[i]
        }
        self
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.dest[i] += rhs.dest[i]
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.dest[i] -= rhs.dest[i]
        }
        self
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in 0..3 {
            self.dest[i] *= rhs
        }
        self
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        for i in 0..3 {
            self.dest[i] /= rhs
        }
        self
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        zip(self.dest, rhs.dest).map(|(a, b)| a * b).sum()
    }
}

#[inline(always)]
fn det2<T: Mul<Output=T> + Sub<Output=T> + Copy>(matrix: &[[T; 2]; 2]) -> T {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

impl BitXor for Vector {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        VECTOR_SPACE.decomposition_to_vector(
            &[
                det2(&[[self.dest[1], self.dest[2]], [rhs.dest[1], rhs.dest[2]]]),
               -det2(&[[self.dest[0], self.dest[2]], [rhs.dest[0], rhs.dest[2]]]),
                det2(&[[self.dest[0], self.dest[1]], [rhs.dest[0], rhs.dest[1]]]),
            ]
        )
    }
}

impl Vector {
    pub fn length(&self) -> f64 {
        VECTOR_SPACE.init_pt.distance(&Point { coord: self.dest.clone() })
    }
}


#[derive(Debug)]
pub struct VectorSpace {
    init_pt: Point,
    basis: [Vector; 3],
}

impl VectorSpace {
    pub const fn default() -> Self {
        VectorSpace {
            init_pt: Point::default(),
            basis: [Vector { dest: [1.0, 0.0, 0.0] },
                    Vector { dest: [0.0, 1.0, 0.0] },
                    Vector { dest: [0.0, 0.0, 1.0] }],
        }
    }

    pub fn decomposition_to_vector(&self, decomp: &[f64; 3]) -> Vector {
        let mut vector = Vector::default();
        for i in 0..3 {
            vector += self.basis[i].clone() * decomp[i];
        }
        vector
    }
}


#[derive(Debug, Clone)]
pub struct Camera {
    position: Point,
    look_dir: Vector,
    fov: u16,
    vfov: Option<u16>,
    draw_distance: u16,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: VECTOR_SPACE.init_pt.clone(),
            look_dir: VECTOR_SPACE.basis[0].clone(),
            fov: 60,
            vfov: None,
            draw_distance: 100,
        }
    }
}

impl Camera {
    pub fn new(position: Point, look_dir: Vector, fov: u16, draw_distance: u16) -> Self {
        Self {
            position,
            look_dir,
            fov,
            vfov: None,
            draw_distance,
        }
    }

    pub fn get_vfov(&mut self) -> u16 {
        if let Some(vfov) = self.vfov {
            vfov
        } else {
            self.vfov = Some(self.fov * SCREEN_HEIGHT / SCREEN_WIDTH);
            self.vfov.unwrap()
        }
    }

    pub fn send_rays(x_size: u16, y_size: u16) {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct Object {
    position: Point,
    dir: Vector,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            position: VECTOR_SPACE.init_pt.clone(),
            dir: VECTOR_SPACE.basis[0].clone(),
        }
    }
}

impl Object {
    pub fn contains(pt: &Point) -> bool {
        todo!()
    }
}
