use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::aggregates::AbstractGeometricAggregateKind;
use crate::model::geometry::primitives::AbstractGeometricPrimitiveKind;
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{
    AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut, Envelope,
};
use crate::{Error, impl_abstract_geometry_mut_traits, impl_abstract_geometry_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractGeometryKind {
    AbstractGeometricAggregateKind(AbstractGeometricAggregateKind),
    AbstractGeometricPrimitiveKind(AbstractGeometricPrimitiveKind),
    // AbstractGeometricComplexKind(AbstractGeometricComplexKind),
    // ImplicitGeometryKind(ImplicitGeometryKind),
}

impl AsAbstractGeometry for AbstractGeometryKind {
    fn abstract_geometry(&self) -> &AbstractGeometry {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.abstract_geometry(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.abstract_geometry(),
            // GeometryKind::GeometricComplexKind(x) => x.abstract_geometry(),
        }
    }
}

impl AsAbstractGeometryMut for AbstractGeometryKind {
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.abstract_geometry_mut(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.abstract_geometry_mut(),
            // GeometryKind::GeometricComplexKind(x) => x.abstract_geometry_mut(),
            // GeometryKind::ImplicitGeometryKind(x) => x.abstract_geometry_mut(),
        }
    }
}

impl_abstract_geometry_traits!(AbstractGeometryKind);
impl_abstract_geometry_mut_traits!(AbstractGeometryKind);

impl HasGeometryType for AbstractGeometryKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.geometry_type(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_geometry_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::AbstractGeometryKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::AbstractGeometryKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_gml_kind!(AbstractGeometryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometry_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_geometry_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::AbstractGeometryKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(x: $crate::model::geometry::AbstractGeometryKind) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::AbstractGeometryKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_gml_kind!(AbstractGeometryKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometry_kind!($variant, $variant);
    };
}

impl_from_for_abstract_geometry_kind!(AbstractGeometricAggregateKind);
impl_from_for_abstract_geometry_kind!(AbstractGeometricPrimitiveKind);
impl_try_from_for_abstract_geometry_kind!(AbstractGeometricAggregateKind);
impl_try_from_for_abstract_geometry_kind!(AbstractGeometricPrimitiveKind);

impl Triangulate for AbstractGeometryKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.triangulate(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.triangulate(),
        }
    }
}

impl IterGeometries for AbstractGeometryKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.iter_geometries(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractGeometryKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.apply_transform(transform),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.apply_transform(transform),
            // GeometryKind::GeometricComplexKind(x) => x.apply_transform(transform),
            // GeometryKind::ImplicitGeometryKind(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.apply_isometry(isometry),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.apply_isometry(isometry),
            // GeometryKind::GeometricComplexKind(x) => x.apply_isometry(isometry),
            // GeometryKind::ImplicitGeometryKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.apply_translation(vector),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.apply_translation(vector),
            // GeometryKind::GeometricComplexKind(x) => x.apply_translation(vector),
            // GeometryKind::ImplicitGeometryKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.apply_rotation(rotation),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.apply_rotation(rotation),
            // GeometryKind::GeometricComplexKind(x) => x.apply_rotation(rotation),
            // GeometryKind::ImplicitGeometryKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.apply_scale(scale),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.apply_scale(scale),
            // GeometryKind::GeometricComplexKind(x) => x.apply_scale(scale),
            // GeometryKind::ImplicitGeometryKind(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractGeometryKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractGeometryKind::AbstractGeometricAggregateKind(x) => x.compute_envelope(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => x.compute_envelope(),
            // GeometryKind::GeometricComplexKind(x) => x.compute_envelope(),
            // GeometryKind::ImplicitGeometryKind(x) => x.compute_envelope(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AbstractGeometryKind;
    use crate::model::common::{GeometryType, HasGeometryType};
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::aggregates::{AbstractGeometricAggregateKind, MultiSurface};
    use crate::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, AbstractSurfaceProperty, LinearRing, Point, Polygon,
    };

    #[test]
    fn point_reports_point_geometry_type() {
        let kind = AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::Point(Point::new(
                DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
            )),
        );

        assert_eq!(kind.geometry_type(), GeometryType::Point);
    }

    #[test]
    fn polygon_nested_three_levels_deep_reports_polygon_geometry_type() {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        let kind = AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        );

        assert_eq!(kind.geometry_type(), GeometryType::Polygon);
    }

    #[test]
    fn multi_surface_reports_multi_surface_geometry_type() {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        let member = AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        let multi_surface = MultiSurface::new([member]).unwrap();
        let kind = AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiSurface(multi_surface),
        );

        assert_eq!(kind.geometry_type(), GeometryType::MultiSurface);
    }
}
