use crate::{Error, GmlLinearRing};
use egml_core::{LinearRing, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "gml:surfaceMember")]
pub struct GmlPolygon {
    #[serde(rename = "@id", default)]
    pub id: String,

    pub exterior: GmlLinearRingProperty,

    #[serde(default)]
    pub interior: Vec<GmlLinearRingProperty>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GmlLinearRingProperty {
    #[serde(rename = "$value")]
    pub linear_ring: Option<GmlLinearRing>,
}

impl GmlPolygon {
    pub fn to_polygon(self) -> Result<Polygon, Error> {
        let exterior: LinearRing = self.exterior.linear_ring.unwrap().to_linear_ring()?;
        //let interior: Vec<LinearRing> = self.interior.iter().map(|x| x.linear_ring.unwrap().to_linear_ring()).collect();

        let polygon = Polygon::new(self.id, exterior, Vec::new())?;
        Ok(polygon)
    }
}
