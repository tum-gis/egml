use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the data for key `{0}` is not available")]
    ValueNotFinite(&'static str),
    #[error("the data for key `{0}` is not available")]
    NotEnoughElements(&'static str),
    #[error("the data for key `{0}` is not available")]
    MustNotBeEmpty(&'static str),
}
