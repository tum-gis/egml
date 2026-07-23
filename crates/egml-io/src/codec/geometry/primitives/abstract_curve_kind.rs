use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_kind, deserialize_line_string, serialize_abstract_ring_kind,
    serialize_line_string,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractCurveKind;

pub fn deserialize_abstract_curve_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractCurveKind>, Error> {
    if let Some(span) = spans.first(GmlElement::LineString) {
        let linear_string = deserialize_line_string(&xml_document[span.start..span.end])?;
        return Ok(Some(linear_string.into()));
    }
    if let Some(x) = deserialize_abstract_ring_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_curve_kind(
    abstract_curve_kind: &AbstractCurveKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_curve_kind {
        AbstractCurveKind::LineString(x) => serialize_line_string(x, formatting),
        AbstractCurveKind::AbstractRingKind(x) => serialize_abstract_ring_kind(x, formatting),
    }
}
