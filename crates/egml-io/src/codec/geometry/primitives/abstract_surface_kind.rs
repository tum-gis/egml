use crate::Error;
use crate::codec::geometry::complexes::{
    deserialize_composite_surface, serialize_composite_surface,
};
use crate::codec::geometry::primitives::polygon::deserialize_polygon;
use crate::codec::geometry::primitives::shell::deserialize_shell;
use crate::codec::geometry::primitives::surface::deserialize_surface;
use crate::codec::geometry::primitives::{
    deserialize_surface_kind, deserialize_surface_kind_for, serialize_polygon, serialize_shell,
    serialize_surface, serialize_surface_kind,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractSurfaceKind;
use std::io::Write;

pub fn deserialize_abstract_surface_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSurfaceKind>, Error> {
    if let Some(node) = index.first(GmlElement::CompositeSurface) {
        let composite_surface =
            deserialize_composite_surface(&xml_document[node.range()], node, config)?;
        return Ok(Some(composite_surface.into()));
    }

    if let Some(node) = index.first(GmlElement::Shell) {
        let shell = deserialize_shell(&xml_document[node.range()], node, config)?;
        return Ok(Some(shell.into()));
    }

    if let Some(node) = index.first(GmlElement::Polygon) {
        let polygon = deserialize_polygon(&xml_document[node.range()], node, config)?;
        return Ok(Some(polygon.into()));
    }

    if let Some(node) = index.first(GmlElement::Surface) {
        let surface = deserialize_surface(&xml_document[node.range()], node, config)?;
        return Ok(Some(surface.into()));
    }

    if let Some(x) = deserialize_surface_kind(xml_document, index, config)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

/// See [`deserialize_abstract_ring_kind_for`](super::deserialize_abstract_ring_kind_for):
/// deserializes `node` as an [`AbstractSurfaceKind`] given that `element` is
/// already known to be its own type, falling through to the `SurfaceKind`
/// vocabulary for anything not matched directly here.
pub fn deserialize_abstract_surface_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSurfaceKind>, Error> {
    if element == GmlElement::CompositeSurface {
        let composite_surface = deserialize_composite_surface(xml_document, node, config)?;
        return Ok(Some(composite_surface.into()));
    }

    if element == GmlElement::Shell {
        let shell = deserialize_shell(xml_document, node, config)?;
        return Ok(Some(shell.into()));
    }

    if element == GmlElement::Polygon {
        let polygon = deserialize_polygon(xml_document, node, config)?;
        return Ok(Some(polygon.into()));
    }

    if element == GmlElement::Surface {
        let surface = deserialize_surface(xml_document, node, config)?;
        return Ok(Some(surface.into()));
    }

    if let Some(x) = deserialize_surface_kind_for(element, xml_document, node, config)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_surface_kind<W: Write>(
    abstract_surface_kind: &AbstractSurfaceKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_surface_kind {
        AbstractSurfaceKind::CompositeSurface(x) => {
            serialize_composite_surface(x, xml_fragment_writer)
        }
        AbstractSurfaceKind::Polygon(x) => serialize_polygon(x, xml_fragment_writer),
        AbstractSurfaceKind::Shell(x) => serialize_shell(x, xml_fragment_writer),
        AbstractSurfaceKind::Surface(x) => serialize_surface(x, xml_fragment_writer),
        AbstractSurfaceKind::SurfaceKind(x) => serialize_surface_kind(x, xml_fragment_writer),
    }
}
