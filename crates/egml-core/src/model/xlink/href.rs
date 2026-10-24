use crate::model::base::Id;
use crate::model::xs::AnyUri;

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
/// use egml_core::model::xs::AnyUri;
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
/// assert_eq!(remote, HRef::Remote(AnyUri::new("https://example.com/geom")));
/// assert_eq!(remote.local_id(), None);
/// assert_eq!(remote.remote_uri().unwrap(), "https://example.com/geom");
/// assert_eq!(remote.to_string(), "https://example.com/geom");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HRef {
    /// A fragment reference to an object in the same document.
    /// Stores the bare ID without the `#` prefix.
    Local(String),
    /// A full URI reference to an external resource.
    Remote(AnyUri),
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
    pub fn from_remote(uri: impl Into<AnyUri>) -> Self {
        HRef::Remote(uri.into())
    }

    /// Returns the bare GML `gml:id` if this is a local fragment reference.
    pub fn local_id(&self) -> Option<&str> {
        match self {
            HRef::Local(id) => Some(id.as_str()),
            HRef::Remote(_) => None,
        }
    }

    /// Returns the URI if this is a remote reference.
    pub fn remote_uri(&self) -> Option<&AnyUri> {
        match self {
            HRef::Local(_) => None,
            HRef::Remote(uri) => Some(uri),
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
            HRef::Remote(AnyUri::from(s))
        }
    }
}

impl From<AnyUri> for HRef {
    /// Treats a value starting with `#` as a local reference, anything else as remote.
    fn from(uri: AnyUri) -> Self {
        match uri.as_str().strip_prefix('#') {
            Some(id) => HRef::Local(id.to_owned()),
            None => HRef::Remote(uri),
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
            HRef::Remote(uri) => f.write_str(uri.as_str()),
        }
    }
}

impl std::str::FromStr for HRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HRef::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::xs::AnyUri;

    const URI: &str = "https://example.com/feature/1";

    #[test]
    fn from_remote_accepts_strings_and_any_uri() {
        let expected = HRef::Remote(AnyUri::new(URI));

        assert_eq!(HRef::from_remote(URI), expected);
        assert_eq!(HRef::from_remote(URI.to_owned()), expected);
        assert_eq!(HRef::from_remote(AnyUri::new(URI)), expected);
    }

    #[test]
    fn remote_uri_is_only_set_for_remote_references() {
        assert_eq!(HRef::from_remote(URI).remote_uri(), Some(&AnyUri::new(URI)));
        assert_eq!(HRef::from_local("#abc").remote_uri(), None);
        assert_eq!(HRef::from_remote(URI).local_id(), None);
        assert!(!HRef::from_remote(URI).is_local());
    }

    #[test]
    fn from_any_uri_splits_on_fragment_prefix() {
        assert_eq!(HRef::from(AnyUri::new("#abc")), HRef::from_local("abc"));
        assert_eq!(HRef::from(AnyUri::new(URI)), HRef::from_remote(URI));
        // A fragment on a remote document is still a remote reference.
        assert_eq!(
            HRef::from(AnyUri::new("other.gml#abc")),
            HRef::from_remote("other.gml#abc")
        );
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for value in ["#abc", URI] {
            let href: HRef = value.parse().unwrap();
            assert_eq!(href.to_string(), value);
        }
    }
}
