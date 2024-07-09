use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EgmlError(#[from] egml_core::Error),
}
