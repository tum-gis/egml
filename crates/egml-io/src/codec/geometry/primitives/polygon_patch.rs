use crate::Error;
use crate::codec::geometry::primitives::abstract_surface_patch::{
    deserialize_abstract_surface_patch, serialize_abstract_surface_patch,
    serialize_abstract_surface_patch_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_property, serialize_abstract_ring_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_child, collect_children,
};
use egml_core::model::geometry::primitives::{
    AbstractRingKind, AsAbstractSurfacePatch, PolygonPatch,
};
use std::io::Write;

pub fn deserialize_polygon_patch(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<PolygonPatch, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_surface_patch = deserialize_abstract_surface_patch(xml_document, index, config)?;

    let exterior = collect_child(
        xml_document,
        index,
        GmlElement::ExteriorProperty,
        config,
        deserialize_abstract_ring_property,
    )?;
    let interior: Vec<AbstractRingKind> = collect_children(
        xml_document,
        index,
        GmlElement::InteriorProperty,
        config,
        deserialize_abstract_ring_property,
    )?;

    let polygon_patch =
        PolygonPatch::from_abstract_surface_patch(abstract_surface_patch, exterior, interior);
    Ok(polygon_patch)
}

pub fn serialize_polygon_patch<W: Write>(
    polygon_patch: &PolygonPatch,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_polygon_patch_attributes(polygon_patch);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::PolygonPatch,
        attributes,
    )?;

    serialize_abstract_surface_patch(polygon_patch.abstract_surface_patch(), xml_fragment_writer)?;

    if let Some(object) = &polygon_patch.exterior() {
        serialize_abstract_ring_property(
            object,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::ExteriorProperty,
        )?;
    }
    for prop in polygon_patch.interior() {
        serialize_abstract_ring_property(
            prop,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::InteriorProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::PolygonPatch)?;

    Ok(())
}

pub fn serialize_polygon_patch_attributes(polygon_patch: &PolygonPatch) -> Vec<(String, String)> {
    serialize_abstract_surface_patch_attributes(polygon_patch.abstract_surface_patch())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::PolygonPatch, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_polygon_patch(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::serialize_polygon_patch;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AbstractRingKind, LinearRing, PolygonPatch};

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
            deserialize(xml_document.as_ref()).expect("deserialize should work");

        let exterior: &AbstractRingKind = polygon_patch.exterior().expect("should be set");
        match exterior {
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
            deserialize(xml_document).expect("deserialize should work");

        assert_eq!(polygon_patch.interior().len(), 2);

        let exterior: &AbstractRingKind = polygon_patch.exterior().expect("should be set");
        match exterior {
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
        PolygonPatch::new(Some(ring_kind), vec![])
    }

    #[test]
    fn serialize_polygon_patch_writes_gml_tags() {
        let polygon_patch = make_polygon_patch();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_polygon_patch(&polygon_patch, &mut xml_fragment_writer)
            .expect("serialize should work");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

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
            deserialize(input_xml.as_ref()).expect("deserialize should work");

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_polygon_patch(&polygon_patch, &mut xml_fragment_writer)
            .expect("serialize should work");
        let output_xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert_eq!(input_xml, output_xml);
    }
}
