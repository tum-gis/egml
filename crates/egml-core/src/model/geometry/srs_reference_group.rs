use crate::model::xs::AnyUri;

/// The `gml:SRSReferenceGroup` attributes ([OGC 07-036 §10.1.3.1](https://docs.ogc.org/is/07-036/07-036.pdf)):
/// the coordinate reference system of a geometry and the dimension of its positions.
///
/// Carried by geometries (see [`AbstractGeometry`](crate::model::geometry::AbstractGeometry))
/// and by [`Envelope`](crate::model::geometry::Envelope).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SrsReferenceGroup {
    srs_name: Option<AnyUri>,
    srs_dimension: Option<u32>,
}

impl SrsReferenceGroup {
    /// Creates an empty group with neither attribute set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if neither `srsName` nor `srsDimension` is set.
    pub fn is_empty(&self) -> bool {
        self.srs_name.is_none() && self.srs_dimension.is_none()
    }

    /// Returns the SRS name identifying the CRS, or `None` if unspecified.
    pub fn srs_name(&self) -> Option<&AnyUri> {
        self.srs_name.as_ref()
    }

    /// Returns the coordinate dimension of the positions, or `None` if unspecified.
    pub fn srs_dimension(&self) -> Option<u32> {
        self.srs_dimension
    }

    /// Sets the SRS name identifying the CRS (e.g. `"urn:ogc:def:crs:EPSG::25832"`).
    pub fn set_srs_name(&mut self, srs_name: impl Into<AnyUri>) {
        self.srs_name = Some(srs_name.into());
    }

    /// Sets or clears the SRS name.
    pub fn set_srs_name_opt(&mut self, srs_name: Option<AnyUri>) {
        self.srs_name = srs_name;
    }

    /// Clears the SRS name.
    pub fn clear_srs_name(&mut self) {
        self.srs_name = None;
    }

    /// Sets the coordinate dimension of the positions (typically `2` or `3`).
    pub fn set_srs_dimension(&mut self, srs_dimension: u32) {
        self.srs_dimension = Some(srs_dimension);
    }

    /// Sets or clears the coordinate dimension.
    pub fn set_srs_dimension_opt(&mut self, srs_dimension: Option<u32>) {
        self.srs_dimension = srs_dimension;
    }

    /// Clears the coordinate dimension.
    pub fn clear_srs_dimension(&mut self) {
        self.srs_dimension = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_group_is_empty() {
        assert!(SrsReferenceGroup::new().is_empty());
    }

    #[test]
    fn group_with_any_attribute_is_not_empty() {
        let mut group = SrsReferenceGroup::new();
        group.set_srs_dimension(3);
        assert!(!group.is_empty());

        group.clear_srs_dimension();
        group.set_srs_name("EPSG:25832");
        assert!(!group.is_empty());
        assert_eq!(group.srs_name().map(AnyUri::as_str), Some("EPSG:25832"));
    }

    #[test]
    fn srs_name_accepts_any_uri() {
        let uri = AnyUri::new("urn:ogc:def:crs:EPSG::25832");
        let mut group = SrsReferenceGroup::new();

        group.set_srs_name(uri.clone());
        assert_eq!(group.srs_name().map(AnyUri::as_str), Some(uri.as_str()));

        group.set_srs_name_opt(None);
        assert!(group.is_empty());

        group.set_srs_name_opt(Some(uri));
        assert_eq!(
            group.srs_name().map(AnyUri::as_str),
            Some("urn:ogc:def:crs:EPSG::25832")
        );
    }
}
