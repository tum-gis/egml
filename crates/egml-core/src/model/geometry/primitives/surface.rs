use crate::Error;
use crate::model::base::AsAbstractGml;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
    AsAbstractSurface, AsAbstractSurfaceMut, SurfacePatchArrayProperty, TriangulatedSurface,
};
use crate::model::geometry::{AsAbstractGeometry, AsAbstractGeometryMut, DirectPosition, Envelope};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    pub(crate) abstract_surface: AbstractSurface,
    patches: SurfacePatchArrayProperty,
}

impl Surface {
    pub fn new(abstract_surface: AbstractSurface, patches: SurfacePatchArrayProperty) -> Self {
        Surface {
            abstract_surface,
            patches,
        }
    }

    fn into_patches(self) -> SurfacePatchArrayProperty {
        self.patches
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let a = self.patches.patches.iter().map(|patch| patch.triangulate());
        let b = a.collect::<Result<Vec<TriangulatedSurface>, Error>>()?;
        TriangulatedSurface::from_triangulated_surfaces(b)
    }

    pub fn compute_envelope(&self) -> Envelope {
        self.patches.compute_envelope()
    }

    pub fn apply_transform(&mut self, _m: &Isometry3<f64>) {
        todo!("needs to be implemented")
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        todo!("needs to be implemented")
    }
}

pub trait AsSurface: AsAbstractSurface {
    fn surface(&self) -> &Surface;

    fn patches(&self) -> &SurfacePatchArrayProperty {
        &self.surface().patches
    }

    fn patches_len(&self) -> usize {
        self.patches().patches_len()
    }
}

pub trait AsSurfaceMut: AsSurface + AsAbstractSurfaceMut {
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
