use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
    MultiCurve, MultiGeometry, MultiPoint, MultiSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::{
    Error, impl_abstract_geometric_aggregate_mut_traits, impl_abstract_geometric_aggregate_traits,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractGeometricAggregateKind {
    MultiCurve(MultiCurve),
    MultiGeometry(MultiGeometry),
    MultiPoint(MultiPoint),
    MultiSurface(MultiSurface),
    // MultiSolid(MultiSolid),
}

impl AsAbstractGeometricAggregate for AbstractGeometricAggregateKind {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.abstract_geometric_aggregate(),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.abstract_geometric_aggregate(),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.abstract_geometric_aggregate(),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.abstract_geometric_aggregate(),
            // GeometricAggregateKind::MultiSolid(x) => x.abstract_geometric_aggregate(),
        }
    }
}

impl AsAbstractGeometricAggregateMut for AbstractGeometricAggregateKind {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.abstract_geometric_aggregate_mut(),
            AbstractGeometricAggregateKind::MultiGeometry(x) => {
                x.abstract_geometric_aggregate_mut()
            }
            AbstractGeometricAggregateKind::MultiPoint(x) => x.abstract_geometric_aggregate_mut(),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.abstract_geometric_aggregate_mut(),
            // GeometricAggregateKind::MultiSolid(x) => x.abstract_geometric_aggregate_mut(),
        }
    }
}

impl_abstract_geometric_aggregate_traits!(AbstractGeometricAggregateKind);
impl_abstract_geometric_aggregate_mut_traits!(AbstractGeometricAggregateKind);

impl HasGeometryType for AbstractGeometricAggregateKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.geometry_type(),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.geometry_type(),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.geometry_type(),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_geometric_aggregate_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::aggregates::AbstractGeometricAggregateKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::aggregates::AbstractGeometricAggregateKind::$variant(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_abstract_geometry_kind!(AbstractGeometricAggregateKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometric_aggregate_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_geometric_aggregate_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::aggregates::AbstractGeometricAggregateKind>
            for $type
        {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::aggregates::AbstractGeometricAggregateKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::aggregates::AbstractGeometricAggregateKind::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometry_kind!(AbstractGeometricAggregateKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometric_aggregate_kind!($variant, $variant);
    };
}

impl_from_for_abstract_geometric_aggregate_kind!(MultiCurve);
impl_from_for_abstract_geometric_aggregate_kind!(MultiGeometry);
impl_from_for_abstract_geometric_aggregate_kind!(MultiPoint);
impl_from_for_abstract_geometric_aggregate_kind!(MultiSurface);
impl_try_from_for_abstract_geometric_aggregate_kind!(MultiCurve);
impl_try_from_for_abstract_geometric_aggregate_kind!(MultiGeometry);
impl_try_from_for_abstract_geometric_aggregate_kind!(MultiPoint);
impl_try_from_for_abstract_geometric_aggregate_kind!(MultiSurface);

impl Triangulate for AbstractGeometricAggregateKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            AbstractGeometricAggregateKind::MultiSurface(x) => x.triangulate(),
            AbstractGeometricAggregateKind::MultiCurve(_) => {
                Err(Error::TriangulationNotSupported {
                    geometry: "MultiCurve",
                })
            }
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.triangulate(),
            AbstractGeometricAggregateKind::MultiPoint(_) => {
                Err(Error::TriangulationNotSupported {
                    geometry: "MultiPoint",
                })
            }
        }
    }
}

impl IterGeometries for AbstractGeometricAggregateKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.iter_geometries(),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.iter_geometries(),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.iter_geometries(),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractGeometricAggregateKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.apply_transform(transform),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.apply_transform(transform),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.apply_transform(transform),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.apply_transform(transform),
            // GeometricAggregateKind::MultiSolid(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.apply_isometry(isometry),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.apply_isometry(isometry),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.apply_isometry(isometry),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.apply_isometry(isometry),
            // GeometricAggregateKind::MultiSolid(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.apply_translation(vector),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.apply_translation(vector),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.apply_translation(vector),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.apply_translation(vector),
            // GeometricAggregateKind::MultiSolid(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.apply_rotation(rotation),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.apply_rotation(rotation),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.apply_rotation(rotation),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.apply_rotation(rotation),
            // GeometricAggregateKind::MultiSolid(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.apply_scale(scale),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.apply_scale(scale),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.apply_scale(scale),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.apply_scale(scale),
            // GeometricAggregateKind::MultiSolid(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractGeometricAggregateKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractGeometricAggregateKind::MultiCurve(x) => x.compute_envelope(),
            AbstractGeometricAggregateKind::MultiGeometry(x) => x.compute_envelope(),
            AbstractGeometricAggregateKind::MultiPoint(x) => x.compute_envelope(),
            AbstractGeometricAggregateKind::MultiSurface(x) => x.compute_envelope(),
            // GeometricAggregateKind::MultiSolid(x) => x.compute_envelope(),
        }
    }
}
