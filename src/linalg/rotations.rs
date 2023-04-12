/// Rotation matrices constructors.
#[allow(non_snake_case)]
use {
    crate::{
        linalg::matrixify::{Matrix, Matrixify},
    }
};

impl Matrix {
    /// Transformation matrix used to perform rotation through the given angle
    /// about the origin counterclockwise in a 2D right-handed coordinate system.
    pub fn rot(angle: f64) -> Matrix {
        let (c, s) = (angle.cos(), angle.sin());
        Matrix::from(vec![
            vec![c, -s],
            vec![s, c]
        ])
    }

    /// Transformation matrix used to perform rotation through the given angle
    /// about the origin clockwise in a Oyz plane.
    pub fn rot_x(angle: f64) -> Matrix {
        let (c, s) = (angle.cos(), angle.sin());
        Matrix::from(vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, c, -s],
            vec![0.0, s, c]
        ])
    }

    /// Transformation matrix used to perform rotation through the given angle
    /// about the origin clockwise in a Ozx plane.
    pub fn rot_y(angle: f64) -> Matrix {
        let (c, s) = (angle.cos(), angle.sin());
        Matrix::from(vec![
            vec![c, 0.0, s],
            vec![0.0, 1.0, 0.0],
            vec![-s, 0.0, c]
        ])
    }

    /// Transformation matrix used to perform rotation through the given angle
    /// about the origin clockwise in a Oxy plane.
    pub fn rot_z(angle: f64) -> Matrix {
        let (c, s) = (angle.cos(), angle.sin());
        Matrix::from(vec![
            vec![c, -s, 0.0],
            vec![s, c, 0.0],
            vec![0.0, 0.0, 1.0]
        ])
    }

    /// Sequentiol applying of rotations in Oyx, Ozx, Oxy planes.
    pub fn rot_xyz(x_angle: f64, y_angle: f64, z_angle: f64) -> Matrix {
        let (cx, sx) = (x_angle.cos(), x_angle.sin());
        let (cy, sy) = (y_angle.cos(), y_angle.sin());
        let (cz, sz) = (z_angle.cos(), z_angle.sin());
        Matrix::from(vec![
            vec![cy * cz, -cy * sz, sy],
            vec![sx * sy * cz + cx * sz, cx * cz - sx * sy * sz, -sx * cx],
            vec![sx * sz - cx * sy * cz, cx * sy * sz + sx * cz, cx * cx]
        ])
    }
}
