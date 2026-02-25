use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GmlError(#[from] egml_core::Error),
    #[error(transparent)]
    QuickXmlError(#[from] quick_xml::Error),
    #[error(transparent)]
    QuickXmlDeError(#[from] quick_xml::DeError),

    #[error("the data for key `{0}` is not available")]
    ElementNotFound(String),
    #[error("the data for key is not available")]
    Only3DSupported(),
    #[error("the data for key is not available")]
    MissingElements(String),
    #[error("missing linear ring")]
    MissingLinearRing(),
    #[error("missing surface kind for `{0}`")]
    MissingSurfaceKind(String),
    #[error("XLinks are not supported yet")]
    XLinksNotSupported(),
}
