use crate::Error;
use crate::model::common::{ApplyTransform, ComputeEnvelope, Triangulate, Triangulation};
use crate::model::geometry::primitives::{
    AbstractRingProperty, AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut,
    LinearRing, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use parry3d_f64::query::PointQuery;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub abstract_surface_patch: AbstractSurfacePatch,
    exterior: AbstractRingProperty,
}

impl Triangle {
    pub fn new(exterior: AbstractRingProperty) -> Result<Self, Error> {
        Self::validate(&exterior)?;

        Ok(Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
        })
    }

    pub(crate) fn new_unchecked(exterior: AbstractRingProperty) -> Self {
        Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
        }
    }

    pub fn from_abstract_surface_patch(
        abstract_surface_patch: AbstractSurfacePatch,
        exterior: AbstractRingProperty,
    ) -> Result<Self, Error> {
        Self::validate(&exterior)?;

        Ok(Self {
            abstract_surface_patch,
            exterior,
        })
    }

    pub fn from_points(
        a: DirectPosition,
        b: DirectPosition,
        c: DirectPosition,
    ) -> Result<Self, Error> {
        let linear_ring = LinearRing::new([a, b, c])?;
        let exterior = AbstractRingProperty::from_object(linear_ring.into());

        Ok(Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
        })
    }

    pub fn from_points_unchecked(a: DirectPosition, b: DirectPosition, c: DirectPosition) -> Self {
        let linear_ring =
            LinearRing::new([a, b, c]).expect("from points unchecked: LinearRing::new");
        let exterior = AbstractRingProperty::from_object(linear_ring.into());

        Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            exterior,
        }
    }

    fn validate(exterior: &AbstractRingProperty) -> Result<(), Error> {
        if let Some(object) = exterior.object() {
            let len = object.points().len();
            if len != 3 {
                return Err(Error::InvalidElementCount {
                    geometry: "Triangle",
                    expected: 3,
                    actual: len,
                    spec: Some("OGC 07-036 §10.5.12"),
                });
            }
        }

        Ok(())
    }

    pub fn exterior(&self) -> &AbstractRingProperty {
        &self.exterior
    }

    pub fn a(&self) -> &DirectPosition {
        &self.exterior.object().unwrap().points()[0]
    }

    pub fn b(&self) -> &DirectPosition {
        &self.exterior.object().unwrap().points()[1]
    }

    pub fn c(&self) -> &DirectPosition {
        &self.exterior.object().unwrap().points()[2]
    }
}

impl AsAbstractSurfacePatch for Triangle {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        &self.abstract_surface_patch
    }
}

impl AsAbstractSurfacePatchMut for Triangle {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        &mut self.abstract_surface_patch
    }
}

impl Triangle {
    pub fn distance_to_local_point(&self, p: &DirectPosition) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        let point: parry3d_f64::math::Vector = (*p).into();
        parry_triangle.distance_to_local_point(point, false)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        vec![&self.a(), &self.b(), &self.c()]
    }

    pub fn area(&self) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        // parry_triangle.distance_to_local_point().
        parry_triangle.area()
    }
}

impl ApplyTransform for Triangle {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(object) = self.exterior.object_mut() {
            object.apply_transform(transform);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(object) = self.exterior.object_mut() {
            object.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(object) = self.exterior.object_mut() {
            object.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(object) = self.exterior.object_mut() {
            object.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(object) = self.exterior.object_mut() {
            object.apply_scale(scale);
        }
    }
}

impl ComputeEnvelope for Triangle {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.exterior.object()?.compute_envelope()
    }
}

impl Triangulate for Triangle {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let surface = TriangulatedSurface::from_triangles(vec![self.clone()])?;
        Ok(Triangulation::new(surface, Vec::new()))
    }
}

impl From<Triangle> for parry3d_f64::shape::Triangle {
    fn from(item: Triangle) -> Self {
        Self::new((*item.a()).into(), (*item.b()).into(), (*item.c()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_construction_test() {
        let linear_ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        ])
        .expect("should work");
        let triangle_result = Triangle::new(AbstractRingProperty::from_object(linear_ring.into()));

        assert!(matches!(
            triangle_result,
            Err(Error::InvalidElementCount { .. })
        ));
    }

    #[test]
    fn triangle_distance_test() {
        let linear_ring = LinearRing::new(vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
        ])
        .expect("LinearRing::new");
        let triangle =
            Triangle::new(AbstractRingProperty::from_object(linear_ring.into())).unwrap();

        let distance =
            triangle.distance_to_local_point(&DirectPosition::new(0.5, 0.5, 1.0).unwrap());

        assert_eq!(distance, 1.0);
    }
}
