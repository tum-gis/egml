use crate::Error;
use crate::primitives::GmlLinearRing;
use crate::primitives::ring::GmlRing;
use egml_core::model::geometry::primitives::RingKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlRingKind {
    LinearRing(GmlLinearRing),
    Ring(GmlRing),
}

impl TryFrom<GmlRingKind> for RingKind {
    type Error = Error;

    fn try_from(item: GmlRingKind) -> Result<Self, Self::Error> {
        let ring_kind = match item {
            GmlRingKind::LinearRing(x) => Self::LinearRing(x.try_into()?),
            GmlRingKind::Ring(_x) => todo!("needs to be implemented"),
        };
        Ok(ring_kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::abstract_ring::GmlRingKind;
    use egml_core::model::geometry::primitives::RingKind;
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_linear_ring() {
        let xml_document = b"<gml:LinearRing>
      <gml:pos>0.0 0.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 1.0</gml:pos>
      <gml:pos>0.0 0.0 0.0</gml:pos>
   </gml:LinearRing>";

        let result: Result<GmlRingKind, DeError> = de::from_reader(xml_document.as_ref());
        let gml_linear_ring = result.expect("should work");
        let ring_kind: RingKind = gml_linear_ring.try_into().unwrap();

        assert_eq!(ring_kind.points().len(), 3);
    }

    #[test]
    fn parsing_ring() {
        let xml_document = b"<gml:Ring>
       <gml:curveMember>
          <gml:LineString>
              <gml:pos>0.0 0.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 1.0</gml:pos>
              <gml:pos>0.0 0.0 0.0</gml:pos>
          </gml:LineString>
       </gml:curveMember>
    </gml:Ring>";

        let result: Result<GmlRingKind, DeError> = de::from_reader(xml_document.as_ref());
        println!("{:?}", result);
        //let gml_linear_ring = result.unwrap().linear_ring.expect("should work");
        //let linear_ring: LinearRing = gml_linear_ring.try_into().unwrap();

        //assert_eq!(linear_ring.points().len(), 3);
    }
}
