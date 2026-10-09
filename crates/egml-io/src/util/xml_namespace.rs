use crate::util::XmlElement;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use strum::{Display, EnumIter, EnumString};

/// Trait for enums that represent XML namespaces (e.g. GML, CityGML).
///
/// `uri` is what identifies a namespace across vocabularies — it's what
/// [`XmlNamespaceContext`] keys prefix bindings by, so namespaces from
/// different `XmlNamespace` implementors (e.g. [`GmlNamespace`] and a future
/// `CitygmlNamespace`) can share a single context, just like real XML
/// namespaces share a single `xmlns:*` binding scope regardless of which
/// vocabulary defines each element.
pub trait XmlNamespace: Copy + Eq + Hash + Debug {
    /// This namespace's URI (e.g. `"http://www.opengis.net/gml/3.2"`).
    fn uri(&self) -> &'static str;

    /// The prefix used when [`XmlNamespaceContext`] has no binding for this
    /// namespace (e.g. `Some("gml")`), or `None` for the default (no-prefix)
    /// namespace.
    fn default_prefix(&self) -> Option<&'static str>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
pub enum GmlNamespace {
    Gml,
}

impl GmlNamespace {
    pub fn default_prefix(&self) -> Option<&'static str> {
        XmlNamespace::default_prefix(self)
    }
}

impl XmlNamespace for GmlNamespace {
    fn uri(&self) -> &'static str {
        match self {
            Self::Gml => "http://www.opengis.net/gml/3.2",
        }
    }

    fn default_prefix(&self) -> Option<&'static str> {
        match self {
            Self::Gml => Some("gml"),
        }
    }
}

/// Tracks the prefix each XML namespace is currently bound to, so
/// serialization isn't hardcoded to [`XmlNamespace::default_prefix`].
///
/// Bindings are keyed by namespace URI rather than by a specific
/// [`XmlNamespace`] implementor, so one context can serve namespaces from
/// multiple vocabularies (e.g. GML and CityGML) at once.
///
/// Starts out empty; [`prefix`](Self::prefix) falls back to
/// [`XmlNamespace::default_prefix`] ("" if that's also `None`) for any
/// namespace with no binding. Override individual bindings (e.g. after
/// reading a document's `xmlns:*` declarations) with [`set_prefix`](Self::set_prefix).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XmlNamespaceContext {
    prefixes: HashMap<&'static str, String>,
}

impl XmlNamespaceContext {
    pub fn new() -> Self {
        Self {
            prefixes: HashMap::new(),
        }
    }

    /// Returns the prefix currently bound to `namespace` ("" if it has
    /// none), falling back to [`XmlNamespace::default_prefix`] when no
    /// binding has been set.
    pub fn prefix<N: XmlNamespace>(&self, namespace: N) -> &str {
        match self.prefixes.get(namespace.uri()) {
            Some(prefix) => prefix.as_str(),
            None => namespace.default_prefix().unwrap_or_default(),
        }
    }

    /// Overrides the prefix bound to `namespace` (e.g. from a document's
    /// `xmlns:*` declarations). Pass `""` to bind it as the default
    /// namespace (no prefix).
    pub fn set_prefix<N: XmlNamespace>(&mut self, namespace: N, prefix: String) {
        self.prefixes.insert(namespace.uri(), prefix);
    }

    /// Returns the fully-qualified tag name for `element` in `namespace`,
    /// using the prefix currently bound in this context (e.g. `"gml:Polygon"`).
    pub fn qualify<N: XmlNamespace, E: XmlElement>(
        &self,
        namespace: N,
        element: E,
    ) -> Cow<'static, str> {
        match self.prefix(namespace) {
            "" => Cow::Borrowed(element.local_name()),
            prefix => Cow::Owned(format!("{prefix}:{}", element.local_name())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::GmlElement;

    #[test]
    fn qualify_uses_bound_prefix() {
        let mut context = XmlNamespaceContext::new();
        context.set_prefix(GmlNamespace::Gml, "gml3".to_string());

        assert_eq!(
            context.qualify(GmlNamespace::Gml, GmlElement::Point),
            "gml3:Point"
        );
    }

    #[test]
    fn qualify_falls_back_to_default_prefix_when_unbound() {
        let context = XmlNamespaceContext::new();

        assert_eq!(
            context.qualify(GmlNamespace::Gml, GmlElement::Point),
            "gml:Point"
        );
    }

    #[test]
    fn qualify_omits_prefix_when_bound_empty() {
        let mut context = XmlNamespaceContext::new();
        context.set_prefix(GmlNamespace::Gml, String::new());

        assert_eq!(
            context.qualify(GmlNamespace::Gml, GmlElement::Point),
            "Point"
        );
    }

    #[test]
    fn unbound_namespace_falls_back_to_default_prefix() {
        let context = XmlNamespaceContext::new();

        assert_eq!(context.prefix(GmlNamespace::Gml), "gml");
    }

    #[test]
    fn set_prefix_overrides_default() {
        let mut context = XmlNamespaceContext::new();
        context.set_prefix(GmlNamespace::Gml, "gml3".to_string());

        assert_eq!(context.prefix(GmlNamespace::Gml), "gml3");
    }

    #[test]
    fn set_prefix_empty_binds_as_default_namespace() {
        let mut context = XmlNamespaceContext::new();
        context.set_prefix(GmlNamespace::Gml, String::new());

        assert_eq!(context.prefix(GmlNamespace::Gml), "");
    }

    /// A minimal second vocabulary, standing in for `CitygmlNamespace`, to
    /// prove one context can hold bindings for multiple `XmlNamespace`
    /// implementors keyed by URI rather than by a single generic type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum OtherNamespace {
        Other,
    }

    impl XmlNamespace for OtherNamespace {
        fn uri(&self) -> &'static str {
            "urn:example:other"
        }

        fn default_prefix(&self) -> Option<&'static str> {
            Some("other")
        }
    }

    #[test]
    fn one_context_holds_bindings_for_multiple_namespace_vocabularies() {
        let mut context = XmlNamespaceContext::new();
        context.set_prefix(GmlNamespace::Gml, "gml3".to_string());
        context.set_prefix(OtherNamespace::Other, "oth".to_string());

        assert_eq!(context.prefix(GmlNamespace::Gml), "gml3");
        assert_eq!(context.prefix(OtherNamespace::Other), "oth");
    }
}
