use thiserror::Error;

/// Errors returned by `egml-transform` operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Wraps a core geometry validation error.
    #[error(transparent)]
    EgmlError(#[from] egml_core::Error),
}
