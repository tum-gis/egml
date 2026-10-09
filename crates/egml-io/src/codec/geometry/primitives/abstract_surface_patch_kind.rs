use crate::Error;
use crate::codec::geometry::primitives::polygon_patch::deserialize_polygon_patch;
use crate::codec::geometry::primitives::triangle::deserialize_triangle;
use crate::codec::geometry::primitives::{serialize_polygon_patch, serialize_triangle};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractSurfacePatchKind;
use std::io::Write;

pub fn deserialize_abstract_surface_patch_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSurfacePatchKind>, Error> {
    if let Some(node) = index.first(GmlElement::PolygonPatch) {
        let polygon_patch = deserialize_polygon_patch(&xml_document[node.range()], node, config)?;
        return Ok(Some(polygon_patch.into()));
    }

    if let Some(node) = index.first(GmlElement::Triangle) {
        let triangle = deserialize_triangle(&xml_document[node.range()], node, config)?;
        return Ok(Some(triangle.into()));
    }

    Ok(None)
}

/// Deserializes `node` as an [`AbstractSurfacePatchKind`] given that
/// `element` is already known to be `node`'s own type — see
/// [`crate::codec::geometry::deserialize_abstract_geometry_kind_for`] for
/// why this avoids a search over `node`.
pub fn deserialize_abstract_surface_patch_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractSurfacePatchKind>, Error> {
    match element {
        GmlElement::PolygonPatch => {
            let polygon_patch = deserialize_polygon_patch(xml_document, node, config)?;
            Ok(Some(polygon_patch.into()))
        }
        GmlElement::Triangle => {
            let triangle = deserialize_triangle(xml_document, node, config)?;
            Ok(Some(triangle.into()))
        }
        _ => Ok(None),
    }
}

pub fn serialize_abstract_surface_patch_kind<W: Write>(
    abstract_surface_patch_kind: &AbstractSurfacePatchKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_surface_patch_kind {
        AbstractSurfacePatchKind::PolygonPatch(x) => {
            serialize_polygon_patch(x, xml_fragment_writer)
        }
        AbstractSurfacePatchKind::Triangle(x) => serialize_triangle(x, xml_fragment_writer),
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_abstract_surface_patch_kind;
    use crate::util::XmlDocumentIndex;
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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let surface_patch_kind: AbstractSurfacePatchKind = deserialize_abstract_surface_patch_kind(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize")
        .expect("should be some");

        if let AbstractSurfacePatchKind::PolygonPatch(x) = surface_patch_kind {
            assert!(x.exterior().is_some());
        } else {
            panic!("should be polygon patch");
        }
    }
}
