use crate::Error;
use crate::model::common::{ApplyTransform, ComputeEnvelope, Triangulate, Triangulation};
use crate::model::geometry::primitives::surface_interpolation::SurfaceInterpolation;
use crate::model::geometry::primitives::{
    AbstractRingKind, AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut,
};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::util::triangulate::triangulate;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// A planar polygon used as a surface patch inside a [`Surface`](crate::model::geometry::primitives::Surface).
///
/// `PolygonPatch` has the same geometry as [`Polygon`](crate::model::geometry::primitives::Polygon)
/// but is used exclusively as a building block of a patched surface rather than
/// as a standalone geometry element.  Corresponds to `gml:PolygonPatch` in [OGC 07-036 §10.5.12.4](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct PolygonPatch {
    pub abstract_surface_patch: AbstractSurfacePatch,
    exterior: Option<AbstractRingKind>,
    interior: Vec<AbstractRingKind>,
    interpolation: SurfaceInterpolation,
}

impl PolygonPatch {
    pub fn new(
        exterior: Option<AbstractRingKind>,
        interior: impl IntoIterator<Item = AbstractRingKind>,
    ) -> Self {
        Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
            interior: interior.into_iter().collect(),
            interpolation: SurfaceInterpolation::Planar,
        }
    }

    pub fn from_abstract_surface_patch(
        abstract_surface_patch: AbstractSurfacePatch,
        exterior: Option<AbstractRingKind>,
        interior: impl IntoIterator<Item = AbstractRingKind>,
    ) -> Self {
        Self {
            abstract_surface_patch,
            exterior,
            interior: interior.into_iter().collect(),
            interpolation: SurfaceInterpolation::Planar,
        }
    }

    pub fn exterior(&self) -> Option<&AbstractRingKind> {
        self.exterior.as_ref()
    }

    pub fn set_exterior(&mut self, exterior: AbstractRingKind) {
        self.exterior = Some(exterior);
    }

    pub fn set_exterior_opt(&mut self, exterior: Option<AbstractRingKind>) {
        self.exterior = exterior;
    }

    pub fn clear_exterior(&mut self) {
        self.exterior = None;
    }

    pub fn interior(&self) -> &[AbstractRingKind] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<AbstractRingKind>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, ring: AbstractRingKind) {
        self.interior.push(ring);
    }

    pub fn extend_interiors(&mut self, rings: impl IntoIterator<Item = AbstractRingKind>) {
        self.interior.extend(rings);
    }

    pub fn interpolation(&self) -> SurfaceInterpolation {
        self.interpolation
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

impl PolygonPatch {
    pub fn area_3d(&self) -> Result<f64, Error> {
        let exterior = self
            .exterior
            .as_ref()
            .ok_or(Error::MissingExteriorRing)?
            .area_3d();
        let holes = self.interior.iter().map(|r| r.area_3d()).sum::<f64>();

        Ok(exterior - holes)
    }

    /// Returns all positions of the exterior ring followed by those of each interior ring.
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.exterior
            .iter()
            .chain(&self.interior)
            .flat_map(|ring| ring.points())
            .collect()
    }
}

impl ApplyTransform for PolygonPatch {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_transform(transform);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_isometry(isometry);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_translation(vector);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_rotation(rotation);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_scale(scale);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_scale(scale));
    }
}

impl ComputeEnvelope for PolygonPatch {
    fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(e) = exterior.compute_envelope()
        {
            return Some(e);
        }

        let envelopes = self
            .interior
            .iter()
            .filter_map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
    }
}

impl Triangulate for PolygonPatch {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let surface = triangulate(self.exterior.clone(), self.interior.clone())?;
        Ok(Triangulation::new(surface, Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::primitives::LinearRing;

    #[test]
    fn triangulate_no_exterior_ring() {
        let patch = PolygonPatch::new(None, []);
        assert_eq!(patch.triangulate().err(), Some(Error::MissingExteriorRing));
    }

    fn ring(points: &[[f64; 3]]) -> AbstractRingKind {
        let points = points
            .iter()
            .map(|&[x, y, z]| DirectPosition::new(x, y, z).unwrap());
        AbstractRingKind::LinearRing(LinearRing::new(points).unwrap())
    }

    #[test]
    fn points_exterior_then_interior() {
        let exterior = ring(&[[0., 0., 0.], [4., 0., 0.], [4., 4., 0.], [0., 4., 0.]]);
        let hole = ring(&[[1., 1., 0.], [2., 1., 0.], [2., 2., 0.]]);
        let patch = PolygonPatch::new(Some(exterior.clone()), [hole.clone()]);

        let expected: Vec<&DirectPosition> =
            exterior.points().iter().chain(hole.points()).collect();
        assert_eq!(patch.points(), expected);
    }

    #[test]
    fn points_without_exterior_returns_interior_only() {
        let hole = ring(&[[1., 1., 0.], [2., 1., 0.], [2., 2., 0.]]);
        let patch = PolygonPatch::new(None, [hole.clone()]);
        assert_eq!(patch.points(), hole.points().iter().collect::<Vec<_>>());
    }
}
