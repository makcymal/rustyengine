use {
    thiserror::Error,
    crate::engn::Prop,
};


/// Errors that can be obtained within `Result::Err::EngnErr`
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum EngnErr {
    #[error("requasted property of GameObject isn't initialized yet")]
    NotInitializedProp,

    #[error("trying to set property {key:?} to value {val:?}")]
    InvalidPropF64{
        key: Prop,
        val: f64
    },
}
