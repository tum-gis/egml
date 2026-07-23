use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_polygon_patch, deserialize_triangle, serialize_polygon_patch, serialize_triangle,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractSurfacePatchKind;

pub fn deserialize_abstract_surface_patch_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractSurfacePatchKind>, Error> {
    if let Some(span) = spans.first(GmlElement::PolygonPatch) {
        let polygon_patch = deserialize_polygon_patch(&xml_document[span.start..span.end])?;
        return Ok(Some(polygon_patch.into()));
    }

    if let Some(span) = spans.first(GmlElement::Triangle) {
        let triangle = deserialize_triangle(&xml_document[span.start..span.end])?;
        return Ok(Some(triangle.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_surface_patch_kind(
    abstract_surface_patch_kind: &AbstractSurfacePatchKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_surface_patch_kind {
        AbstractSurfacePatchKind::PolygonPatch(x) => serialize_polygon_patch(x, formatting),
        AbstractSurfacePatchKind::Triangle(x) => serialize_triangle(x, formatting),
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_abstract_surface_patch_kind;
    use crate::util::extract_xml_element_spans;
    use egml_core::model::geometry::primitives::AbstractSurfacePatchKind;

    #[test]
    fn deserialize_surface_patch_kind_as_polygon_patch() {
        let xml_document = b"<>
            <gml:PolygonPatch>
                <gml:exterior>
                    <gml:LinearRing>
                        <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                    </gml:LinearRing>
                </gml:exterior>
            </gml:PolygonPatch></>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let surface_patch_kind: AbstractSurfacePatchKind =
            deserialize_abstract_surface_patch_kind(xml_document.as_ref(), &spans)
                .expect("should deserialize")
                .expect("should be some");

        if let AbstractSurfacePatchKind::PolygonPatch(x) = surface_patch_kind {
            assert!(x.exterior().is_some());
        } else {
            panic!("should be polygon patch");
        }
    }
}
