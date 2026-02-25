use crate::Error;
use crate::primitives::abstract_ring::GmlRingKind;
use egml_core::model::geometry::primitives::RingPropertyKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlRingProperty {
    #[serde(rename = "$value")]
    pub content: GmlRingKind,
}

impl TryFrom<GmlRingProperty> for RingPropertyKind {
    type Error = Error;

    fn try_from(item: GmlRingProperty) -> Result<Self, Self::Error> {
        match item.content {
            GmlRingKind::LinearRing(x) => x.try_into().map(RingPropertyKind::LinearRing),
            GmlRingKind::Ring(_) => todo!("needs to be implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::abstract_ring::GmlRingKind;
    use crate::primitives::abstract_ring_property::GmlRingProperty;
    use egml_core::model::geometry::primitives::{LinearRing, RingKind};
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_linear_ring() {
        let xml_document = b"<gml:exterior>
   <gml:LinearRing>
      <gml:pos>0.0 0.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 1.0</gml:pos>
      <gml:pos>0.0 0.0 0.0</gml:pos>
   </gml:LinearRing>
</gml:exterior>";

        let result: Result<GmlRingProperty, DeError> = de::from_reader(xml_document.as_ref());
        let gml_ring_kind = result.unwrap().content;
        let ring_kind: RingKind = gml_ring_kind.try_into().unwrap();

        let linear_ring = match ring_kind {
            RingKind::LinearRing(x) => x,
            _ => panic!("expected linear ring"),
        };

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn parsing_ring() {
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
        let gml_ring_kind = result.unwrap().content;

        match gml_ring_kind {
            GmlRingKind::Ring(_) => {} // Success case
            GmlRingKind::LinearRing(_) => panic!("Expected Ring, got LinearRing"),
        }
    }
}
