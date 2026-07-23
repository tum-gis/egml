use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_kind, deserialize_abstract_solid_kind,
    deserialize_abstract_surface_kind, deserialize_point, serialize_abstract_curve_kind,
    serialize_abstract_solid_kind, serialize_abstract_surface_kind, serialize_point,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractGeometricPrimitiveKind;

pub fn deserialize_abstract_geometric_primitive_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractGeometricPrimitiveKind>, Error> {
    if let Some(x) = deserialize_abstract_curve_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_solid_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(span) = spans.first(GmlElement::Point) {
        let point = deserialize_point(&xml_document[span.start..span.end])?;
        return Ok(Some(point.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_geometric_primitive_kind(
    abstract_geometric_primitive_kind: &AbstractGeometricPrimitiveKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_geometric_primitive_kind {
        AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => {
            serialize_abstract_curve_kind(x, formatting)
        }
        AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => {
            serialize_abstract_solid_kind(x, formatting)
        }
        AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => {
            serialize_abstract_surface_kind(x, formatting)
        }
        AbstractGeometricPrimitiveKind::Point(x) => serialize_point(x, formatting),
    }
}
