use thiserror::Error;

#[derive(Error, Debug)]
pub enum GmlIoError {
    #[error(transparent)]
    GmlError(#[from] egml_core::error::Error),
    #[error(transparent)]
    Io(#[from] quick_xml::DeError),

    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
}
