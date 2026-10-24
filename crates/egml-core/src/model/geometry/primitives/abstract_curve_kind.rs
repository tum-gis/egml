use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries,
};
use crate::model::geometry::primitives::{
    AbstractCurve, AbstractRingKind, AsAbstractCurve, AsAbstractCurveMut, LineString,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{impl_abstract_curve_mut_traits, impl_abstract_curve_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractCurveKind {
    LineString(LineString),
    AbstractRingKind(AbstractRingKind),
}

impl AsAbstractCurve for AbstractCurveKind {
    fn abstract_curve(&self) -> &AbstractCurve {
        match self {
            AbstractCurveKind::LineString(x) => x.abstract_curve(),
            AbstractCurveKind::AbstractRingKind(x) => x.abstract_curve(),
        }
    }
}

impl AsAbstractCurveMut for AbstractCurveKind {
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve {
        match self {
            AbstractCurveKind::LineString(x) => x.abstract_curve_mut(),
            AbstractCurveKind::AbstractRingKind(x) => x.abstract_curve_mut(),
        }
    }
}

impl_abstract_curve_traits!(AbstractCurveKind);
impl_abstract_curve_mut_traits!(AbstractCurveKind);

impl HasGeometryType for AbstractCurveKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractCurveKind::LineString(x) => x.geometry_type(),
            AbstractCurveKind::AbstractRingKind(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_curve_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractCurveKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractCurveKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind!(AbstractCurveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_curve_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_curve_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractCurveKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractCurveKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractCurveKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind!(AbstractCurveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_curve_kind!($variant, $variant);
    };
}

impl_from_for_abstract_curve_kind!(LineString);
impl_from_for_abstract_curve_kind!(AbstractRingKind);
impl_try_from_for_abstract_curve_kind!(LineString);
impl_try_from_for_abstract_curve_kind!(AbstractRingKind);

impl AbstractCurveKind {
    /// Returns the ordered positions of this curve.
    pub fn points(&self) -> &[DirectPosition] {
        match self {
            AbstractCurveKind::LineString(x) => x.points(),
            AbstractCurveKind::AbstractRingKind(x) => x.points(),
        }
    }

    pub fn length_3d(&self) -> f64 {
        match self {
            AbstractCurveKind::LineString(x) => x.length_3d(),
            AbstractCurveKind::AbstractRingKind(x) => x.length_3d(),
        }
    }

    /// Returns the minimum 2D distance between `point` and this curve,
    /// measured in the XY plane (z coordinates are ignored).
    ///
    /// See [`LineString::distance_2d`] and [`AbstractRingKind::distance_2d`].
    pub fn distance_2d(&self, point: &DirectPosition) -> f64 {
        match self {
            AbstractCurveKind::LineString(x) => x.distance_2d(point),
            AbstractCurveKind::AbstractRingKind(x) => x.distance_2d(point),
        }
    }
}

impl IterGeometries for AbstractCurveKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractCurveKind::LineString(x) => x.iter_geometries(),
            AbstractCurveKind::AbstractRingKind(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractCurveKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractCurveKind::LineString(x) => x.apply_transform(transform),
            AbstractCurveKind::AbstractRingKind(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractCurveKind::LineString(x) => x.apply_isometry(isometry),
            AbstractCurveKind::AbstractRingKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractCurveKind::LineString(x) => x.apply_translation(vector),
            AbstractCurveKind::AbstractRingKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractCurveKind::LineString(x) => x.apply_rotation(rotation),
            AbstractCurveKind::AbstractRingKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractCurveKind::LineString(x) => x.apply_scale(scale),
            AbstractCurveKind::AbstractRingKind(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractCurveKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractCurveKind::LineString(x) => x.compute_envelope(),
            AbstractCurveKind::AbstractRingKind(x) => x.compute_envelope(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::primitives::LinearRing;

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    #[test]
    fn distance_2d_line_string() {
        let kind = AbstractCurveKind::LineString(
            LineString::new([pos(0.0, 0.0, 0.0), pos(10.0, 0.0, 0.0)]).unwrap(),
        );
        assert!((kind.distance_2d(&pos(5.0, 3.0, 9.0)) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_ring() {
        let ring = LinearRing::new([
            pos(0.0, 0.0, 0.0),
            pos(10.0, 0.0, 0.0),
            pos(10.0, 10.0, 0.0),
            pos(0.0, 10.0, 0.0),
        ])
        .unwrap();
        let kind = AbstractCurveKind::AbstractRingKind(AbstractRingKind::LinearRing(ring));
        // Closing edge (0,10)→(0,0).
        assert!((kind.distance_2d(&pos(-2.0, 5.0, 0.0)) - 2.0).abs() < 1e-10);
    }
}
