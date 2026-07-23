use crate::Error;
use crate::codec::geometry::complexes::deserialize_composite_surface;
use crate::codec::geometry::complexes::serialize_composite_surface;
use crate::codec::geometry::primitives::{
    deserialize_polygon, deserialize_shell, deserialize_surface, deserialize_surface_kind,
    serialize_polygon, serialize_shell, serialize_surface, serialize_surface_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractSurfaceKind;

pub fn deserialize_abstract_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractSurfaceKind>, Error> {
    if let Some(span) = spans.first(GmlElement::CompositeSurface) {
        let composite_surface = deserialize_composite_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(composite_surface.into()));
    }

    if let Some(span) = spans.first(GmlElement::Shell) {
        let shell = deserialize_shell(&xml_document[span.start..span.end])?;
        return Ok(Some(shell.into()));
    }

    if let Some(span) = spans.first(GmlElement::Polygon) {
        let polygon = deserialize_polygon(&xml_document[span.start..span.end])?;
        return Ok(Some(polygon.into()));
    }

    if let Some(span) = spans.first(GmlElement::Surface) {
        let surface = deserialize_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(surface.into()));
    }

    if let Some(x) = deserialize_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_surface_kind(
    abstract_surface_kind: &AbstractSurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_surface_kind {
        AbstractSurfaceKind::CompositeSurface(x) => serialize_composite_surface(x, formatting),
        AbstractSurfaceKind::Polygon(x) => serialize_polygon(x, formatting),
        AbstractSurfaceKind::Shell(x) => serialize_shell(x, formatting),
        AbstractSurfaceKind::Surface(x) => serialize_surface(x, formatting),
        AbstractSurfaceKind::SurfaceKind(x) => serialize_surface_kind(x, formatting),
    }
}
