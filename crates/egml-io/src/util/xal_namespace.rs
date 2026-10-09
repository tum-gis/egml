use crate::util::XmlNamespace;
use strum::{Display, EnumIter, EnumString};

/// XML namespace for the OASIS Extensible Address Language (xAL) 2.0 schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
pub enum XalNamespace {
    Xal,
}

impl XalNamespace {
    pub fn default_prefix(&self) -> Option<&'static str> {
        XmlNamespace::default_prefix(self)
    }
}

impl XmlNamespace for XalNamespace {
    fn uri(&self) -> &'static str {
        match self {
            Self::Xal => "urn:oasis:names:tc:ciq:xsdschema:xAL:2.0",
        }
    }

    fn default_prefix(&self) -> Option<&'static str> {
        match self {
            Self::Xal => Some("xAL"),
        }
    }
}
