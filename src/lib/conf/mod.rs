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
    std::{f64::consts::PI, fs::read_to_string},
    toml::{Table, Value},
};

/// Struct that packages configuration parameters,
/// it further is used for `Game` object instanciating
#[derive(Debug, Clone, PartialEq)]
pub struct Conf {
    pub initpt: Point,
    pub wfov: f64,
    pub hfov: Option<f64>,
    pub draw_dist: f64,
    pub wscr: usize,
    pub hscr: usize,
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
                .parse_wfov(&mut table)?
                .parse_hfov(&mut table)?
                .parse_draw_dist(&mut table)?
                .parse_wscr(&mut table)?
                .parse_hscr(&mut table)?
                .parse_precision(&mut table)?;
        }
        Ok(conf)
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

    /// Parses `HORIZONTAL_FIELD_OF_VIEW` parameter from the `Table` parsed from TOML
    pub fn parse_wfov(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("HORIZONTAL_FIELD_OF_VIEW") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(fov) => self.wfov = fov as f64,
            Value::Float(fov) => self.wfov = fov,
            _ => return Err(GameErr(InvalidConfValue("HORIZONTAL_FIELD_OF_VIEW"))),
        }
        Ok(self)
    }

    /// Parses `VERTICAL_FIELD_OF_VIEW` parameter from the `Table` parsed from TOML
    pub fn parse_hfov(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("VERTICAL_FIELD_OF_VIEW") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(fov) => self.hfov = Some(fov as f64),
            Value::Float(fov) => self.hfov = Some(fov),
            _ => return Err(GameErr(InvalidConfValue("VERTICAL_FIELD_OF_VIEW"))),
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

    /// Parses `SCREEN_WIDTH` parameter from the `Table` parsed from TOML
    pub fn parse_wscr(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("SCREEN_WIDTH") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(scr_x) => self.wscr = scr_x as usize,
            _ => return Err(GameErr(InvalidConfValue("SCREEN_WIDTH"))),
        }
        Ok(self)
    }

    /// Parses `SCREEN_HEIGHT` parameter from the `Table` parsed from TOML
    pub fn parse_hscr(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("SCREEN_HEIGHT") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(scr_y) => self.hscr = scr_y as usize,
            _ => return Err(GameErr(InvalidConfValue("SCREEN_HEIGHT"))),
        }
        Ok(self)
    }

    /// Parses `CHARMAP` parameter from the `Table` parsed from TOML
    pub fn parse_charmap(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("CHARMAP") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::String(charmap) => self.charmap = charmap,
            _ => return Err(GameErr(InvalidConfValue("CHARMAP"))),
        }
        Ok(self)
    }

    /// Parses `PRECISION` parameter from the `Table` parsed from TOML
    pub fn parse_precision(mut self, table: &mut Table) -> ReRes<Self> {
        let value = match table.remove("PRECISION") {
            Some(value) => value,
            None => return Ok(self),
        };
        match value {
            Value::Integer(precision) => self.precision = (precision % 256) as u8,
            _ => return Err(GameErr(InvalidConfValue("PRECISION"))),
        }
        Ok(self)
    }
}

/// Parses `Vec<f64>` parameter from the `toml::Value::Array(toml::Array)`.
/// `key` that is the name of parameter is used for error messages
fn parse_single(value: Value, key: &'static str) -> ReRes<Vec<f64>> {
    let array = match value {
        Value::Array(array) => array,
        _ => return Err(GameErr(InvalidConfValue(key))),
    };
    let mut single: Vec<f64> = vec![];
    for val in array {
        if single.len() == 3 {
            return Err(GameErr(InvalidConfValue(key)));
        }
        match val {
            Value::Integer(c) => single.push(c as f64),
            Value::Float(c) => single.push(c),
            _ => return Err(GameErr(InvalidConfValue(key))),
        }
    }
    Ok(single)
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            initpt: Point::new(vec![0.0; 3]),
            wfov: PI / 2.0,
            hfov: None,
            draw_dist: 100.0,
            wscr: 100,
            hscr: 60,
            charmap: ".:;><+r*zsvfwqkP694VOGbUAKXH8RD#$B0MNWQ%&@".to_string(),
            precision: 100,
        }
    }
}
