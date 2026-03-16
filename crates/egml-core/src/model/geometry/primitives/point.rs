use crate::impl_abstract_geometric_primitive_traits;
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// A 0-D geometry that wraps a single [`DirectPosition`].
///
/// Corresponds to `gml:Point` in ISO 19136 §10.3.1.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
    pos: DirectPosition,
}

impl Point {
    /// Creates a new `Point` from a position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::{AbstractGeometricPrimitive, Point};
    ///
    /// let pos = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// let pt = Point::new(AbstractGeometricPrimitive::default(), pos);
    /// assert_eq!(pt.pos().x(), 1.0);
    /// ```
    pub fn new(
        abstract_geometric_primitive: AbstractGeometricPrimitive,
        pos: DirectPosition,
    ) -> Self {
        Self {
            abstract_geometric_primitive,
            pos,
        }
    }

    /// Returns the coordinate of this point.
    pub fn pos(&self) -> &DirectPosition {
        &self.pos
    }

    /// Applies a rigid-body transform to this point in place.
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.pos.apply_transform(m);
    }

    /// Returns a degenerate (zero-volume) envelope at the point's position.
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
