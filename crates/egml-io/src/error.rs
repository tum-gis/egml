use thiserror::Error;

/// Errors returned by `egml-io` parsing operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Wraps a core geometry validation error.
    #[error(transparent)]
    EgmlError(#[from] egml_core::Error),

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

    /// The GML input uses a coordinate dimension other than 3.
    ///
    /// `found` is the `srsDimension` value from the input; only 3-D geometries
    /// (`srsDimension="3"`) are supported.
    #[error("only 3-D coordinates (srsDimension=\"3\") are supported; found srsDimension={found}")]
    UnsupportedDimension { found: u32 },

    /// A `gml:posList` contains a number of values that is not a multiple of 3.
    ///
    /// `count` is the actual number of values encountered.
    #[error(
        "coordinate list has {count} value(s), which is not a multiple of 3; \
         3-D coordinates require groups of exactly 3 values (x, y, z)"
    )]
    InvalidCoordinateCount { count: usize },

    /// One or more required child elements are absent from the GML fragment.
    ///
    /// The inner string names the parent element or context where children are
    /// missing (e.g. `"gml:Polygon"`, `"gml:Surface"`).
    #[error("required child elements are absent from '{0}'")]
    MissingElements(String),

    /// A `gml:LinearRing` element was expected but not found.
    #[error("missing gml:LinearRing element")]
    MissingLinearRing,

    /// A surface kind discriminator was expected but not found.
    ///
    /// The inner string names the parent element.
    #[error("missing surface kind for '{0}'")]
    MissingSurfaceKind(String),

    /// The GML input uses XLink references (`xlink:href`), which are not yet resolved.
    #[error("XLinks are not supported yet")]
    UnsupportedXLink,
}
