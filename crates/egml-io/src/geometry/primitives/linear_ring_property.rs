use crate::primitives::GmlLinearRing;
use egml_core::model::geometry::primitives::LinearRing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRingProperty {
    #[serde(
        rename(serialize = "gml:LinearRing", deserialize = "LinearRing"),
        skip_serializing_if = "Option::is_none"
    )]
    pub linear_ring: Option<GmlLinearRing>,
}

impl From<&LinearRing> for GmlLinearRingProperty {
    fn from(ring: &LinearRing) -> Self {
        Self {
            linear_ring: Some(GmlLinearRing::from(ring)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlLinearRingProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AbstractRing, LinearRing};
    use quick_xml::{de, se};

    fn make_ring() -> LinearRing {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        LinearRing::new(AbstractRing::default(), points).unwrap()
    }

    #[test]
    fn serialize_linear_ring_property_writes_gml_linear_ring_element() {
        let ring = make_ring();
        let prop = GmlLinearRingProperty::from(&ring);
        let xml = se::to_string_with_root("gml:exterior", &prop).unwrap();

        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_linear_ring_property_from_xml() {
        let input_xml = "<gml:exterior>\
            <gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing>\
            </gml:exterior>";

        let gml: GmlLinearRingProperty = de::from_reader(input_xml.as_bytes()).unwrap();
        let output_xml = se::to_string_with_root("gml:exterior", &gml).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
