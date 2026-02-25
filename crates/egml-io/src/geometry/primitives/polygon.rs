use crate::Error;
use crate::primitives::GmlLinearRingProperty;
use egml_core::model::base::AsAbstractGmlMut;
use egml_core::model::geometry::primitives::{
    AbstractSurface, LinearRing, Polygon, RingPropertyKind,
};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::hash::Hasher;
use tracing::warn;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPolygon {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    pub exterior: GmlLinearRingProperty,

    #[serde(default)]
    pub interior: Vec<GmlLinearRingProperty>,
}

impl TryFrom<GmlPolygon> for Polygon {
    type Error = Error;

    fn try_from(value: GmlPolygon) -> Result<Self, Self::Error> {
        let id = value.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_surface = AbstractSurface::default();
        abstract_surface.set_id(id);

        let exterior: LinearRing = value.exterior.linear_ring.unwrap().try_into()?;
        let interior: Vec<LinearRing> = value
            .interior
            .into_iter()
            .flat_map(|x| x.linear_ring)
            .flat_map(|x| {
                let linear_ring_id = x.id.clone();
                x.try_into()
                    .map_err(|e| {
                        warn!(
                            "Error during parsing of gml:LinearRing with id={:?}: {}",
                            &linear_ring_id, e
                        );
                    })
                    .ok()
            })
            .collect();

        let polygon = Polygon::new(
            abstract_surface,
            Some(RingPropertyKind::LinearRing(exterior)),
            interior
                .into_iter()
                .map(RingPropertyKind::LinearRing)
                .collect(),
        )?;
        Ok(polygon)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlPolygon;
    use egml_core::model::geometry::primitives::Polygon;
    use quick_xml::de;

    #[test]
    fn parsing_basic_polygon() {
        let xml_document = "<gml:Polygon gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_\">
                      <gml:posList>690985.156 5336010.964 530.92 691004.477 5336059.877 530.92 690987.939 5336066.373 530.92 690968.654 5336017.45 530.92 690985.156 5336010.964 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.p848qtslOqDyVvoEINYt\">
                      <gml:posList>690997.492 5336051.391 530.92 690996.582 5336049.058 530.92 690987.295 5336052.68 530.92 690988.205 5336055.013 530.92 690997.492 5336051.391 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.kRRfuBGJHBBSenuutsor\">
                      <gml:posList>690976.045 5336024.363 530.92 690976.805 5336026.31 530.92 690986.204 5336022.644 530.92 690985.444 5336020.697 530.92 690976.045 5336024.363 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                </gml:Polygon>";

        let parsed_geometry: GmlPolygon = de::from_str(xml_document).expect("");
        let p: Polygon = parsed_geometry.try_into().unwrap();

        assert_eq!(p.interior.len(), 2)
    }
}
