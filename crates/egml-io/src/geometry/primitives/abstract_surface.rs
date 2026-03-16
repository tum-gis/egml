use crate::Error;
use crate::complexes::GmlCompositeSurface;
use crate::primitives::{GmlPolygon, GmlSurface};
use egml_core::model::geometry::primitives::SurfaceKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlSurfaceKind {
    #[serde(rename(serialize = "gml:Polygon", deserialize = "Polygon"))]
    Polygon(GmlPolygon),
    #[serde(rename(serialize = "gml:CompositeSurface", deserialize = "CompositeSurface"))]
    CompositeSurface(GmlCompositeSurface),
    #[serde(rename(serialize = "gml:Surface", deserialize = "Surface"))]
    Surface(GmlSurface),
}

impl TryFrom<GmlSurfaceKind> for SurfaceKind {
    type Error = Error;

    fn try_from(item: GmlSurfaceKind) -> Result<Self, Self::Error> {
        let surface_kind = match item {
            GmlSurfaceKind::Polygon(x) => SurfaceKind::Polygon(x.try_into()?),
            GmlSurfaceKind::CompositeSurface(x) => SurfaceKind::CompositeSurface(x.try_into()?),
            GmlSurfaceKind::Surface(x) => SurfaceKind::Surface(x.try_into()?),
        };
        Ok(surface_kind)
    }
}

impl From<&SurfaceKind> for GmlSurfaceKind {
    fn from(kind: &SurfaceKind) -> Self {
        match kind {
            SurfaceKind::Polygon(p) => GmlSurfaceKind::Polygon(GmlPolygon::from(p)),
            SurfaceKind::Surface(s) => GmlSurfaceKind::Surface(GmlSurface::from(s)),
            SurfaceKind::CompositeSurface(c) => {
                GmlSurfaceKind::CompositeSurface(GmlCompositeSurface::from(c))
            }
        }
    }
}
