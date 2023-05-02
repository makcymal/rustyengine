/// Errors that can replace Matrixify inner;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrErr {
    MultidimRow,
    MultidimCol,
    ErrByDesign,
    CrookedSquare,
    CurveSides,
    EmptyAtAll,
}

/// Result can be obtained from some Matrixify methods;
#[derive(Debug, Clone, PartialEq)]
pub enum MatrRes<T> {
    Go(T),
    SoloOutOfBounds,
    DuetOutOfBounds,
    TreatSoloAsDuet,
    TreatDuetAsSolo,
    Untransposable,
    UnhandledMatrErr,
}

impl<T> MatrRes<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Go(t) => t,
            _ => panic!("Unwrap on errored MatrRes"),
        }
    }
}
