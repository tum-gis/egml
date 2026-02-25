use crate::impl_abstract_geometric_primitive_traits;
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};
use crate::model::geometry::{AsAbstractGeometry, DirectPosition, Envelope};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
    pos: DirectPosition,
}

impl Point {
    pub fn new(
        abstract_geometric_primitive: AbstractGeometricPrimitive,
        pos: DirectPosition,
    ) -> Self {
        Self {
            abstract_geometric_primitive,
            pos,
        }
    }

    pub fn pos(&self) -> &DirectPosition {
        &self.pos
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.pos.apply_transform(m);
    }

    pub fn compute_envelope(&self) -> Envelope {
        Envelope::new_unchecked(self.pos, self.pos)
    }
}

impl AsAbstractGeometricPrimitive for Point {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive {
        &self.abstract_geometric_primitive
    }
}

impl AsAbstractGeometricPrimitiveMut for Point {
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive {
        &mut self.abstract_geometric_primitive
    }
}

impl_abstract_geometric_primitive_traits!(Point);
