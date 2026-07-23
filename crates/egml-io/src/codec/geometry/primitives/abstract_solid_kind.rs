use crate::Error;
use crate::codec::geometry::primitives::{deserialize_solid, serialize_solid};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractSolidKind;

pub fn deserialize_abstract_solid_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractSolidKind>, Error> {
    if let Some(span) = spans.first(GmlElement::Solid) {
        let solid = deserialize_solid(&xml_document[span.start..span.end])?;
        return Ok(Some(solid.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_solid_kind(
    abstract_solid_kind: &AbstractSolidKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_solid_kind {
        AbstractSolidKind::Solid(x) => serialize_solid(x, formatting),
    }
}
