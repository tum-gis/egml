use crate::error::GmlIoError;
use egml_core::{LinearRing, MultiSurface};
use quick_xml::de;

use crate::{LinearRingElement, PosListElement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:MultiSurface")]
struct MultiSurfaceElement {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "$value")]
    members: Vec<SurfaceMemberElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:surfaceMember")]
struct SurfaceMemberElement {
    #[serde(rename = "$value")]
    polygon: Option<PolygonElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:surfaceMember")]
struct PolygonElement {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "$value")]
    exterior: Option<ExteriorElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:exterior")]
struct ExteriorElement {
    #[serde(rename = "$value")]
    linear_ring: Option<LinearRingElement>,
}

pub fn parse_multi_surface(source_text: &str) -> Result<MultiSurface, GmlIoError> {
    let parsed_geometry: MultiSurfaceElement = de::from_str(source_text)?;

    let position_list_elements: Vec<&PosListElement> = parsed_geometry
        .members
        .iter()
        .flat_map(|x| &x.polygon)
        .flat_map(|x| &x.exterior)
        .flat_map(|x| &x.linear_ring)
        .flat_map(|x| &x.pos_list)
        .collect();

    let linear_rings: Vec<LinearRing> = position_list_elements
        .iter()
        .map(|x| LinearRing::from(*x))
        .collect();

    let multi_surface = MultiSurface::new(linear_rings)?;
    Ok(multi_surface)
}
