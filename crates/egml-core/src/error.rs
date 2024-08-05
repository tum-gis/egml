use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Error {
    #[error("Value not finite `{0}`")]
    ValueNotFinite(&'static str),
    #[error("Not enough elements: `{0}`")]
    NotEnoughElements(&'static str),
    #[error("Invalid number of elements: `{0}`")]
    InvalidNumberOfElements(&'static str),
    #[error("Must not be empty: `{0}`")]
    MustNotBeEmpty(&'static str),
    #[error("Contains equal elements")]
    ContainsEqualElements,
    #[error("Contains duplicate elements")]
    ContainsDuplicateElements,
    #[error("Contains equal first and last element")]
    ContainsEqualStartAndLastElement,

    #[error("Lower corner must be below upper corner: `{0}`")]
    LowerCornerMustBeEqualOrBelowUpperCorner(&'static str),
}
