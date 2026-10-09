use crate::Error;
use crate::codec::geometry::primitives::line_string::deserialize_line_string;
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_kind, deserialize_abstract_ring_kind_for,
    serialize_abstract_ring_kind, serialize_line_string,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractCurveKind;
use std::io::Write;

pub fn deserialize_abstract_curve_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractCurveKind>, Error> {
    if let Some(node) = index.first(GmlElement::LineString) {
        let linear_string = deserialize_line_string(&xml_document[node.range()], node, config)?;
        return Ok(Some(linear_string.into()));
    }
    if let Some(x) = deserialize_abstract_ring_kind(xml_document, index, config)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

/// See [`deserialize_abstract_ring_kind_for`]: deserializes `node` as an
/// [`AbstractCurveKind`] given that `element` is already known to be its
/// own type, falling through to the ring-kind vocabulary for anything that
/// isn't a `LineString`, and returning `None` (never panicking) for an
/// `element` outside this whole vocabulary.
pub fn deserialize_abstract_curve_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractCurveKind>, Error> {
    if element == GmlElement::LineString {
        let line_string = deserialize_line_string(xml_document, node, config)?;
        return Ok(Some(line_string.into()));
    }

    if let Some(x) = deserialize_abstract_ring_kind_for(element, xml_document, node, config)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_curve_kind<W: Write>(
    abstract_curve_kind: &AbstractCurveKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_curve_kind {
        AbstractCurveKind::LineString(x) => serialize_line_string(x, xml_fragment_writer),
        AbstractCurveKind::AbstractRingKind(x) => {
            serialize_abstract_ring_kind(x, xml_fragment_writer)
        }
    }
}
