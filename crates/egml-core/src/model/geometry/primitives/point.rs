use crate::model::common::{ApplyTransform, ComputeEnvelope, IterGeometries};
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{
    impl_abstract_geometric_primitive_mut_traits, impl_abstract_geometric_primitive_traits,
    impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// A 0-D geometry that wraps a single [`DirectPosition`].
///
/// Corresponds to `gml:Point` in [OGC 07-036 §10.3.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point {
    pub abstract_geometric_primitive: AbstractGeometricPrimitive,
    pos: DirectPosition,
}

impl Point {
    /// Creates a new `Point` from a position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::Point;
    ///
    /// let pos = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// let pt = Point::new(pos);
    /// assert_eq!(pt.pos().x(), 1.0);
    /// ```
    pub fn new(direct_position: DirectPosition) -> Self {
        Self {
            abstract_geometric_primitive: AbstractGeometricPrimitive::default(),
            pos: direct_position,
        }
    }

    pub fn from_abstract_geometric_primitive(
        abstract_geometric_primitive: AbstractGeometricPrimitive,
        direct_position: DirectPosition,
    ) -> Self {
        Self {
            abstract_geometric_primitive,
            pos: direct_position,
        }
    }

    /// Returns the coordinate of this point.
    pub fn pos(&self) -> &DirectPosition {
        &self.pos
    }

    pub fn set_pos(&mut self, pos: DirectPosition) {
        self.pos = pos;
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
impl_abstract_geometric_primitive_mut_traits!(Point);
impl_has_geometry_type!(Point, Point);

impl ApplyTransform for Point {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.pos.apply_transform(transform);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.pos.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.pos.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.pos.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.pos.apply_scale(scale);
    }
}

impl ComputeEnvelope for Point {
    /// Returns a degenerate (zero-volume) envelope at the point's position.
    fn compute_envelope(&self) -> Option<Envelope> {
        Some(Envelope::new_unchecked(self.pos, self.pos))
    }
}

impl IterGeometries for Point {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()))
    }
}
