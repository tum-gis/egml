use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_triangulated_surface, serialize_triangulated_surface,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::SurfaceKind;

pub fn deserialize_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<SurfaceKind>, Error> {
    if let Some(span) = spans.first(GmlElement::TriangulatedSurface) {
        let triangulated_surface =
            deserialize_triangulated_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(triangulated_surface.into()));
    }

    Ok(None)
}

pub fn serialize_surface_kind(
    surface_kind: &SurfaceKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match surface_kind {
        SurfaceKind::TriangulatedSurface(x) => serialize_triangulated_surface(x, formatting),
    }
}
