use crate::Error;
use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::common::{ApplyTransform, ComputeEnvelope};
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::abstract_surface_patch_kind::AbstractSurfacePatchKind;
use crate::model::xlink::HRef;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// An ordered collection of [`AbstractSurfacePatchKind`] elements.
///
/// Corresponds to `gml:SurfacePatchArrayPropertyType` in ISO 19136.
/// Used inside [`Surface`](crate::model::geometry::primitives::Surface) to
/// hold its constituent patches.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbstractSurfacePatchArrayProperty {
    objects: Vec<AbstractSurfacePatchKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl AbstractSurfacePatchArrayProperty {
    pub fn new(
        objects: Vec<AbstractSurfacePatchKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            objects,
            association,
            ownership,
        }
    }

    pub fn from_objects(objects: Vec<AbstractSurfacePatchKind>) -> Self {
        Self {
            objects,
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            objects: Vec::new(),
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn objects(&self) -> &[AbstractSurfacePatchKind] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<AbstractSurfacePatchKind> {
        &mut self.objects
    }

    pub fn set_objects(&mut self, objects: Vec<AbstractSurfacePatchKind>) {
        self.objects = objects;
    }

    pub fn push_object(&mut self, object: AbstractSurfacePatchKind) {
        self.objects.push(object);
    }

    pub fn extend_objects(&mut self, objects: impl IntoIterator<Item = AbstractSurfacePatchKind>) {
        self.objects.extend(objects);
    }
}

impl HasAssociationAttributes for AbstractSurfacePatchArrayProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractSurfacePatchArrayProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractSurfacePatchArrayProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractSurfacePatchArrayProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

impl AbstractSurfacePatchArrayProperty {
    /// Returns the number of patches.
    pub fn objects_len(&self) -> usize {
        self.objects.len()
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.objects
            .iter()
            .map(|p| p.area_3d())
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }
}

impl ApplyTransform for AbstractSurfacePatchArrayProperty {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.objects
            .iter_mut()
            .for_each(|x| x.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.objects
            .iter_mut()
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.objects
            .iter_mut()
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.objects
            .iter_mut()
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.objects.iter_mut().for_each(|x| x.apply_scale(scale));
    }
}

impl ComputeEnvelope for AbstractSurfacePatchArrayProperty {
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .objects
            .iter()
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}
