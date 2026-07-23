use crate::Error;
use crate::model::base::HasAssociationAttributes;
use crate::model::common::{ApplyTransform, ComputeEnvelope, Triangulate, Triangulation};
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::surface_interpolation::SurfaceInterpolation;
use crate::model::geometry::primitives::{
    AbstractRingProperty, AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut,
};
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
    exterior: Option<AbstractRingProperty>,
    interior: Vec<AbstractRingProperty>,
    interpolation: SurfaceInterpolation,
}

impl PolygonPatch {
    pub fn new(
        exterior: Option<AbstractRingProperty>,
        interior: impl IntoIterator<Item = AbstractRingProperty>,
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
        exterior: Option<AbstractRingProperty>,
        interior: impl IntoIterator<Item = AbstractRingProperty>,
    ) -> Self {
        Self {
            abstract_surface_patch,
            exterior,
            interior: interior.into_iter().collect(),
            interpolation: SurfaceInterpolation::Planar,
        }
    }

    pub fn exterior(&self) -> Option<&AbstractRingProperty> {
        self.exterior.as_ref()
    }

    pub fn set_exterior(&mut self, exterior: AbstractRingProperty) {
        self.exterior = Some(exterior);
    }

    pub fn set_exterior_opt(&mut self, exterior: Option<AbstractRingProperty>) {
        self.exterior = exterior;
    }

    pub fn clear_exterior(&mut self) {
        self.exterior = None;
    }

    pub fn interior(&self) -> &[AbstractRingProperty] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<AbstractRingProperty>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, ring: AbstractRingProperty) {
        self.interior.push(ring);
    }

    pub fn extend_interiors(&mut self, rings: impl IntoIterator<Item = AbstractRingProperty>) {
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
        let exterior_ring = self.exterior.as_ref().ok_or(Error::MissingExteriorRing)?;
        let exterior = exterior_ring
            .object()
            .ok_or_else(|| Error::UnresolvedRingReference {
                href: exterior_ring.href().map(|h| h.to_string()),
            })?
            .area_3d();

        let holes = self
            .interior
            .iter()
            .map(|r| {
                r.object()
                    .ok_or_else(|| Error::UnresolvedRingReference {
                        href: r.href().map(|h| h.to_string()),
                    })
                    .map(|ring| ring.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()?
            .into_iter()
            .sum::<f64>();

        Ok(exterior - holes)
    }
}

impl ApplyTransform for PolygonPatch {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_transform(transform);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_isometry(isometry);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_translation(vector);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_rotation(rotation);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_scale(scale);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for PolygonPatch {
    fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(object) = exterior.object()
            && let Some(e) = object.compute_envelope()
        {
            return Some(e);
        }

        let envelopes = self
            .interior
            .iter()
            .filter_map(|x| x.object())
            .filter_map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
    }
}

impl Triangulate for PolygonPatch {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let exterior = match self.exterior.clone() {
            Some(x) => x,
            None => {
                todo!("triangulate polygon patch with no exterior ring needs to be implemented")
            }
        };

        let surface = triangulate(Some(exterior), self.interior.clone())?;
        Ok(Triangulation::new(surface, Vec::new()))
    }
}
