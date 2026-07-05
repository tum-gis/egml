use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::surface_interpolation::SurfaceInterpolation;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, RingProperty,
    TriangulatedSurface,
};
use crate::util::triangulate::triangulate;
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// A planar polygon used as a surface patch inside a [`Surface`](crate::model::geometry::primitives::Surface).
///
/// `PolygonPatch` has the same geometry as [`Polygon`](crate::model::geometry::primitives::Polygon)
/// but is used exclusively as a building block of a patched surface rather than
/// as a standalone geometry element.  Corresponds to `gml:PolygonPatch` in [OGC 07-036 §10.5.12.4](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct PolygonPatch {
    pub(crate) abstract_surface_patch: AbstractSurfacePatch,
    exterior: Option<RingProperty>,
    interior: Vec<RingProperty>,
    interpolation: SurfaceInterpolation,
}

impl PolygonPatch {
    pub fn new(
        exterior: Option<RingProperty>,
        interior: impl IntoIterator<Item = RingProperty>,
    ) -> Self {
        Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
            interior: interior.into_iter().collect(),
            interpolation: SurfaceInterpolation::Planar,
        }
    }

    pub fn exterior(&self) -> Option<&RingProperty> {
        self.exterior.as_ref()
    }

    pub fn set_exterior(&mut self, exterior: Option<RingProperty>) {
        self.exterior = exterior;
    }

    pub fn interior(&self) -> &[RingProperty] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<RingProperty>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, ring: RingProperty) {
        self.interior.push(ring);
    }

    pub fn extend_interiors(&mut self, rings: impl IntoIterator<Item = RingProperty>) {
        self.interior.extend(rings);
    }

    pub fn interpolation(&self) -> SurfaceInterpolation {
        self.interpolation
    }
}

impl PolygonPatch {
    pub fn area_3d(&self) -> Result<f64, Error> {
        let exterior_ring = self.exterior.as_ref().ok_or(Error::MissingExteriorRing)?;
        let exterior = exterior_ring
            .object
            .as_ref()
            .ok_or_else(|| Error::UnresolvedRingReference {
                href: exterior_ring.href.clone(),
            })?
            .area_3d();

        let holes = self
            .interior
            .iter()
            .map(|r| {
                r.object
                    .as_ref()
                    .ok_or_else(|| Error::UnresolvedRingReference {
                        href: r.href.clone(),
                    })
                    .map(|ring| ring.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()?
            .into_iter()
            .sum::<f64>();

        Ok(exterior - holes)
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(object) = exterior.object.as_ref()
            && let e = object.compute_envelope()
        {
            return Some(e);
        }

        let envelopes = self
            .interior
            .iter()
            .filter_map(|x| x.object.as_ref())
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
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
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object.as_mut()
        {
            object.apply_transform(m);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object.as_mut() {
                object.apply_transform(m);
            }
        });
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
