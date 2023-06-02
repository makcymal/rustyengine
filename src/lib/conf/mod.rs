//! Module defines struct packages configuration information - `Conf`.
//! Constructor `Conf::read` reads provided configuration parameters from
//! the given TOML files, counting from the same level as `src` folder

#[cfg(test)]
mod test;

use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GameErr::{self, *},
        },
        engn::*,
        math::*,
    },
    std::{
        f64::consts::PI,
        fs::read_to_string,
    },
    toml::{
        Table,
        Value,
    },
};


/// Struct that packages configuration parameters,
/// it further is used for `Game` object instanciating
#[derive(Debug, Clone, PartialEq)]
pub struct Conf {
    pub biform: Matrix,
    pub basis: Matrix,
    pub initpt: Point,
    pub camera_dir: Vector,
    pub camera_lookat: Option<Point>,
    pub camera_fov: f64,
    pub draw_dist: f64,
    pub scr_height: usize,
    pub scr_width: usize,
}

impl Conf {
    /// Reads parameters from the given TOML files.
    /// Each encountered in TOML parameter will bew reassigned if it has been already encountered
    pub fn read(filepaths: Vec<&'static str>) -> ReRes<Self> {
        let mut conf = Self::default();
        for path in filepaths {
            let content = match read_to_string(path) {
                Ok(cont) => cont,
                Err(_) => return Err(GameErr(InvalidConfFilePath(path))),
            };
            let mut table = match content.parse::<Table>() {
                Ok(table) => table,
                Err(_) => return Err(GameErr(InvalidConfFileContent(path))),
            };
            conf = conf
                .parse_biform(&mut table)?
                .parse_basis(&mut table)?
                .parse_initpt(&mut table)?
                .parse_camera_dir(&mut table)?
                .parse_camera_lookat(&mut table)?
                .parse_camera_fov(&mut table)?
                .parse_draw_dist(&mut table)?
                .parse_scr_height(&mut table)?
                .parse_scr_width(&mut table)?;
        }
        Ok(conf)
    }

    /// Parses `BILINEAR_FORM` parameter from the `Table` parsed from TOML
    pub fn parse_biform(mut self, table: &mut Table) -> ReRes<Self> {
        let key = "BILINEAR_FORM";
        let value = match table.remove(key) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.biform = Matrix::from_double(parse_double(value, key)?).to_square();
        Ok(self)
    }

    /// Parses `BASIS` parameter from the `Table` parsed from TOML
    pub fn parse_basis(mut self, table: &mut Table) -> ReRes<Self> {
        let key = "BASIS";
        let value = match table.remove(key) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.basis = Matrix::from_double(parse_double(value, key)?).to_multicol();
        Ok(self)
    }

    /// Parses `INITIAL_POINT` parameter from the `Table` parsed from TOML
    pub fn parse_initpt(mut self, table: &mut Table) -> ReRes<Self> {
        let key = "INITIAL_POINT";
        let value = match table.remove(key) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.initpt = Point::new(parse_single(value, key)?);
        Ok(self)
    }

    /// Parses `CAMERA_DIRECTION` parameter from the `Table` parsed from TOML
    pub fn parse_camera_dir(mut self, table: &mut Table) -> ReRes<Self> {
        let key = "CAMERA_DIRECTION";
        let value = match table.remove(key) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.camera_dir = Vector::col(parse_single(value, key)?);
        Ok(self)
    }

    /// Parses `CAMERA_LOOKS_AT` parameter from the `Table` parsed from TOML
    pub fn parse_camera_lookat(mut self, table: &mut Table) -> ReRes<Self> {
        let key = "CAMERA_LOOKS_AT";
        let value = match table.remove(key) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.camera_lookat = Some(Point::new(parse_single(value, key)?));
        Ok(self)
    }

    /// Parses `FIELD_OF_VIEW` parameter from the `Table` parsed from TOML
    pub fn parse_camera_fov(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("FIELD_OF_VIEW") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(fov) => self.camera_fov = fov as f64,
            Value::Float(fov) => self.camera_fov = fov,
            _ => return Err(GameErr(InvalidConfValue("FIELD_OF_VIEW"))),
        }
        Ok(self)
    }

    /// Parses `DRAW_DISTANCE` parameter from the `Table` parsed from TOML
    pub fn parse_draw_dist(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("DRAW_DISTANCE") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(draw_dist) => self.draw_dist = draw_dist as f64,
            Value::Float(draw_dist) => self.draw_dist = draw_dist,
            _ => return Err(GameErr(InvalidConfValue("DRAW_DISTANCE"))),
        }
        Ok(self)
    }

    /// Parses `SCREEN_HEIGHT` parameter from the `Table` parsed from TOML
    pub fn parse_scr_height(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("SCREEN_HEIGHT") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(scr_height) => self.scr_height = scr_height as usize,
            _ => return Err(GameErr(InvalidConfValue("SCREEN_HEIGHT"))),
        }
        Ok(self)
    }

    /// Parses `SCREEN_WIDTH` parameter from the `Table` parsed from TOML
    pub fn parse_scr_width(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("SCREEN_WIDTH") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(scr_width) => self.scr_width = scr_width as usize,
            _ => return Err(GameErr(InvalidConfValue("SCREEN_WIDTH"))),
        }
        Ok(self)
    }
}

/// Parses `Vec<f64>` parameter from the `toml::Value::Array(toml::Array)`.
/// `key` that is the name of parameter is used for error messages
fn parse_single(value: Value, key: &'static str) -> ReRes<Vec<f64>> {
    let array = match value {
        Value::Array(array) => array,
        _ => return Err(GameErr(InvalidConfValue(key)))
    };
    let mut single: Vec<f64> = vec!();
    for val in array {
        if single.len() == 3 {
            return Err(GameErr(InvalidConfValue(key)));
        }
        match val {
            Value::Integer(c) => single.push(c as f64),
            Value::Float(c) => single.push(c),
            _ => return Err(GameErr(InvalidConfValue(key)))
        }
    }
    Ok(single)
}

/// Parses `Vec<Vec<f64>>` parameter from the `toml::Value::Array(toml::Array)`.
/// `key` that is the name of parameter is used for error messages
fn parse_double(value: Value, key: &'static str) -> ReRes<Vec<Vec<f64>>> {
    let array = match value {
        Value::Array(array) => array,
        _ => return Err(GameErr(InvalidConfValue(key)))
    };
    let mut double: Vec<Vec<f64>> = vec!();
    for val in array {
        if double.len() == 3 {
            return Err(GameErr(InvalidConfValue(key)));
        }
        double.push(parse_single(val, key)?);
    }
    Ok(double)
}


impl Default for Conf {
    fn default() -> Self {
        let mut camera_dir = Vector::col(vec![0.0; 3]);
        *camera_dir.at_mut(0) = 1.0;
        Self {
            biform: Matrix::identity(3),
            basis: Matrix::identity(3).to_multicol(),
            initpt: Point::new(vec![0.0; 3]),
            camera_dir,
            camera_lookat: None,
            camera_fov: PI / 2.0,
            draw_dist: 100.0,
            scr_height: 3,
            scr_width: 3,
        }
    }
}
