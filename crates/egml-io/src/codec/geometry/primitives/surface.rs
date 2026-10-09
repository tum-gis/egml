use crate::Error;
use crate::codec::geometry::primitives::abstract_surface::{
    deserialize_abstract_surface, serialize_abstract_surface, serialize_abstract_surface_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_patch_array_property,
    serialize_abstract_surface_patch_array_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlElement,
    XmlFragmentWriter, collect_child,
};
use egml_core::model::geometry::primitives::{AsAbstractSurface, Surface};
use std::io::Write;

pub fn deserialize_surface(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Surface, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_surface = deserialize_abstract_surface(xml_document, index, config)?;

    let patches = {
        let via_patches = collect_child(
            xml_document,
            index,
            GmlElement::PatchesProperty,
            config,
            deserialize_abstract_surface_patch_array_property,
        )?
        .filter(|patches| !patches.is_empty());
        if via_patches.is_some() {
            via_patches
        } else {
            collect_child(
                xml_document,
                index,
                GmlElement::TrianglePatchesProperty,
                config,
                deserialize_abstract_surface_patch_array_property,
            )?
            .filter(|patches| !patches.is_empty())
        }
    }
    .ok_or_else(|| Error::ElementNotFound(GmlElement::PatchesProperty.local_name().to_string()))?;

    let surface = Surface::from_abstract_surface(abstract_surface, patches);
    Ok(surface)
}

pub fn serialize_surface<W: Write>(
    surface: &Surface,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_surface_attributes(surface);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::Surface,
        attributes,
    )?;

    serialize_abstract_surface(surface.abstract_surface(), xml_fragment_writer)?;

    serialize_abstract_surface_patch_array_property(
        surface.patches(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::PatchesProperty,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::Surface)?;

    Ok(())
}

pub fn serialize_surface_attributes(surface: &Surface) -> Vec<(String, String)> {
    serialize_abstract_surface_attributes(surface.abstract_surface())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::Surface, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_surface(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::serialize_surface;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::base::{AsAbstractGml, Id};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfacePatchKind, LinearRing, PolygonPatch, Surface, Triangle,
    };

    fn make_surface_with_polygon_patches() -> Surface {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let exterior = AbstractRingKind::LinearRing(ring);
        let patch = PolygonPatch::new(Some(exterior), vec![]);
        let patches = vec![AbstractSurfacePatchKind::PolygonPatch(patch)];
        Surface::new(patches)
    }

    fn make_surface_with_triangles() -> Surface {
        let t1 = Triangle::from_points(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        let t2 = Triangle::from_points(
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        let patches = vec![
            AbstractSurfacePatchKind::Triangle(t1),
            AbstractSurfacePatchKind::Triangle(t2),
        ];
        Surface::new(patches)
    }

    #[test]
    fn deserialize_surface_with_polygon_patches() {
        let xml_document = b"
        <gml:Surface gml:id=\"my-surface-id\">
            <gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>
        </gml:Surface>";

        let surface = deserialize(xml_document).expect("should deserialize");

        assert_eq!(
            surface.id().unwrap(),
            &Id::try_from("my-surface-id").unwrap()
        );
        assert_eq!(surface.patches().len(), 1);
        assert!(matches!(
            surface.patches()[0],
            AbstractSurfacePatchKind::PolygonPatch(_)
        ));
    }

    #[test]
    fn deserialize_surface_with_triangles() {
        let xml_document = b"<gml:Surface>
            <gml:patches>
                <gml:Triangle>
                    <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>
                </gml:Triangle>
                <gml:Triangle>
                    <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList></gml:LinearRing></gml:exterior>
                </gml:Triangle>
            </gml:patches>
        </gml:Surface>";

        let surface = deserialize(xml_document).expect("should deserialize");

        assert_eq!(surface.patches().len(), 2);
        assert!(matches!(
            surface.patches()[0],
            AbstractSurfacePatchKind::Triangle(_)
        ));
    }

    #[test]
    fn serialize_surface_writes_gml_tags() {
        let surface = make_surface_with_polygon_patches();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:Surface"));
        assert!(xml.contains("<gml:patches"));
        assert!(xml.contains("<gml:PolygonPatch"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn serialize_surface_with_triangles_writes_gml_tags() {
        let surface = make_surface_with_triangles();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:Surface"));
        assert!(xml.contains("<gml:patches"));
        assert_eq!(xml.matches("<gml:Triangle").count(), 2);
    }

    #[test]
    fn deserialize_deprecated_surface_with_triangle_patches() {
        let xml_document = b"<gml:Surface>
            <gml:trianglePatches>
                <gml:Triangle>
                    <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>
                </gml:Triangle>
                <gml:Triangle>
                    <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList></gml:LinearRing></gml:exterior>
                </gml:Triangle>
                <gml:Triangle>
                    <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 1 0 1 1 0 0 0 1 0 1 0</gml:posList></gml:LinearRing></gml:exterior>
                </gml:Triangle>
            </gml:trianglePatches>
        </gml:Surface>";

        let surface = deserialize(xml_document).expect("should deserialize");

        assert_eq!(surface.patches().len(), 3);
        assert!(matches!(
            surface.patches()[0],
            AbstractSurfacePatchKind::Triangle(_)
        ));
    }

    #[test]
    fn round_trip_surface_with_polygon_patches_preserves_patch_count() {
        let surface = make_surface_with_polygon_patches();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_bytes()).expect("should deserialize");

        assert_eq!(recovered.patches().len(), surface.patches().len());
    }

    #[test]
    fn round_trip_surface_with_triangles_preserves_patch_count() {
        let surface = make_surface_with_triangles();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_bytes()).expect("should deserialize");

        assert_eq!(recovered.patches().len(), surface.patches().len());
    }
}
