use crate::Error;
use crate::codec::geometry::primitives::serialize_solid;
use crate::codec::geometry::primitives::solid::deserialize_solid;
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractSolidKind;
use std::io::Write;

pub fn deserialize_abstract_solid_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSolidKind>, Error> {
    if let Some(node) = index.first(GmlElement::Solid) {
        let solid = deserialize_solid(&xml_document[node.range()], node, config)?;
        return Ok(Some(solid.into()));
    }

    Ok(None)
}

/// See [`deserialize_abstract_ring_kind_for`](super::deserialize_abstract_ring_kind_for):
/// deserializes `node` as an [`AbstractSolidKind`] given that `element` is
/// already known to be its own type.
pub fn deserialize_abstract_solid_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSolidKind>, Error> {
    if element == GmlElement::Solid {
        let solid = deserialize_solid(xml_document, node, config)?;
        return Ok(Some(solid.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_solid_kind<W: Write>(
    abstract_solid_kind: &AbstractSolidKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_solid_kind {
        AbstractSolidKind::Solid(x) => serialize_solid(x, xml_fragment_writer),
    }
}
