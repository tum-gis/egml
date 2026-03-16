use crate::Error;
use crate::primitives::abstract_ring_property::GmlRingProperty;
use egml_core::model::geometry::primitives::{
    AbstractRing, LinearRing, RingPropertyKind, Triangle,
};
use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangle {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(rename(serialize = "gml:exterior", deserialize = "exterior"))]
    pub exterior: GmlRingProperty,
}

impl TryFrom<GmlTriangle> for Triangle {
    type Error = Error;

    fn try_from(item: GmlTriangle) -> Result<Triangle, Self::Error> {
        let exterior: RingPropertyKind = item.exterior.try_into()?;
        let exterior: LinearRing = match exterior {
            RingPropertyKind::LinearRing(x) => x,
            _ => todo!("needs to be implemented"),
        };

        if exterior.points().len() != 3 {
            return Err(Error::MissingElements(
                "triangle must have exactly 3 points".to_string(),
            ));
        }

        let a = *exterior.points().first().unwrap();
        let b = *exterior.points().get(1).unwrap();
        let c = *exterior.points().get(2).unwrap();

        Ok(Triangle::new(a, b, c)?)
    }
}

impl From<&Triangle> for GmlTriangle {
    fn from(triangle: &Triangle) -> Self {
        let points = vec![triangle.a, triangle.b, triangle.c];
        let linear_ring = LinearRing::new(AbstractRing::default(), points)
            .expect("triangle always yields a valid linear ring");
        Self {
            id: None,
            exterior: GmlRingProperty::from(&linear_ring),
        }
    }
}

/// Serializes a [`Triangle`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_triangle(triangle: &Triangle) -> Result<String, Error> {
    let gml_triangle = GmlTriangle::from(triangle);
    Ok(se::to_string_with_root("gml:Triangle", &gml_triangle)?)
}

#[cfg(test)]
mod tests {
    use super::GmlTriangle;
    use crate::Error;
    use crate::primitives::triangle::serialize_triangle;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::Triangle;
    use quick_xml::de;

    fn make_triangle() -> Triangle {
        let a = DirectPosition::new(1.0, 0.0, 0.0).unwrap();
        let b = DirectPosition::new(0.0, 1.0, 0.0).unwrap();
        let c = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        Triangle::new(a, b, c).unwrap()
    }

    #[test]
    fn deserialize_triangle() {
        let xml_document = b"
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 355.39898681640625 978.8480224609375 2.388849973678589 355.3919982910156 978.8480224609375 2.1084799766540527 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>";

        let parsed_gml: GmlTriangle =
            de::from_reader(xml_document.as_ref()).expect("parsing should work");
        let triangle: Triangle = parsed_gml.try_into().unwrap();

        assert_eq!(triangle.a.x(), 354.0249938964844);
        assert_eq!(triangle.a.y(), 978.864990234375);
        assert_eq!(triangle.a.z(), 2.388849973678589);
    }

    #[test]
    fn serialize_triangle_writes_gml_tags() {
        let triangle = make_triangle();
        let xml = serialize_triangle(&triangle).unwrap();

        assert!(xml.contains("<gml:Triangle"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_triangle_preserves_points() {
        let triangle = make_triangle();
        let xml = serialize_triangle(&triangle).unwrap();

        let gml: GmlTriangle = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: Triangle = gml.try_into().unwrap();

        assert_eq!(recovered.a.x(), triangle.a.x());
        assert_eq!(recovered.a.y(), triangle.a.y());
        assert_eq!(recovered.a.z(), triangle.a.z());
        assert_eq!(recovered.b.x(), triangle.b.x());
        assert_eq!(recovered.b.y(), triangle.b.y());
        assert_eq!(recovered.b.z(), triangle.b.z());
        assert_eq!(recovered.c.x(), triangle.c.x());
        assert_eq!(recovered.c.y(), triangle.c.y());
        assert_eq!(recovered.c.z(), triangle.c.z());
    }

    #[test]
    fn round_trip_triangle_from_xml() {
        let input_xml = "<gml:Triangle>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:Triangle>";

        let gml: GmlTriangle = de::from_reader(input_xml.as_bytes()).unwrap();
        let triangle: Triangle = gml.try_into().unwrap();
        let output_xml = serialize_triangle(&triangle).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_triangle_preserves_float_precision() {
        let a =
            DirectPosition::new(354.0249938964844, 978.864990234375, 2.388849973678589).unwrap();
        let b =
            DirectPosition::new(355.39898681640625, 978.8480224609375, 2.388849973678589).unwrap();
        let c =
            DirectPosition::new(355.3919982910156, 978.8480224609375, 2.1084799766540527).unwrap();
        let triangle = Triangle::new(a, b, c).unwrap();

        let xml = serialize_triangle(&triangle).unwrap();
        let gml: GmlTriangle = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: Result<Triangle, Error> = gml.try_into();
        let recovered = recovered.unwrap();

        assert_eq!(recovered.a.x(), a.x());
        assert_eq!(recovered.a.y(), a.y());
        assert_eq!(recovered.a.z(), a.z());
        assert_eq!(recovered.b.x(), b.x());
        assert_eq!(recovered.c.x(), c.x());
    }
}
