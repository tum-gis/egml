use crate::Error;
use crate::primitives::linear_ring::GmlLinearRing;
use crate::primitives::ring_kind::GmlRingKind;
use egml_core::model::geometry::primitives::{LinearRing, RingProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlRingProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(rename = "$value")]
    pub object: Option<GmlRingKind>,
}

impl TryFrom<GmlRingProperty> for RingProperty {
    type Error = Error;

    fn try_from(item: GmlRingProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&LinearRing> for GmlRingProperty {
    fn from(ring: &LinearRing) -> Self {
        Self {
            href: None,
            object: Some(GmlRingKind::LinearRing(GmlLinearRing::from(ring))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::ring_kind::GmlRingKind;
    use crate::primitives::ring_property::GmlRingProperty;
    use egml_core::model::geometry::primitives::RingKind;
    use quick_xml::{DeError, de};

    #[test]
    fn deserialize_ring_property_as_linear_ring() {
        let xml_document = b"<gml:exterior>
   <gml:LinearRing>
      <gml:pos>0.0 0.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 1.0</gml:pos>
      <gml:pos>0.0 0.0 0.0</gml:pos>
   </gml:LinearRing>
</gml:exterior>";

        let result: Result<GmlRingProperty, DeError> = de::from_reader(xml_document.as_ref());
        let gml_ring_kind = result.unwrap().object.expect("should be there");
        let ring_kind: RingKind = gml_ring_kind.try_into().unwrap();

        let RingKind::LinearRing(linear_ring) = ring_kind else {
            panic!("expected LinearRing variant");
        };

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_ring_property_as_ring() {
        let xml_document = b"<gml:exterior>
   <gml:Ring>
       <gml:curveMember>
          <gml:LineString>
              <gml:pos>0.0 0.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 1.0</gml:pos>
              <gml:pos>0.0 0.0 0.0</gml:pos>
          </gml:LineString>
       </gml:curveMember>
    </gml:Ring>
</gml:exterior>";

        let result: Result<GmlRingProperty, DeError> = de::from_reader(xml_document.as_ref());
        let gml_ring_kind = result.unwrap().object.expect("should be there");

        match gml_ring_kind {
            GmlRingKind::Ring(_) => {} // Success case
            GmlRingKind::LinearRing(_) => panic!("Expected Ring, got LinearRing"),
        }
    }
}
