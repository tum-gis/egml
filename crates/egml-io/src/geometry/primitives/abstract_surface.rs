use crate::Error;
use crate::complexes::GmlCompositeSurface;
use crate::primitives::{GmlPolygon, GmlSurface};
use egml_core::model::geometry::primitives::SurfaceKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlSurfaceKind {
    Polygon(GmlPolygon),
    CompositeSurface(GmlCompositeSurface),
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
