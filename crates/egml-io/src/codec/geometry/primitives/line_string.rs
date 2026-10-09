use crate::Error;
use crate::codec::geometry::direct_position::direct_position_from_start;
use crate::codec::geometry::direct_position_list::{
    direct_position_list_from_start, serialize_direct_position_list,
};
use crate::codec::geometry::primitives::abstract_curve::{
    deserialize_abstract_curve, serialize_abstract_curve, serialize_abstract_curve_attributes,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    dedup_adjacent_positions,
};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AsAbstractCurve, LineString};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Write;

pub fn deserialize_line_string(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<LineString, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_curve = deserialize_abstract_curve(xml_document, index, config)?;

    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event()? {
            Event::Start(start) if start.local_name().as_ref() == "LineString" => break,
            Event::Eof => return Err(Error::ElementNotFound("gml:LineString".to_string())),
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
            Event::End(end) if end.local_name().as_ref() == "LineString" => break,
            Event::Eof => return Err(Error::ElementNotFound("gml:posList".to_string())),
            _ => {}
        }
    }

    dedup_adjacent_positions(&mut points, "LineString");

    let line_string = LineString::from_abstract_curve(abstract_curve, points)?;
    Ok(line_string)
}

pub fn serialize_line_string<W: Write>(
    line_string: &LineString,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_line_string_attributes(line_string);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::LineString,
        attributes,
    )?;

    serialize_abstract_curve(line_string.abstract_curve(), xml_fragment_writer)?;

    serialize_direct_position_list(
        line_string.points(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::PosListProperty,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::LineString)?;

    Ok(())
}

pub fn serialize_line_string_attributes(line_string: &LineString) -> Vec<(String, String)> {
    serialize_abstract_curve_attributes(line_string.abstract_curve())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::LineString, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_line_string(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::line_string::serialize_line_string;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::LineString;

    fn make_line_string() -> LineString {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 0.0, 0.0).unwrap(),
        ];
        LineString::new(points).unwrap()
    }

    #[test]
    fn deserialize_line_string() {
        let xml_document = b"<gml:LineString>
                      <gml:posList srsDimension=\"3\">0.0 0.0 0.0 1.0 1.0 1.0 2.0 2.0 2.0</gml:posList>
                    </gml:LineString>";

        let line_string = deserialize(xml_document.as_ref()).expect("should deserialize");
        assert_eq!(line_string.points().len(), 3);
    }

    #[test]
    fn deserialize_line_string_repairs_adjacent_duplicate_points() {
        let xml_document = b"<gml:LineString>
                      <gml:posList srsDimension=\"3\">0 0 0 1 0 0 1 0 0 2 0 0</gml:posList>
                    </gml:LineString>";

        let line_string = deserialize(xml_document).expect("should deserialize");

        assert_eq!(line_string.points().len(), 3);
    }

    #[test]
    fn deserialize_line_string_with_repeated_pos() {
        let xml_document = b"<gml:LineString>
              <gml:pos>0.0 0.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 1.0</gml:pos>
            </gml:LineString>";

        let line_string = deserialize(xml_document.as_ref()).expect("should deserialize");
        assert_eq!(line_string.points().len(), 3);
    }

    #[test]
    fn serialize_line_string_writes_gml_tags() {
        let line_string = make_line_string();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_line_string(&line_string, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_line_string_preserves_points() {
        let line_string = make_line_string();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_line_string(&line_string, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered_line_string = deserialize(xml.as_ref()).expect("should deserialize");

        assert_eq!(
            recovered_line_string.points().len(),
            line_string.points().len()
        );
        for (a, b) in recovered_line_string
            .points()
            .iter()
            .zip(line_string.points().iter())
        {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_line_string_from_xml() {
        let input_xml = "<gml:LineString>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 2 0 0</gml:posList>\
            </gml:LineString>";

        let line_string = deserialize(input_xml.as_bytes()).unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_line_string(&line_string, &mut xml_fragment_writer).unwrap();
        let output_xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_line_string_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let line_string = LineString::new(points.clone()).unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_line_string(&line_string, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered_line_string = deserialize(xml.as_ref()).expect("should deserialize");

        for (a, b) in recovered_line_string.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }
}
