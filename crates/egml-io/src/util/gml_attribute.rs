use crate::Error;
use crate::util::XmlElement;
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GmlAttribute {
    Id,
    SrsDimension,
    SrsName,
}

impl XmlElement for GmlAttribute {
    fn from_local_name(local_name: &str) -> Option<Self> {
        match local_name {
            "id" => Some(Self::Id),
            _ => {
                tracing::debug!("unknown XML element: {local_name}");
                None
            }
        }
    }

    fn local_name(&self) -> &'static str {
        match self {
            GmlAttribute::Id => "gml:id",
            GmlAttribute::SrsDimension => "srsDimension",
            GmlAttribute::SrsName => "srsName",
        }
    }
}

pub fn deserialize_gml_attributes(
    xml_document: &[u8],
) -> Result<HashMap<GmlAttribute, String>, Error> {
    let mut reader = Reader::from_reader(xml_document);
    loop {
        match reader.read_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let mut attributes = HashMap::new();
                for attr in e.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    let Some(gml_attribute) =
                        GmlAttribute::from_local_name(attr.key.local_name().as_ref())
                    else {
                        continue;
                    };
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
                    attributes.insert(gml_attribute, value);
                }
                return Ok(attributes);
            }
            Event::Eof => return Err(Error::XmlDe(quick_xml::DeError::UnexpectedEof)),
            _ => {}
        }
    }
}
