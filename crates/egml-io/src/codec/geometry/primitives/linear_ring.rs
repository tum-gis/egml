use crate::codec::geometry::direct_position::direct_position_from_start;
use crate::codec::geometry::direct_position_list::{
    direct_position_list_from_start, serialize_direct_position_list,
};
use crate::codec::geometry::primitives::abstract_ring::{
    deserialize_abstract_ring, serialize_abstract_ring, serialize_abstract_ring_attributes,
};
use crate::error::Error;
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    dedup_adjacent_positions,
};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AsAbstractRing, LinearRing};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Write;

pub fn deserialize_linear_ring(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<LinearRing, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_ring = deserialize_abstract_ring(xml_document, index, config)?;

    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event()? {
            Event::Start(start) if start.local_name().as_ref() == "LinearRing" => break,
            Event::Eof => return Err(Error::ElementNotFound("gml:LinearRing".to_string())),
            _ => {}
        }
    }

    let mut points: Vec<DirectPosition> = Vec::new();
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => match start.local_name().as_ref() {
                "posList" => {
                    points = direct_position_list_from_start(&start, &mut reader)?;
                    break;
                }
                "pos" => {
                    points.push(direct_position_from_start(&start, &mut reader)?);
                }
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "LinearRing" => break,
            Event::Eof => return Err(Error::ElementNotFound("gml:posList".to_string())),
            _ => {}
        }
    }

    dedup_adjacent_positions(&mut points, "LinearRing");
    if points.first().unwrap() == points.last().unwrap() {
        points.pop();
    }

    let linear_ring = LinearRing::from_abstract_ring(abstract_ring, points)?;
    Ok(linear_ring)
}

pub fn serialize_linear_ring<W: Write>(
    linear_ring: &LinearRing,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_linear_ring_attributes(linear_ring);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::LinearRing,
        attributes,
    )?;

    serialize_abstract_ring(linear_ring.abstract_ring(), xml_fragment_writer)?;

    // GML requires the ring to be closed: the closing vertex (= first point) must
    // be written explicitly, but LinearRing stores points in open form (no repeat).
    let mut points: Vec<DirectPosition> = linear_ring.points().to_vec();
    if let Some(&first) = linear_ring.points().first() {
        points.push(first);
    }
    serialize_direct_position_list(
        &points,
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::PosListProperty,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::LinearRing)?;

    Ok(())
}

pub fn serialize_linear_ring_attributes(linear_ring: &LinearRing) -> Vec<(String, String)> {
    serialize_abstract_ring_attributes(linear_ring.abstract_ring())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::LinearRing, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_linear_ring(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::serialize_linear_ring;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::LinearRing;

    fn make_triangle() -> LinearRing {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        LinearRing::new(points).unwrap()
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list_and_id() {
        let xml_document = b"<gml:LinearRing gml:id=\"4018115_LR.d2yyBHNssydL8g3B8MvW\">
<gml:posList srsDimension=\"3\">
678058.447 5403817.567 424.209
678058.275 5403817.484 424.209
678058.689 5403816.628 424.209
678058.871 5403816.718 424.209
678058.447 5403817.567 424.209
</gml:posList>
</gml:LinearRing>";

        let linear_ring: LinearRing = deserialize(xml_document).expect("should deserialize");
        use egml_core::model::base::AsAbstractGml;
        assert!(linear_ring.id().is_some());
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list() {
        let xml_document = b"<gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>";

        let linear_ring_result = deserialize(xml_document);

        assert!(linear_ring_result.is_ok());
        let linear_ring = linear_ring_result.unwrap();
        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_linear_ring_repairs_adjacent_duplicate_points() {
        let xml_document = b"<gml:LinearRing>
                      <gml:posList>0 0 0 1 0 0 1 0 0 1 1 0 0 0 0</gml:posList>
                    </gml:LinearRing>";

        let linear_ring = deserialize(xml_document).expect("should deserialize");

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_linear_ring_with_duplicate_points_returns_error() {
        let xml_document = b"<gml:LinearRing gml:id=\"DEBY_LOD2_4959457_LR.EEAbfUPItTlOGZGH7VDv\">
                      <gml:posList>691040.851 5336002.449 529.908 691040.741 5336002.172 529.908 691040.851 5336002.449 529.908 691040.851 5336002.449 529.908</gml:posList>
                    </gml:LinearRing>";

        let linear_ring_result = deserialize(xml_document);

        assert!(linear_ring_result.is_err());
    }

    #[test]
    fn serialize_linear_ring_writes_gml_tags() {
        let linear_ring = make_triangle();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&linear_ring, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_linear_ring_appends_closing_vertex() {
        let ring = make_triangle(); // 3 open points

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&ring, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_ref()).expect("should deserialize");
        assert_eq!(recovered.points().len(), 3);
    }

    #[test]
    fn serialize_linear_ring_with_id() {
        use egml_core::model::base::AsAbstractGmlMut;
        let mut ring = make_triangle();
        ring.set_id(Id::from_hashed_string("test-ring"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&ring, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("id="));
    }

    #[test]
    fn round_trip_linear_ring_from_xml() {
        let xml_document = "<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing>";

        let linear_ring = deserialize(xml_document.as_ref()).expect("should deserialize");

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&linear_ring, &mut xml_fragment_writer).unwrap();
        let output_xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert_eq!(xml_document, output_xml);
    }

    #[test]
    fn deserialize_linear_ring_with_pos_elements() {
        let xml_document = b"<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                      <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                    </gml:LinearRing>";

        let linear_ring_result = deserialize(xml_document);

        assert!(linear_ring_result.is_ok());
        let linear_ring = linear_ring_result.unwrap();
        assert_eq!(linear_ring.points().len(), 5);
    }

    #[test]
    fn round_trip_linear_ring_preserves_points() {
        let ring = make_triangle();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&ring, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_ref()).expect("should deserialize");

        assert_eq!(recovered.points().len(), ring.points().len());
        for (a, b) in recovered.points().iter().zip(ring.points().iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_linear_ring_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let ring = LinearRing::new(points.clone()).unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_linear_ring(&ring, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_ref()).expect("should deserialize");

        for (a, b) in recovered.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }
}
