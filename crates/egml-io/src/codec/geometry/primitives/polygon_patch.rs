use crate::Error;
use crate::codec::geometry::primitives::abstract_surface_patch::{
    deserialize_abstract_surface_patch, serialize_abstract_surface_patch,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_property, serialize_abstract_ring_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, collect_children,
    extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::{
    AbstractRingProperty, AsAbstractSurfacePatch, PolygonPatch,
};

pub fn deserialize_polygon_patch(xml_document: &[u8]) -> Result<PolygonPatch, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface_patch = deserialize_abstract_surface_patch(xml_document, &spans)?;

    let exterior = collect_child(
        xml_document,
        &spans,
        GmlElement::ExteriorProperty,
        deserialize_abstract_ring_property,
    )?;
    let interior: Vec<AbstractRingProperty> = collect_children(
        xml_document,
        &spans,
        GmlElement::InteriorProperty,
        deserialize_abstract_ring_property,
    )?;

    let polygon_patch =
        PolygonPatch::from_abstract_surface_patch(abstract_surface_patch, exterior, interior);
    Ok(polygon_patch)
}

pub fn serialize_polygon_patch(
    polygon_patch: &PolygonPatch,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_surface_patch(polygon_patch.abstract_surface_patch(), formatting)?;

    if let Some(object) = &polygon_patch.exterior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_ring_property(
                object,
                formatting,
                GmlElement::ExteriorProperty.into(),
            )?));
    }
    for prop in polygon_patch.interior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_ring_property(
                prop,
                formatting,
                GmlElement::InteriorProperty.into(),
            )?));
    }

    Ok(XmlNode::new(
        GmlElement::PolygonPatch.into(),
        xml_node_parts,
    ))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{deserialize_polygon_patch, serialize_polygon_patch};
    use crate::util::Formatting;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, LinearRing, PolygonPatch,
    };

    #[test]
    fn deserialize_polygon_patch_test() {
        let xml_document = b"<gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>";
        let polygon_patch: PolygonPatch =
            deserialize_polygon_patch(xml_document.as_ref()).expect("deserialize should work");

        let exterior: &AbstractRingProperty = polygon_patch.exterior().expect("should be set");
        match exterior.object().expect("should be set") {
            AbstractRingKind::LinearRing(x) => {
                assert_eq!(x.points().len(), 3);
            }
            _ => panic!("should be linear ring"),
        }
    }

    #[test]
    fn deserialize_polygon_patch_with_interior_rings() {
        let xml_document = b"<gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                    <gml:interior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:interior>
                    <gml:interior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:interior>
                </gml:PolygonPatch>";
        let polygon_patch: PolygonPatch =
            deserialize_polygon_patch(xml_document).expect("deserialize should work");

        assert_eq!(polygon_patch.interior().len(), 2);

        let exterior: &AbstractRingProperty = polygon_patch.exterior().expect("should be set");
        match exterior.object().expect("should be set") {
            AbstractRingKind::LinearRing(x) => {
                assert_eq!(x.points().len(), 3);
            }
            _ => panic!("should be linear ring"),
        }
    }

    fn make_polygon_patch() -> PolygonPatch {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring_kind = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        PolygonPatch::new(Some(AbstractRingProperty::from_object(ring_kind)), vec![])
    }

    #[test]
    fn serialize_polygon_patch_writes_gml_tags() {
        let polygon_patch = make_polygon_patch();
        let xml_node = serialize_polygon_patch(&polygon_patch, Formatting::Compact)
            .expect("serialize should work");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:PolygonPatch"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_polygon_patch_from_xml() {
        let input_xml = "<gml:PolygonPatch>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:PolygonPatch>";

        let polygon_patch: PolygonPatch =
            deserialize_polygon_patch(input_xml.as_ref()).expect("deserialize should work");
        let output_xml_node = serialize_polygon_patch(&polygon_patch, Formatting::Compact)
            .expect("serialize should work");
        let output_xml = output_xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert_eq!(input_xml, output_xml);
    }
}
