use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

/// A URI reference.
///
/// Corresponds to `xs:anyURI` in [XSD 1.1 Part 2 §3.3.17](https://www.w3.org/TR/xmlschema11-2/#anyURI).
/// XSD 1.1 accepts nearly any string as a URI reference, so this type does
/// not validate its value and stores it exactly as given.
///
/// Equality, ordering and hashing compare the stored string, so two URIs
/// that differ only in case or percent-encoding are not equal.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::xs::AnyUri;
///
/// let uri = AnyUri::new("https://www.opengis.net/def/crs/EPSG/0/25832");
/// assert_eq!(uri.as_str(), "https://www.opengis.net/def/crs/EPSG/0/25832");
/// assert_eq!(uri, "https://www.opengis.net/def/crs/EPSG/0/25832");
/// assert_eq!(uri, "https://www.opengis.net/def/crs/EPSG/0/25832".parse::<AnyUri>().unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AnyUri {
    value: Box<str>,
}

impl AnyUri {
    /// Creates a URI reference from its value.
    pub fn new(value: impl Into<Box<str>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the URI reference as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the URI reference and returns its value.
    pub fn into_inner(self) -> Box<str> {
        self.value
    }
}

impl fmt::Display for AnyUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl FromStr for AnyUri {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<String> for AnyUri {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for AnyUri {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<AnyUri> for String {
    fn from(value: AnyUri) -> Self {
        value.value.into()
    }
}

impl AsRef<str> for AnyUri {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl PartialEq<str> for AnyUri {
    fn eq(&self, other: &str) -> bool {
        *self.value == *other
    }
}

impl PartialEq<&str> for AnyUri {
    fn eq(&self, other: &&str) -> bool {
        *self.value == **other
    }
}

impl PartialEq<AnyUri> for str {
    fn eq(&self, other: &AnyUri) -> bool {
        *self == *other.value
    }
}

impl PartialEq<AnyUri> for &str {
    fn eq(&self, other: &AnyUri) -> bool {
        **self == *other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const URI: &str = "http://www.opengis.net/def/crs/EPSG/0/25832";

    #[test]
    fn test_new_stores_value_verbatim() {
        for value in [URI, "#UUID_abc", "urn:ogc:def:crs:EPSG::4326", " a  b ", ""] {
            assert_eq!(AnyUri::new(value).as_str(), value);
        }
    }

    #[test]
    fn test_display_and_from_str_agree() {
        let uri: AnyUri = URI.parse().unwrap();

        assert_eq!(uri.to_string(), URI);
        assert_eq!(uri.to_string().parse::<AnyUri>().unwrap(), uri);
    }

    #[test]
    fn test_conversions() {
        let uri = AnyUri::from(URI.to_owned());

        assert_eq!(uri, AnyUri::from(URI));
        assert_eq!(uri.as_ref(), URI);
        assert_eq!(String::from(uri.clone()), URI);
        assert_eq!(&*uri.into_inner(), URI);
    }

    #[test]
    fn test_equality_compares_exact_string() {
        assert_ne!(AnyUri::new("http://a.org/x"), AnyUri::new("HTTP://a.org/x"));
        assert_ne!(
            AnyUri::new("http://a.org/~x"),
            AnyUri::new("http://a.org/%7Ex")
        );

        let set: HashSet<AnyUri> = [URI, URI, "#a"].into_iter().map(AnyUri::new).collect();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_compares_with_str() {
        let uri = AnyUri::new(URI);

        assert_eq!(uri, URI);
        assert_eq!(URI, uri);
        assert_eq!(uri, *URI);
        assert_ne!(uri, "#other");
    }

    #[test]
    fn test_default_is_empty() {
        assert_eq!(AnyUri::default().as_str(), "");
    }
}
