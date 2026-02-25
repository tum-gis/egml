use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::surface_interpolation::SurfaceInterpolation;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, RingPropertyKind,
    TriangulatedSurface,
};
use crate::util::triangulate::triangulate;

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

        Envelope::from_envelopes(&envelopes.iter().collect::<Vec<_>>())
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
