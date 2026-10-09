use crate::Error;
use crate::impl_has_geometry_type;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractSurface, AbstractSurfacePatchKind, AsAbstractSurface, AsAbstractSurfaceMut,
    TriangulatedSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// A 2-D geometry composed of one or more surface patches.
///
/// Corresponds to `gml:Surface` in [OGC 07-036 §10.5.10](https://docs.ogc.org/is/07-036/07-036.pdf).  Patches are stored as
/// a list of [`AbstractSurfacePatchKind`] and may be of mixed kinds (polygons, triangles).
#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    pub abstract_surface: AbstractSurface,
    patches: Vec<AbstractSurfacePatchKind>,
}

impl Surface {
    /// Creates a new `Surface` from a patch array.
    pub fn new(patches: Vec<AbstractSurfacePatchKind>) -> Self {
        Surface {
            abstract_surface: AbstractSurface::default(),
            patches,
        }
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        patches: Vec<AbstractSurfacePatchKind>,
    ) -> Self {
        Self {
            abstract_surface,
            patches,
        }
    }

    pub fn patches(&self) -> &[AbstractSurfacePatchKind] {
        &self.patches
    }
}

/// Object-safe read accessor for [`Surface`] data.
pub trait AsSurface: AsAbstractSurface {
    /// Returns a reference to the underlying [`Surface`].
    fn surface(&self) -> &Surface;

    /// Returns the patch array of this surface.
    fn patches(&self) -> &[AbstractSurfacePatchKind] {
        &self.surface().patches
    }

    fn patches_len(&self) -> usize {
        self.patches().len()
    }
}

/// Mutable companion to [`AsSurface`].
pub trait AsSurfaceMut: AsSurface + AsAbstractSurfaceMut {
    /// Returns a mutable reference to the underlying [`Surface`].
    fn surface_mut(&mut self) -> &mut Surface;

    fn patches_mut(&mut self) -> &mut Vec<AbstractSurfacePatchKind> {
        &mut self.surface_mut().patches
    }
}

impl AsSurface for Surface {
    fn surface(&self) -> &Surface {
        self
    }
}

impl AsSurfaceMut for Surface {
    fn surface_mut(&mut self) -> &mut Surface {
        self
    }
}

#[macro_export]
macro_rules! impl_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_surface_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractSurface for $type {
            fn abstract_surface(&self) -> &$crate::model::geometry::primitives::AbstractSurface {
                &<$type as $crate::model::geometry::primitives::AsSurface>::surface(self)
                    .abstract_surface
            }
        }
    };
}

#[macro_export]
macro_rules! impl_surface_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_surface_mut_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractSurfaceMut for $type {
            fn abstract_surface_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractSurface {
                &mut <$type as $crate::model::geometry::primitives::AsSurfaceMut>::surface_mut(self)
                    .abstract_surface
            }
        }
    };
}

impl_surface_traits!(Surface);
impl_surface_mut_traits!(Surface);
impl_has_geometry_type!(Surface, Surface);

impl Surface {
    pub(crate) fn into_patches(self) -> Vec<AbstractSurfacePatchKind> {
        self.patches
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.patches
            .iter()
            .map(|p| p.area_3d())
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }

    /// Returns the positions of all patches, in patch order.
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.patches.iter().flat_map(|p| p.points()).collect()
    }
}

impl IterGeometries for Surface {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()))
    }
}

impl ApplyTransform for Surface {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.patches
            .iter_mut()
            .for_each(|x| x.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.patches
            .iter_mut()
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.patches
            .iter_mut()
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.patches
            .iter_mut()
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.patches.iter_mut().for_each(|x| x.apply_scale(scale));
    }
}

impl ComputeEnvelope for Surface {
    /// Returns the union of the bounding boxes of all patches.
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .patches
            .iter()
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl Triangulate for Surface {
    /// Patches that fail to triangulate individually (e.g. a degenerate ring) are
    /// skipped rather than failing the whole surface; see their errors via
    /// [`Triangulation::skipped`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if no patch could be triangulated.
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let mut surfaces = Vec::new();
        let mut skipped = Vec::new();

        for patch in &self.patches {
            match patch.triangulate() {
                Ok(triangulation) => {
                    let (surface, nested_skipped) = triangulation.into_parts();
                    surfaces.push(surface);
                    skipped.extend(nested_skipped);
                }
                Err(error) => {
                    skipped.push(error);
                }
            }
        }

        let combined = TriangulatedSurface::from_triangulated_surfaces(surfaces)?;
        Ok(Triangulation::new(combined, skipped))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfacePatchKind, LinearRing, PolygonPatch, Triangle,
    };

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    #[test]
    fn points_collects_all_patches() {
        let square = LinearRing::new([
            pos(0., 0., 0.),
            pos(1., 0., 0.),
            pos(1., 1., 0.),
            pos(0., 1., 0.),
        ])
        .unwrap();
        let patch = PolygonPatch::new(Some(AbstractRingKind::LinearRing(square)), []);
        let triangle =
            Triangle::from_points(pos(0., 0., 1.), pos(1., 0., 1.), pos(0., 1., 1.)).unwrap();
        let surface = Surface::new(vec![
            AbstractSurfacePatchKind::PolygonPatch(patch),
            AbstractSurfacePatchKind::Triangle(triangle),
        ]);

        let points = surface.points();
        assert_eq!(points.len(), 7);
        assert_eq!(points[0], &pos(0., 0., 0.));
        assert_eq!(points[4], &pos(0., 0., 1.));
    }
}
