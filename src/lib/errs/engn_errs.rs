use thiserror::Error;

/// Errors that can be obtained within `Result::Err::EngnErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum GameErr {
    #[error("requasted property of GameObject isn't initialized yet")]
    NotInitializedProp,

    #[error("trying to set property {key:?} to value {val:?}")]
    InvalidPropF64 { key: &'static str, val: f64 },

    #[error("failed to read file {0}")]
    InvalidConfFilePath(&'static str),

    #[error("failed to parse TOML from {0}")]
    InvalidConfFileContent(&'static str),

    #[error("failed to parse value with key {0}")]
    InvalidConfValue(&'static str),
}
