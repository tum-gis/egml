use crate::{Error, GmlLinearRing};
use egml_core::model::base::{Gml, Id};
use egml_core::model::geometry::{LinearRing, Polygon};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::hash::{DefaultHasher, Hash, Hasher};

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
        let gml = Gml::new(id);

        let exterior: LinearRing = value.exterior.linear_ring.unwrap().try_into()?;
        //let interior: Vec<LinearRing> = self.interior.iter().map(|x| x.linear_ring.unwrap().to_linear_ring()).collect();

        let polygon = Polygon::new(gml, exterior, Vec::new())?;
        Ok(polygon)
    }
}
