use {
    std::{
        ops::{Neg, Add, Sub, Mul, Div},
        iter::zip,
    },
    super::EPSILON
};

#[derive(Debug, Default)]
struct Trio {
    trio: [f64; 3],
}

impl PartialEq for Trio {
    fn eq(&self, other: &Self) -> bool {
        for (lhs, rhs) in zip(self.trio, other.trio) {
            if (lhs - rhs).abs() > EPSILON {
                return false
            }
        }
        true
    }
}
impl From<[f64; 3]> for Trio {
    fn from(trio: [f64; 3]) -> Self {
        Self { trio }
    }
}
impl Neg for Trio {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for (i, num) in self.trio.map(|num| -num).iter().enumerate() {
            self.trio[i] = *num;
        }
        self
    }
}
impl Add for Trio {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for (i, val) in zip(self.trio.iter(), rhs.trio.iter()).map(|(a, b)| a+b).enumerate() {
            res.trio[i] = val;
        }
        res
    }
}
impl Sub for Trio {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
impl Mul<f64> for Trio {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for (i, num) in self.trio.map(|num| num * rhs).iter().enumerate() {
            self.trio[i] = *num;
        }
        self
    }
}
impl Div<f64> for Trio {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}


#[derive(Debug, Default, PartialEq)]
pub struct Point {
    coord: Trio,
}

impl From<[f64; 3]> for Point {
    fn from(trio: [f64; 3]) -> Self {
        Self { coord: Trio::from(trio) }
    }
}
impl From<Trio> for Point {
    fn from(coord: Trio) -> Self {
        Self { coord }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.coord + rhs.coord)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.coord - rhs.coord)
    }
}
impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.coord * rhs)
    }
}
impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from(self.coord / rhs)
    }
}

#[derive(Debug, Default)]
pub struct Vector {
    dest: Trio,
}

impl From<[f64; 3]> for Vector {
    fn from(trio: [f64; 3]) -> Self {
        Self { dest: Trio::from(trio) }
    }
}
impl From<Trio> for Vector {
    fn from(dest: Trio) -> Self {
        Self { dest }
    }
}
impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.dest + rhs.dest)
    }
}
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.dest - rhs.dest)
    }
}
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.dest * rhs)
    }
}
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from(self.dest / rhs)
    }
}
