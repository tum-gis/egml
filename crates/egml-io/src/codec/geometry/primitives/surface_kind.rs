use crate::Error;
use crate::codec::geometry::primitives::serialize_triangulated_surface;
use crate::codec::geometry::primitives::triangulated_surface::deserialize_triangulated_surface;
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::SurfaceKind;
use std::io::Write;

pub fn deserialize_surface_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<SurfaceKind>, Error> {
    if let Some(node) = index.first(GmlElement::TriangulatedSurface) {
        let triangulated_surface =
            deserialize_triangulated_surface(&xml_document[node.range()], node, config)?;
        return Ok(Some(triangulated_surface.into()));
    }

    Ok(None)
}

/// See [`deserialize_abstract_ring_kind_for`](super::deserialize_abstract_ring_kind_for):
/// deserializes `node` as a [`SurfaceKind`] given that `element` is already
/// known to be its own type.
pub fn deserialize_surface_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<SurfaceKind>, Error> {
    if element == GmlElement::TriangulatedSurface {
        let triangulated_surface = deserialize_triangulated_surface(xml_document, node, config)?;
        return Ok(Some(triangulated_surface.into()));
    }

    Ok(None)
}

pub fn serialize_surface_kind<W: Write>(
    surface_kind: &SurfaceKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match surface_kind {
        SurfaceKind::TriangulatedSurface(x) => {
            serialize_triangulated_surface(x, xml_fragment_writer)
        }
    }
}
