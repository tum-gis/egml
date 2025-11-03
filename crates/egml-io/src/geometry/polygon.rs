use crate::{Error, GmlLinearRing};
use egml_core::model::base::{AbstractGml, Id};
use egml_core::model::geometry::{LinearRing, Polygon};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::hash::{DefaultHasher, Hash, Hasher};
use tracing::warn;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:surfaceMember")]
pub struct GmlPolygon {
    #[serde(rename = "@id", default)]
    pub id: String,

    pub exterior: GmlLinearRingProperty,

    #[serde(default)]
    pub interior: Vec<GmlLinearRingProperty>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct GmlLinearRingProperty {
    #[serde(rename = "$value")]
    pub linear_ring: Option<GmlLinearRing>,
}

impl TryFrom<GmlPolygon> for Polygon {
    type Error = Error;

    fn try_from(value: GmlPolygon) -> Result<Self, Self::Error> {
        let id: Id = value.id.clone().try_into().ok().unwrap_or_else(|| {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            Id::from_hashed_u64(hasher.finish())
        });
        let abstract_gml = AbstractGml::new(id);

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
                            "Error during parsing of gml:LinearRing with id={}: {}",
                            &linear_ring_id, e
                        );
                    })
                    .ok()
            })
            .collect();

        let polygon = Polygon::new(abstract_gml, exterior, interior)?;
        Ok(polygon)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::polygon::GmlPolygon;
    use egml_core::model::geometry::Polygon;
    use quick_xml::de;

    #[test]
    fn parsing_basic_polygon() {
        let source_text = "<gml:Polygon gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly\">
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

        let parsed_geometry: GmlPolygon = de::from_str(source_text).expect("");
        let p: Polygon = parsed_geometry.try_into().unwrap();

        assert_eq!(p.interior.len(), 2)
    }
}
