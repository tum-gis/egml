use crate::Error;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, SurfacePatchArrayProperty,
    TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// A 2-D geometry composed of one or more surface patches.
///
/// Corresponds to `gml:Surface` in ISO 19136 §10.5.3.  Patches are stored as
/// a [`SurfacePatchArrayProperty`] and may be of mixed kinds (polygons, triangles).
#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    pub(crate) abstract_surface: AbstractSurface,
    patches: SurfacePatchArrayProperty,
}

impl Surface {
    /// Creates a new `Surface` from its abstract base and patch array.
    pub fn new(abstract_surface: AbstractSurface, patches: SurfacePatchArrayProperty) -> Self {
        Surface {
            abstract_surface,
            patches,
        }
    }

    pub(crate) fn into_patches(self) -> SurfacePatchArrayProperty {
        self.patches
    }

    /// Decomposes this surface into a [`TriangulatedSurface`] by triangulating each patch.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TriangulationFailed`] if any patch cannot be triangulated.
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let a = self.patches.patches.iter().map(|patch| patch.triangulate());
        let b = a.collect::<Result<Vec<TriangulatedSurface>, Error>>()?;
        TriangulatedSurface::from_triangulated_surfaces(b)
    }

    /// Returns the union of the bounding boxes of all patches.
    pub fn compute_envelope(&self) -> Envelope {
        self.patches.compute_envelope()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.patches.apply_transform(m)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        todo!("needs to be implemented")
    }
}

/// Object-safe read accessor for [`Surface`] data.
pub trait AsSurface: AsAbstractSurface {
    /// Returns a reference to the underlying [`Surface`].
    fn surface(&self) -> &Surface;

    /// Returns the patch array of this surface.
    fn patches(&self) -> &SurfacePatchArrayProperty {
        &self.surface().patches
    }

    fn patches_len(&self) -> usize {
        self.patches().patches_len()
    }
}

/// Mutable companion to [`AsSurface`].
pub trait AsSurfaceMut: AsSurface + AsAbstractSurfaceMut {
    /// Returns a mutable reference to the underlying [`Surface`].
    fn surface_mut(&mut self) -> &mut Surface;

    fn patches_mut(&mut self) -> &mut SurfacePatchArrayProperty {
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_surface_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractSurface for $type {
            fn abstract_surface(&self) -> &$crate::model::geometry::primitives::AbstractSurface {
                use $crate::model::geometry::primitives::AsSurface;
                &self.surface().abstract_surface
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractSurfaceMut for $type {
            fn abstract_surface_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractSurface {
                use $crate::model::geometry::primitives::AsSurfaceMut;
                &mut self.surface_mut().abstract_surface
            }
        }
    };
}

impl_surface_traits!(Surface);
