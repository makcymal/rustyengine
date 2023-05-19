use {
    thiserror::Error,
};


/// It's not error, moreover it means computations succeeded yet without valuable result
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum NoneOpt {
    #[error("there is no intersection point")]
    NoIntersection,

    #[error("intersection point is so far away")]
    FarThenDrawDist,
}