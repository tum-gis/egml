use thiserror::Error;

/// Errors returned by `egml-io` parsing operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Wraps a core geometry validation error.
    #[error(transparent)]
    Core(#[from] egml_core::Error),

    /// Wraps a low-level XML syntax error from `quick-xml`.
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),

    /// Wraps a serde deserialization error from `quick-xml`.
    #[error(transparent)]
    XmlDe(#[from] quick_xml::DeError),

    /// Wraps a serde serialization error from `quick-xml`.
    #[error(transparent)]
    XmlSe(#[from] quick_xml::SeError),

    /// A required XML element with the given name was not found in the parsed GML fragment.
    ///
    /// The inner string names the missing element (e.g. `"gml:exterior"`,
    /// `"gml:posList"`).
    #[error("required GML element '{0}' was not found in the XML input")]
    ElementNotFound(String),

    /// The GML input contains 2-D coordinates (`srsDimension=\"2\"`), but only
    /// 3-D geometries (`srsDimension=\"3\"`) are supported by this parser.
    #[error(
        "only 3-D coordinates (srsDimension=\"3\") are supported; 2-D GML input is not accepted"
    )]
    UnsupportedDimension(),

    /// One or more required child elements are absent from the GML fragment.
    ///
    /// The inner string names the parent element or context where children are
    /// missing (e.g. `"gml:Polygon"`, `"gml:Surface"`).
    #[error("required child elements are absent from '{0}'")]
    MissingElements(String),

    /// A `gml:LinearRing` element was expected but not found.
    #[error("missing linear ring")]
    MissingLinearRing(),

    /// A surface kind discriminator was expected but not found.
    ///
    /// The inner string names the parent element.
    #[error("missing surface kind for `{0}`")]
    MissingSurfaceKind(String),

    /// The GML input uses XLink references (`xlink:href`), which are not yet resolved.
    #[error("XLinks are not supported yet")]
    UnsupportedXLink(),
}
