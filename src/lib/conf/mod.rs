//! Module defines struct packages configuration information - `Conf`.
//! Constructor `Conf::read` reads provided configuration parameters from
//! the given TOML files, counting from the same level as `src` folder

#[cfg(test)]
mod test;

use {
    crate::{
        engn::*,
        errs::{
            GameErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        math::*,
    },
    std::{f32::consts::PI, fs::read_to_string},
    toml::{Table, Value},
};

const INITPT_KEY: &str = "INITIAL_POINT";
const ANGLE_DISCR_KEY: &str = "ROTATION_HALF_PI_DISCRETIZATION";
const WFOV_KEY: &str = "HORIZONTAL_FIELD_OF_VIEW_OUT_OF_PI";
const HFOV_KEY: &str = "VERTICAL_FIELD_OF_VIEW_OUT_OF_PI";
const DRAW_DIST_KEY: &str = "DRAW_DISTANCE";
const CHARMAP_KEY: &str = "CHARMAP";
const PRECISION_KEY: &str = "PRECISION";

/// Struct that packages configuration parameters,
/// it further is used for `Game` object instanciating
#[derive(Debug, Clone, PartialEq)]
pub struct Conf {
    pub initpt: Point,
    pub angle_discr: usize,
    pub wfov: f32,
    pub hfov: Option<f32>,
    pub draw_dist: f32,
    pub charmap: String,
    pub precision: u8,
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
                .parse_initpt(&mut table)?
                .parse_angle_discr(&mut table)?
                .parse_wfov(&mut table)?
                .parse_hfov(&mut table)?
                .parse_draw_dist(&mut table)?
                .parse_charmap(&mut table)?
                .parse_precision(&mut table)?;
        }
        Ok(conf)
    }

    pub fn parse_initpt(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(INITPT_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        self.initpt = Point::new(parse_single(value, INITPT_KEY)?);
        Ok(self)
    }

    pub fn parse_angle_discr(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(ANGLE_DISCR_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(val) => self.angle_discr = val as usize,
            Value::Float(val) => self.angle_discr = val as usize,
            _ => return Err(GameErr(InvalidConfValue(ANGLE_DISCR_KEY))),
        }
        Ok(self)
    }

    pub fn parse_wfov(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(WFOV_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(fov) => self.wfov = fov as f32,
            Value::Float(fov) => self.wfov = fov as f32,
            _ => return Err(GameErr(InvalidConfValue(WFOV_KEY))),
        }
        Ok(self)
    }

    pub fn parse_hfov(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(HFOV_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(fov) => self.hfov = Some(fov as f32),
            Value::Float(fov) => self.hfov = Some(fov as f32),
            _ => return Err(GameErr(InvalidConfValue(HFOV_KEY))),
        }
        Ok(self)
    }

    pub fn parse_draw_dist(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(DRAW_DIST_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(draw_dist) => self.draw_dist = draw_dist as f32,
            Value::Float(draw_dist) => self.draw_dist = draw_dist as f32,
            _ => return Err(GameErr(InvalidConfValue(DRAW_DIST_KEY))),
        }
        Ok(self)
    }

    pub fn parse_charmap(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(CHARMAP_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::String(charmap) => self.charmap = charmap,
            _ => return Err(GameErr(InvalidConfValue(CHARMAP_KEY))),
        }
        Ok(self)
    }

    /// Parses `PRECISION` parameter from the `Table` parsed from TOML
    pub fn parse_precision(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove(PRECISION_KEY) {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(precision) => self.precision = (precision % 256) as u8,
            _ => return Err(GameErr(InvalidConfValue(PRECISION_KEY))),
        }
        Ok(self)
    }
}

/// Parses `[f32; 3]` parameter from the `toml::Value::Array(toml::Array)`.
/// `key` that is the name of parameter is used for error messages
fn parse_single(value: Value, key: &'static str) -> ReRes<[f32; 3]> {
    let array = match value {
        Value::Array(array) => array,
        _ => return Err(GameErr(InvalidConfValue(key))),
    };
    let mut single = [0.0; 3];
    for (i, val) in array.iter().enumerate() {
        if i == 3 {
            return Err(GameErr(InvalidConfValue(key)));
        }
        single[i] = match val {
            Value::Integer(c) => *c as f32,
            Value::Float(c) => *c as f32,
            _ => return Err(GameErr(InvalidConfValue(key))),
        }
    }
    Ok(single)
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            initpt: Point::new([0.0; 3]),
            angle_discr: 6,
            wfov: PI / 2.0,
            hfov: None,
            draw_dist: 100.0,
            charmap: "$@&%#WMNB8RGAHP694XKYJOUVIL*+:-·".to_string(),
            precision: 100,
        }
    }
}
