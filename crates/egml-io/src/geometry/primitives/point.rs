use crate::GmlDirectPosition;
use crate::error::Error;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut, Id};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractGeometricPrimitive, Point};
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPoint {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(rename(serialize = "gml:pos", deserialize = "pos"))]
    pos: GmlDirectPosition,
}

impl TryFrom<GmlPoint> for Point {
    type Error = Error;

    fn try_from(item: GmlPoint) -> Result<Self, Self::Error> {
        let mut abstract_geometric_primitive = AbstractGeometricPrimitive::default();
        if let Some(id) = item.id {
            let id: Id = id.try_into()?;
            abstract_geometric_primitive.set_id(Some(id));
        }

        Ok(Point::new(
            abstract_geometric_primitive,
            item.pos.try_into()?,
        ))
    }
}

impl From<&Point> for GmlPoint {
    fn from(point: &Point) -> Self {
        Self {
            id: point.id().map(|id| id.to_string()),
            pos: GmlDirectPosition::from(point.pos()),
        }
    }
}

pub fn deserialize_point<T: BufRead>(source: T) -> Result<DirectPosition, Error> {
    let parsed_point: GmlPoint = de::from_reader(source)?;
    parsed_point.pos.try_into()
}

/// Serializes a [`Point`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_point(point: &Point) -> Result<String, Error> {
    let gml_point = GmlPoint::from(point);
    Ok(se::to_string_with_root("gml:Point", &gml_point)?)
}

#[cfg(test)]
mod tests {
    use crate::primitives::{GmlPoint, deserialize_point, serialize_point};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AbstractGeometricPrimitive, Point};
    use quick_xml::de;

    #[test]
    fn deserialize_point_with_srs_dimension_and_id() {
        let xml_document = "<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id() {
        let xml_document = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_point_without_id_and_srs_dimension() {
        let xml_document = "<gml:Point>
              <gml:pos>678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = deserialize_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    fn make_point(x: f64, y: f64, z: f64) -> Point {
        let pos = DirectPosition::new(x, y, z).unwrap();
        Point::new(AbstractGeometricPrimitive::default(), pos)
    }

    #[test]
    fn serialize_point_writes_gml_tags() {
        let point = make_point(1.0, 2.0, 3.0);
        let xml = serialize_point(&point).unwrap();

        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert!(xml.contains("1 2 3"));
    }

    #[test]
    fn round_trip_point_without_id() {
        let original = make_point(678000.9484065345, 5403659.060043676, 417.3802376791456);
        let xml = serialize_point(&original).unwrap();
        let parsed = deserialize_point(xml.as_bytes()).unwrap();

        assert_eq!(parsed.x(), original.pos().x());
        assert_eq!(parsed.y(), original.pos().y());
        assert_eq!(parsed.z(), original.pos().z());
    }

    #[test]
    fn round_trip_point_with_id() {
        let pos = DirectPosition::new(10.0, 20.0, 30.0).unwrap();
        let mut prim = AbstractGeometricPrimitive::default();
        use egml_core::model::base::AsAbstractGmlMut;
        prim.set_id(Some(Id::from_hashed_string("test-point")));
        let original = Point::new(prim, pos);

        let xml = serialize_point(&original).unwrap();

        assert!(xml.contains("id="));
        let parsed = deserialize_point(xml.as_bytes()).unwrap();
        assert_eq!(parsed.x(), 10.0);
        assert_eq!(parsed.y(), 20.0);
        assert_eq!(parsed.z(), 30.0);
    }

    #[test]
    fn round_trip_point_from_xml_with_id() {
        let input_xml = "<gml:Point gml:id=\"test-pt\">\
              <gml:pos srsDimension=\"3\">1 2 3</gml:pos>\
            </gml:Point>";

        let gml_point: GmlPoint = de::from_reader(input_xml.as_bytes()).unwrap();
        let point: Point = gml_point.try_into().unwrap();
        let output_xml = serialize_point(&point).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_point_from_xml() {
        let input_xml = "<gml:Point>\
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>\
            </gml:Point>";

        let gml_point: GmlPoint = de::from_reader(input_xml.as_bytes()).unwrap();
        let point: Point = gml_point.try_into().unwrap();
        let output_xml = serialize_point(&point).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_point_preserves_float_precision() {
        let x = std::f64::consts::PI;
        let y = std::f64::consts::E;
        let z = std::f64::consts::SQRT_2;
        let original = make_point(x, y, z);

        let xml = serialize_point(&original).unwrap();
        let parsed = deserialize_point(xml.as_bytes()).unwrap();

        assert_eq!(parsed.x(), x);
        assert_eq!(parsed.y(), y);
        assert_eq!(parsed.z(), z);
    }
}
