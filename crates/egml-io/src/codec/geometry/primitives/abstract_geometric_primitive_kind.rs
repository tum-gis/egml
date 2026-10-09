use crate::Error;
use crate::codec::geometry::primitives::point::deserialize_point;
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_kind, deserialize_abstract_curve_kind_for,
    deserialize_abstract_solid_kind, deserialize_abstract_solid_kind_for,
    deserialize_abstract_surface_kind, deserialize_abstract_surface_kind_for,
    serialize_abstract_curve_kind, serialize_abstract_solid_kind, serialize_abstract_surface_kind,
    serialize_point,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractGeometricPrimitiveKind;
use std::io::Write;

pub fn deserialize_abstract_geometric_primitive_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractGeometricPrimitiveKind>, Error> {
    if let Some(x) = deserialize_abstract_curve_kind(xml_document, index, config)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_solid_kind(xml_document, index, config)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_surface_kind(xml_document, index, config)? {
        return Ok(Some(x.into()));
    }

    if let Some(node) = index.first(GmlElement::Point) {
        let point = deserialize_point(&xml_document[node.range()], node, config)?;
        return Ok(Some(point.into()));
    }

    Ok(None)
}

/// See [`deserialize_abstract_ring_kind_for`](super::deserialize_abstract_ring_kind_for):
/// deserializes `node` as an [`AbstractGeometricPrimitiveKind`] given that
/// `element` is already known to be its own type, dispatching straight to
/// whichever sub-vocabulary (curve, solid, surface, or `Point` itself)
/// `element` belongs to.
pub fn deserialize_abstract_geometric_primitive_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractGeometricPrimitiveKind>, Error> {
    if let Some(x) = deserialize_abstract_curve_kind_for(element, xml_document, node, config)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_solid_kind_for(element, xml_document, node, config)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_surface_kind_for(element, xml_document, node, config)? {
        return Ok(Some(x.into()));
    }

    if element == GmlElement::Point {
        let point = deserialize_point(xml_document, node, config)?;
        return Ok(Some(point.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_geometric_primitive_kind<W: Write>(
    abstract_geometric_primitive_kind: &AbstractGeometricPrimitiveKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_geometric_primitive_kind {
        AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => {
            serialize_abstract_curve_kind(x, xml_fragment_writer)
        }
        AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => {
            serialize_abstract_solid_kind(x, xml_fragment_writer)
        }
        AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => {
            serialize_abstract_surface_kind(x, xml_fragment_writer)
        }
        AbstractGeometricPrimitiveKind::Point(x) => serialize_point(x, xml_fragment_writer),
    }
}
