use crate::util::XmlElement;
use strum::Display;

/// Element vocabulary for the OASIS xAL 2.0 schema, as used for `xAL:Address`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum XalElement {
    Address,
    Country,
    Locality,
    NameElement,
    Number,
    Thoroughfare,
}

impl XmlElement for XalElement {
    fn from_local_name(local_name: &str) -> Option<Self> {
        match local_name {
            "Address" => Some(Self::Address),
            "Country" => Some(Self::Country),
            "Locality" => Some(Self::Locality),
            "NameElement" => Some(Self::NameElement),
            "Number" => Some(Self::Number),
            "Thoroughfare" => Some(Self::Thoroughfare),
            _ => {
                tracing::debug!("unknown XML element: {local_name}");
                None
            }
        }
    }

    fn local_name(&self) -> &'static str {
        match self {
            Self::Address => "Address",
            Self::Country => "Country",
            Self::Locality => "Locality",
            Self::NameElement => "NameElement",
            Self::Number => "Number",
            Self::Thoroughfare => "Thoroughfare",
        }
    }
}
