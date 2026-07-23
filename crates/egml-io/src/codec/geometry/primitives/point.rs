use crate::codec::geometry::GmlDirectPosition;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
};
use crate::error::Error;
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AsAbstractGeometricPrimitive, Point};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_point(xml_document: &[u8]) -> Result<Point, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, &spans)?;

    let parsed: GmlPoint = de::from_reader(xml_document)?;
    let direct_position: DirectPosition = parsed.pos.try_into()?;

    let point =
        Point::from_abstract_geometric_primitive(abstract_geometric_primitive, direct_position);
    Ok(point)
}

pub fn serialize_point(point: &Point, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_geometric_primitive(point.abstract_geometric_primitive(), formatting)?;

    if let Some(raw) = serialize_inner(GmlPoint::from(point), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(GmlElement::Point.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPoint {
    #[serde(rename(serialize = "gml:pos", deserialize = "pos"))]
    pos: GmlDirectPosition,
}

impl TryFrom<GmlPoint> for Point {
    type Error = Error;

    fn try_from(item: GmlPoint) -> Result<Self, Self::Error> {
        let point = Point::new(item.pos.try_into()?);
        Ok(point)
    }
}

impl From<&Point> for GmlPoint {
    fn from(point: &Point) -> Self {
        Self {
            pos: GmlDirectPosition::from(point.pos()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{deserialize_point, serialize_point};
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::Point;

    #[test]
    fn deserialize_point_with_srs_dimension_and_id() {
        let xml_document = b"<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document).unwrap();

        assert_eq!(result.pos().x(), 678000.9484065345);
        assert_eq!(result.pos().y(), 5403659.060043676);
        assert_eq!(result.pos().z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id() {
        let xml_document = b"<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document).unwrap();

        assert_eq!(result.pos().x(), 678000.9484065345);
        assert_eq!(result.pos().y(), 5403659.060043676);
        assert_eq!(result.pos().z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id_and_srs_dimension() {
        let xml_document = b"<gml:Point>
              <gml:pos>678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document).unwrap();

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
        let xml_node = serialize_point(&point, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert!(xml.contains("1 2 3"));
    }

    #[test]
    fn round_trip_point_without_id() {
        let original = make_point(678000.9484065345, 5403659.060043676, 417.3802376791456);
        let xml_node = serialize_point(&original, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");
        let parsed = deserialize_point(xml.as_ref()).unwrap();

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

        let xml_node = serialize_point(&original, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("id="));
        let parsed = deserialize_point(xml.as_ref()).unwrap();
        assert_eq!(parsed.pos().x(), 10.0);
        assert_eq!(parsed.pos().y(), 20.0);
        assert_eq!(parsed.pos().z(), 30.0);
    }

    #[test]
    fn round_trip_point_from_xml_with_id() {
        let input_xml = b"<gml:Point gml:id=\"test-pt\">\
              <gml:pos srsDimension=\"3\">1 2 3</gml:pos>\
            </gml:Point>";

        let point = deserialize_point(input_xml).unwrap();
        let xml_node = serialize_point(&point, Formatting::Compact).unwrap();
        let output_xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert_eq!(std::str::from_utf8(input_xml).unwrap(), output_xml);
    }

    #[test]
    fn round_trip_point_from_xml() {
        let input_xml = b"<gml:Point>\
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>\
            </gml:Point>";

        let point = deserialize_point(input_xml).unwrap();
        let xml_node = serialize_point(&point, Formatting::Compact).unwrap();
        let output_xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert_eq!(std::str::from_utf8(input_xml).unwrap(), output_xml);
    }

    #[test]
    fn round_trip_point_preserves_float_precision() {
        let x = std::f64::consts::PI;
        let y = std::f64::consts::E;
        let z = std::f64::consts::SQRT_2;
        let original = make_point(x, y, z);

        let xml_node = serialize_point(&original, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");
        let parsed = deserialize_point(xml.as_ref()).unwrap();

        assert_eq!(parsed.pos().x(), x);
        assert_eq!(parsed.pos().y(), y);
        assert_eq!(parsed.pos().z(), z);
    }
}
