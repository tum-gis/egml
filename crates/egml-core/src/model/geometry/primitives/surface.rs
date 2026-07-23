use crate::Error;
use crate::impl_has_geometry_type;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractSurface, AbstractSurfacePatchArrayProperty, AsAbstractSurface, AsAbstractSurfaceMut,
    TriangulatedSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// A 2-D geometry composed of one or more surface patches.
///
/// Corresponds to `gml:Surface` in [OGC 07-036 §10.5.10](https://docs.ogc.org/is/07-036/07-036.pdf).  Patches are stored as
/// a [`AbstractSurfacePatchArrayProperty`] and may be of mixed kinds (polygons, triangles).
#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    pub abstract_surface: AbstractSurface,
    patches: AbstractSurfacePatchArrayProperty,
}

impl Surface {
    /// Creates a new `Surface` from a patch array.
    pub fn new(patches: AbstractSurfacePatchArrayProperty) -> Self {
        Surface {
            abstract_surface: AbstractSurface::default(),
            patches,
        }
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        patches: AbstractSurfacePatchArrayProperty,
    ) -> Self {
        Self {
            abstract_surface,
            patches,
        }
    }

    pub fn patches(&self) -> &AbstractSurfacePatchArrayProperty {
        &self.patches
    }
}

/// Object-safe read accessor for [`Surface`] data.
pub trait AsSurface: AsAbstractSurface {
    /// Returns a reference to the underlying [`Surface`].
    fn surface(&self) -> &Surface;

    /// Returns the patch array of this surface.
    fn patches(&self) -> &AbstractSurfacePatchArrayProperty {
        &self.surface().patches
    }

    fn patches_len(&self) -> usize {
        self.patches().objects_len()
    }
}

/// Mutable companion to [`AsSurface`].
pub trait AsSurfaceMut: AsSurface + AsAbstractSurfaceMut {
    /// Returns a mutable reference to the underlying [`Surface`].
    fn surface_mut(&mut self) -> &mut Surface;

    fn patches_mut(&mut self) -> &mut AbstractSurfacePatchArrayProperty {
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
    pub(crate) fn into_patches(self) -> AbstractSurfacePatchArrayProperty {
        self.patches
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.patches.area_3d()
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        todo!("needs to be implemented")
    }
}

impl IterGeometries for Surface {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()))
    }
}

impl ApplyTransform for Surface {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.patches.apply_transform(transform)
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.patches.apply_isometry(isometry)
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.patches.apply_translation(vector)
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.patches.apply_rotation(rotation)
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.patches.apply_scale(scale)
    }
}

impl ComputeEnvelope for Surface {
    /// Returns the union of the bounding boxes of all patches.
    fn compute_envelope(&self) -> Option<Envelope> {
        self.patches.compute_envelope()
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

        for patch in self.patches.objects() {
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
