use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
    serialize_abstract_geometric_primitive_attributes,
};
use crate::codec::geometry::{deserialize_direct_position, serialize_direct_position};
use crate::error::Error;
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
};
use egml_core::model::geometry::primitives::{AsAbstractGeometricPrimitive, Point};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Write;

pub fn deserialize_point(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Point, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, index, config)?;

    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event()? {
            Event::Start(start) if start.local_name().as_ref() == "Point" => break,
            Event::Eof => return Err(Error::ElementNotFound("gml:Point".to_string())),
            _ => {}
        }
    }
    let direct_position = deserialize_direct_position(&mut reader)?;

    let point =
        Point::from_abstract_geometric_primitive(abstract_geometric_primitive, direct_position);
    Ok(point)
}

pub fn serialize_point<W: Write>(
    point: &Point,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_point_attributes(point);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::Point,
        attributes,
    )?;

    serialize_abstract_geometric_primitive(
        point.abstract_geometric_primitive(),
        xml_fragment_writer,
    )?;

    serialize_direct_position(
        point.pos(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::Pos,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::Point)?;

    Ok(())
}

pub fn serialize_point_attributes(point: &Point) -> Vec<(String, String)> {
    serialize_abstract_geometric_primitive_attributes(point.abstract_geometric_primitive())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::Point, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_point(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::serialize_point;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::Point;

    #[test]
    fn deserialize_point_with_srs_dimension_and_id() {
        let xml_document = b"<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize(xml_document).unwrap();

        assert_eq!(result.pos().x(), 678000.9484065345);
        assert_eq!(result.pos().y(), 5403659.060043676);
        assert_eq!(result.pos().z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id() {
        let xml_document = b"<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize(xml_document).unwrap();

        assert_eq!(result.pos().x(), 678000.9484065345);
        assert_eq!(result.pos().y(), 5403659.060043676);
        assert_eq!(result.pos().z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id_and_srs_dimension() {
        let xml_document = b"<gml:Point>
              <gml:pos>678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize(xml_document).unwrap();

        assert_eq!(result.pos().x(), 678000.9484065345);
        assert_eq!(result.pos().y(), 5403659.060043676);
        assert_eq!(result.pos().z(), 417.3802376791456);
    }

    fn make_point(x: f64, y: f64, z: f64) -> Point {
        let pos = DirectPosition::new(x, y, z).unwrap();
        Point::new(pos)
    }

    #[test]
    fn serialize_point_writes_gml_tags() {
        let point = make_point(1.0, 2.0, 3.0);
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&point, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert!(xml.contains("1 2 3"));
    }

    #[test]
    fn round_trip_point_without_id() {
        let original = make_point(678000.9484065345, 5403659.060043676, 417.3802376791456);
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&original, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");
        let parsed = deserialize(xml.as_ref()).unwrap();

        assert_eq!(parsed.pos().x(), original.pos().x());
        assert_eq!(parsed.pos().y(), original.pos().y());
        assert_eq!(parsed.pos().z(), original.pos().z());
    }

    #[test]
    fn round_trip_point_with_id() {
        use egml_core::model::base::AsAbstractGmlMut;
        let pos = DirectPosition::new(10.0, 20.0, 30.0).unwrap();
        let mut original = Point::new(pos);
        original.set_id(Id::from_hashed_string("test-point"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&original, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("id="));
        let parsed = deserialize(xml.as_ref()).unwrap();
        assert_eq!(parsed.pos().x(), 10.0);
        assert_eq!(parsed.pos().y(), 20.0);
        assert_eq!(parsed.pos().z(), 30.0);
    }

    #[test]
    fn round_trip_point_from_xml_with_id() {
        let input_xml = b"<gml:Point gml:id=\"test-pt\">\
              <gml:pos srsDimension=\"3\">1 2 3</gml:pos>\
            </gml:Point>";

        let point = deserialize(input_xml).unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&point, &mut xml_fragment_writer).unwrap();
        let output_xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert_eq!(std::str::from_utf8(input_xml).unwrap(), output_xml);
    }

    #[test]
    fn round_trip_point_from_xml() {
        let input_xml = b"<gml:Point>\
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>\
            </gml:Point>";

        let point = deserialize(input_xml).unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&point, &mut xml_fragment_writer).unwrap();
        let output_xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert_eq!(std::str::from_utf8(input_xml).unwrap(), output_xml);
    }

    #[test]
    fn round_trip_point_preserves_float_precision() {
        let x = std::f64::consts::PI;
        let y = std::f64::consts::E;
        let z = std::f64::consts::SQRT_2;
        let original = make_point(x, y, z);

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point(&original, &mut xml_fragment_writer).unwrap();
        let output_xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");
        let parsed = deserialize(output_xml.as_ref()).unwrap();

        assert_eq!(parsed.pos().x(), x);
        assert_eq!(parsed.pos().y(), y);
        assert_eq!(parsed.pos().z(), z);
    }
}
