use crate::model::base::Id;

/// An `xlink:href` reference to a GML object.
///
/// Distinguishes between local fragment references (same document) and remote
/// URI references at the type level, keeping the `#` prefix as a
/// serialization-only detail.
///
/// # Examples
///
/// ```
/// use egml_core::model::xlink::HRef;
///
/// // Both forms produce the same local reference
/// assert_eq!(HRef::from_local("#UUID_abc_def"), HRef::from_local("UUID_abc_def"));
///
/// let local: HRef = "#UUID_abc_def".parse().unwrap();
/// assert_eq!(local, HRef::from_local("UUID_abc_def"));
/// assert_eq!(local.local_id(), Some("UUID_abc_def"));
/// assert_eq!(local.to_string(), "#UUID_abc_def");
///
/// let remote: HRef = "https://example.com/geom".parse().unwrap();
/// assert_eq!(remote, HRef::Remote("https://example.com/geom".to_string()));
/// assert_eq!(remote.local_id(), None);
/// assert_eq!(remote.to_string(), "https://example.com/geom");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HRef {
    /// A fragment reference to an object in the same document.
    /// Stores the bare ID without the `#` prefix.
    Local(String),
    /// A full URI reference to an external resource.
    Remote(String),
}

impl HRef {
    /// Constructs a local fragment reference, stripping a leading `#` if present.
    ///
    /// Accepts both raw `xlink:href` attribute values (`"#UUID_abc"`) and bare
    /// GML ids (`"UUID_abc"`). The `#` prefix is added back on serialization.
    pub fn from_local(id: impl Into<String>) -> Self {
        let s = id.into();
        let bare = s.strip_prefix('#').unwrap_or(&s).to_owned();
        HRef::Local(bare)
    }

    /// Constructs a local fragment reference from a GML [`Id`].
    pub fn from_local_id(id: Id) -> Self {
        HRef::Local(id.into())
    }

    /// Constructs a remote URI reference.
    pub fn from_remote(uri: impl Into<String>) -> Self {
        HRef::Remote(uri.into())
    }

    /// Returns the bare GML `gml:id` if this is a local fragment reference.
    pub fn local_id(&self) -> Option<&str> {
        match self {
            HRef::Local(id) => Some(id.as_str()),
            HRef::Remote(_) => None,
        }
    }

    /// Returns `true` if this is a local fragment reference.
    pub fn is_local(&self) -> bool {
        matches!(self, HRef::Local(_))
    }
}

impl From<String> for HRef {
    fn from(s: String) -> Self {
        if let Some(id) = s.strip_prefix('#') {
            HRef::Local(id.to_owned())
        } else {
            HRef::Remote(s)
        }
    }
}

impl From<&str> for HRef {
    fn from(s: &str) -> Self {
        HRef::from(s.to_owned())
    }
}

impl std::fmt::Display for HRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HRef::Local(id) => write!(f, "#{id}"),
            HRef::Remote(uri) => f.write_str(uri),
        }
    }
}

impl std::str::FromStr for HRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HRef::from(s))
    }
}
