use crate::Error;
use crate::codec::geometry::primitives::abstract_surface::{
    deserialize_abstract_surface, serialize_abstract_surface,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_patch_array_property,
    serialize_abstract_surface_patch_array_property,
};
use crate::util::{
    Formatting, GmlElement, XmlElement, XmlNode, XmlNodeContent, collect_child,
    extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::{AsAbstractSurface, Surface};

pub fn deserialize_surface(xml_document: &[u8]) -> Result<Surface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface = deserialize_abstract_surface(xml_document, &spans)?;

    let patches = {
        let via_patches = collect_child(
            xml_document,
            &spans,
            GmlElement::PatchesProperty,
            deserialize_abstract_surface_patch_array_property,
        )?
        .flatten();
        if via_patches.is_some() {
            via_patches
        } else {
            collect_child(
                xml_document,
                &spans,
                GmlElement::TrianglePatchesProperty,
                deserialize_abstract_surface_patch_array_property,
            )?
            .flatten()
        }
    }
    .ok_or_else(|| Error::ElementNotFound(GmlElement::PatchesProperty.as_str().to_string()))?;

    let surface = Surface::from_abstract_surface(abstract_surface, patches);
    Ok(surface)
}

pub fn serialize_surface(surface: &Surface, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_surface(surface.abstract_surface(), formatting)?;

    xml_node_parts.content.push(XmlNodeContent::Child(
        serialize_abstract_surface_patch_array_property(
            surface.patches(),
            formatting,
            GmlElement::PatchesProperty.into(),
        )?,
    ));

    Ok(XmlNode::new(GmlElement::Surface.into(), xml_node_parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{deserialize_surface, serialize_surface};
    use crate::util::Formatting;
    use egml_core::model::base::{AsAbstractGml, Id};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfacePatchArrayProperty,
        AbstractSurfacePatchKind, LinearRing, PolygonPatch, Surface, Triangle,
    };

    fn make_surface_with_polygon_patches() -> Surface {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let exterior = AbstractRingProperty::from_object(AbstractRingKind::LinearRing(ring));
        let patch = PolygonPatch::new(Some(exterior), vec![]);
        let patches = AbstractSurfacePatchArrayProperty::from_objects(vec![
            AbstractSurfacePatchKind::PolygonPatch(patch),
        ]);
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
        let patches = AbstractSurfacePatchArrayProperty::from_objects(vec![
            AbstractSurfacePatchKind::Triangle(t1),
            AbstractSurfacePatchKind::Triangle(t2),
        ]);
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

        let surface = deserialize_surface(xml_document).expect("should deserialize");

        assert_eq!(
            surface.id().unwrap(),
            &Id::try_from("my-surface-id").unwrap()
        );
        assert_eq!(surface.patches().objects().len(), 1);
        assert!(matches!(
            surface.patches().objects()[0],
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

        let surface = deserialize_surface(xml_document).expect("should deserialize");

        assert_eq!(surface.patches().objects().len(), 2);
        assert!(matches!(
            surface.patches().objects()[0],
            AbstractSurfacePatchKind::Triangle(_)
        ));
    }

    #[test]
    fn serialize_surface_writes_gml_tags() {
        let surface = make_surface_with_polygon_patches();

        let xml_node = serialize_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

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

        let xml_node = serialize_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

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

        let surface = deserialize_surface(xml_document).expect("should deserialize");

        assert_eq!(surface.patches().objects().len(), 3);
        assert!(matches!(
            surface.patches().objects()[0],
            AbstractSurfacePatchKind::Triangle(_)
        ));
    }

    #[test]
    fn round_trip_surface_with_polygon_patches_preserves_patch_count() {
        let surface = make_surface_with_polygon_patches();

        let xml_node = serialize_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        let recovered = deserialize_surface(xml.as_bytes()).expect("should deserialize");

        assert_eq!(
            recovered.patches().objects().len(),
            surface.patches().objects().len()
        );
    }

    #[test]
    fn round_trip_surface_with_triangles_preserves_patch_count() {
        let surface = make_surface_with_triangles();

        let xml_node = serialize_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        let recovered = deserialize_surface(xml.as_bytes()).expect("should deserialize");

        assert_eq!(
            recovered.patches().objects().len(),
            surface.patches().objects().len()
        );
    }
}
