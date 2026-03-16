use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::surface_interpolation::SurfaceInterpolation;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, RingPropertyKind,
    TriangulatedSurface,
};
use crate::util::triangulate::triangulate;
use nalgebra::Isometry3;

/// A planar polygon used as a surface patch inside a [`Surface`](crate::model::geometry::primitives::Surface).
///
/// `PolygonPatch` has the same geometry as [`Polygon`](crate::model::geometry::primitives::Polygon)
/// but is used exclusively as a building block of a patched surface rather than
/// as a standalone geometry element.  Corresponds to `gml:PolygonPatch` in ISO 19136 §10.5.8.
#[derive(Debug, Clone, PartialEq)]
pub struct PolygonPatch {
    pub(crate) abstract_surface_patch: AbstractSurfacePatch,
    exterior: Option<RingPropertyKind>,
    interior: Vec<RingPropertyKind>,
    interpolation: SurfaceInterpolation,
}

impl PolygonPatch {
    pub fn new(
        abstract_surface_patch: AbstractSurfacePatch,
        exterior: Option<RingPropertyKind>,
        interior: Vec<RingPropertyKind>,
    ) -> Self {
        Self {
            abstract_surface_patch,
            exterior,
            interior,
            interpolation: SurfaceInterpolation::Planar,
        }
    }

    pub fn exterior(&self) -> Option<&RingPropertyKind> {
        self.exterior.as_ref()
    }

    pub fn interior(&self) -> &[RingPropertyKind] {
        &self.interior
    }

    pub fn interpolation(&self) -> SurfaceInterpolation {
        self.interpolation
    }

    pub fn area(&self) -> f64 {
        todo!("needs to be implemented")
    }

    pub fn compute_envelope(&self) -> Envelope {
        if let Some(exterior) = &self.exterior {
            return exterior.compute_envelope();
        }

        let envelopes = self
            .interior
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
            .expect("PolygonPatch must have at least one exterior ring or interior ring")
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let exterior = match self.exterior.clone() {
            Some(x) => x,
            None => {
                todo!("triangulate polygon patch with no exterior ring needs to be implemented")
            }
        };

        triangulate(Some(exterior), self.interior.clone())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_transform(m);
        }
        self.interior.iter_mut().for_each(|x| x.apply_transform(m));
    }
}

impl AsAbstractSurfacePatch for PolygonPatch {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        &self.abstract_surface_patch
    }
}

impl AsAbstractSurfacePatchMut for PolygonPatch {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        &mut self.abstract_surface_patch
    }
}
