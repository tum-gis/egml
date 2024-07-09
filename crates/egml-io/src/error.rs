use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GmlError(#[from] egml_core::Error),
    #[error(transparent)]
    Io(#[from] quick_xml::DeError),

    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
    #[error("the data for key is not available")]
    Only3DSupported(),
    #[error("the data for key is not available")]
    MissingElements(),
}
