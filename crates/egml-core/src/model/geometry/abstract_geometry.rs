use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};
use crate::model::geometry::SrsReferenceGroup;

/// Base data shared by all GML geometry types ([OGC 07-036 §10.1.3.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// Embeds [`AbstractGml`] and is in turn embedded by every concrete and
/// abstract geometry.
///
/// The [`SrsReferenceGroup`] (`srsName`, `srsDimension`) is stored out of line
/// in a lazily allocated box: it is almost never set on nested geometries
/// (rings, patches, members of aggregates), so keeping it inline would cost
/// every geometry object 24 bytes for data it does not have.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometry {
    pub abstract_gml: AbstractGml,
    /// `None` whenever the group is empty, so equality and hashing do not
    /// depend on whether an empty group was ever allocated.
    srs_reference_group: Option<Box<SrsReferenceGroup>>,
}

impl AbstractGeometry {
    /// Creates a new `AbstractGeometry` wrapping the provided GML base data.
    pub fn new() -> Self {
        Self {
            abstract_gml: AbstractGml::default(),
            srs_reference_group: None,
        }
    }

    pub fn from_abstract_gml(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            srs_reference_group: None,
        }
    }

    /// Applies `update` to the SRS reference group, allocating it on demand
    /// and dropping the allocation again once the group is empty.
    fn update_srs_reference_group(&mut self, update: impl FnOnce(&mut SrsReferenceGroup)) {
        let group = self.srs_reference_group.get_or_insert_with(Box::default);
        update(group);
        if group.is_empty() {
            self.srs_reference_group = None;
        }
    }
}

/// Object-safe read accessor for [`AbstractGeometry`] fields.
pub trait AsAbstractGeometry: AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry(&self) -> &AbstractGeometry;

    /// Returns the SRS reference group, or `None` if neither attribute is set.
    fn srs_reference_group(&self) -> Option<&SrsReferenceGroup> {
        self.abstract_geometry().srs_reference_group.as_deref()
    }

    fn srs_name(&self) -> Option<&str> {
        self.srs_reference_group()
            .and_then(SrsReferenceGroup::srs_name)
    }

    fn srs_dimension(&self) -> Option<u32> {
        self.srs_reference_group()
            .and_then(SrsReferenceGroup::srs_dimension)
    }
}

/// Mutable companion to [`AsAbstractGeometry`].
pub trait AsAbstractGeometryMut: AsAbstractGeometry + AsAbstractGmlMut {
    /// Returns a mutable reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry;

    /// Replaces the SRS reference group. An empty group is stored as `None`.
    fn set_srs_reference_group(&mut self, srs_reference_group: SrsReferenceGroup) {
        self.abstract_geometry_mut().srs_reference_group =
            (!srs_reference_group.is_empty()).then(|| Box::new(srs_reference_group));
    }

    /// Removes both SRS attributes.
    fn clear_srs_reference_group(&mut self) {
        self.abstract_geometry_mut().srs_reference_group = None;
    }

    fn set_srs_name(&mut self, srs_name: impl Into<String>) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(|group| group.set_srs_name(srs_name));
    }

    fn set_srs_name_opt(&mut self, srs_name: Option<String>) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(|group| group.set_srs_name_opt(srs_name));
    }

    fn clear_srs_name(&mut self) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(SrsReferenceGroup::clear_srs_name);
    }

    fn set_srs_dimension(&mut self, srs_dimension: u32) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(|group| group.set_srs_dimension(srs_dimension));
    }

    fn set_srs_dimension_opt(&mut self, srs_dimension: Option<u32>) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(|group| group.set_srs_dimension_opt(srs_dimension));
    }

    fn clear_srs_dimension(&mut self) {
        self.abstract_geometry_mut()
            .update_srs_reference_group(SrsReferenceGroup::clear_srs_dimension);
    }
}

impl AsAbstractGeometry for AbstractGeometry {
    fn abstract_geometry(&self) -> &AbstractGeometry {
        self
    }
}

impl AsAbstractGeometryMut for AbstractGeometry {
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_geometry_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_traits!($type);

        impl $crate::model::base::AsAbstractGml for $type {
            fn abstract_gml(&self) -> &$crate::model::base::AbstractGml {
                &<$type as $crate::model::geometry::AsAbstractGeometry>::abstract_geometry(self)
                    .abstract_gml
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_geometry_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_mut_traits!($type);

        impl $crate::model::base::AsAbstractGmlMut for $type {
            fn abstract_gml_mut(&mut self) -> &mut $crate::model::base::AbstractGml {
                &mut <$type as $crate::model::geometry::AsAbstractGeometryMut>::abstract_geometry_mut(self)
                    .abstract_gml
            }
        }
    };
}

impl_abstract_geometry_traits!(AbstractGeometry);
impl_abstract_geometry_mut_traits!(AbstractGeometry);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srs_attributes_are_not_allocated_by_default() {
        let geometry = AbstractGeometry::new();
        assert!(geometry.srs_reference_group().is_none());
        assert_eq!(geometry.srs_name(), None);
        assert_eq!(geometry.srs_dimension(), None);
    }

    #[test]
    fn setting_srs_attributes_allocates_and_reads_back() {
        let mut geometry = AbstractGeometry::new();
        geometry.set_srs_name("EPSG:25832");
        geometry.set_srs_dimension(3);

        assert_eq!(geometry.srs_name(), Some("EPSG:25832"));
        assert_eq!(geometry.srs_dimension(), Some(3));
    }

    #[test]
    fn clearing_both_srs_attributes_drops_the_allocation() {
        let mut geometry = AbstractGeometry::new();
        geometry.set_srs_name("EPSG:25832");
        geometry.set_srs_dimension(3);

        geometry.clear_srs_name();
        assert!(geometry.srs_reference_group().is_some());
        assert_eq!(geometry.srs_dimension(), Some(3));

        geometry.set_srs_dimension_opt(None);
        assert!(geometry.srs_reference_group().is_none());
        assert_eq!(geometry, AbstractGeometry::new());
    }

    #[test]
    fn setting_an_empty_group_stores_none() {
        let mut geometry = AbstractGeometry::new();
        geometry.set_srs_name("EPSG:25832");

        geometry.set_srs_reference_group(SrsReferenceGroup::new());

        assert!(geometry.srs_reference_group().is_none());
        assert_eq!(geometry, AbstractGeometry::new());
    }

    #[test]
    fn clearing_unset_attributes_does_not_allocate() {
        let mut geometry = AbstractGeometry::new();
        geometry.clear_srs_name();
        geometry.set_srs_name_opt(None);
        assert!(geometry.srs_reference_group().is_none());
    }
}
