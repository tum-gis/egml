use crate::GmlDirectPosition;
use crate::error::Error;
use egml_core::model::base::{AsAbstractGmlMut, Id};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractGeometricPrimitive, Point};
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPoint {
    #[serde(rename = "@id")]
    id: Option<String>,

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

pub fn parse_point<T: BufRead>(source: T) -> Result<DirectPosition, Error> {
    let parsed_point: GmlPoint = de::from_reader(source)?;
    parsed_point.pos.try_into()
}

#[cfg(test)]
mod tests {
    use crate::primitives::parse_point;

    #[test]
    fn parsing_point() {
        let xml_document = "<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = parse_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    #[test]
    fn parsing_point_without_id() {
        let xml_document = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = parse_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    #[test]
    fn parsing_point_without_id_and_dimension() {
        let xml_document = "<gml:Point>
              <gml:pos>678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = parse_point(xml_document.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }
}
