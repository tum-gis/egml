use crate::codec::base::{GmlAssociationAttributes, GmlOwnershipAttributes};
use egml_core::Error;
use egml_core::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasOwnershipAttributes, OwnershipAttributes,
    Reference,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlReference {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

impl TryFrom<GmlReference> for Reference {
    type Error = Error;

    fn try_from(item: GmlReference) -> Result<Self, Self::Error> {
        Ok(Self {
            association: AssociationAttributes::try_from(item.association)?,
            ownership: OwnershipAttributes::from(item.ownership),
        })
    }
}

impl From<&Reference> for GmlReference {
    fn from(item: &Reference) -> Self {
        Self {
            association: GmlAssociationAttributes::from(item.association()),
            ownership: GmlOwnershipAttributes::from(item.ownership()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlReference;
    use crate::codec::base::{GmlAssociationAttributes, GmlOwnershipAttributes};
    use egml_core::Error;
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes, Reference};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn try_from_converts_full_attribute_group() {
        let item = GmlReference {
            association: GmlAssociationAttributes {
                href: Some("#some-id".to_string()),
                title: Some("Some Title".to_string()),
                role: Some("http://example.com/role".to_string()),
                arcrole: Some("http://example.com/arcrole".to_string()),
                show: Some("new".to_string()),
                actuate: Some("onLoad".to_string()),
            },
            ownership: GmlOwnershipAttributes { owns: Some(true) },
        };

        let reference = Reference::try_from(item).unwrap();

        assert_eq!(reference.href(), Some(&HRef::from_local("some-id")));
        assert_eq!(reference.title(), Some("Some Title"));
        assert_eq!(reference.role(), Some("http://example.com/role"));
        assert_eq!(reference.arcrole(), Some("http://example.com/arcrole"));
        assert_eq!(reference.show(), Some(&ShowType::New));
        assert_eq!(reference.actuate(), Some(&ActuateType::OnLoad));
        assert!(reference.owns());
    }

    #[test]
    fn try_from_rejects_invalid_show_value() {
        let item = GmlReference {
            association: GmlAssociationAttributes {
                show: Some("bogus".to_string()),
                ..Default::default()
            },
            ownership: GmlOwnershipAttributes::default(),
        };

        let result = Reference::try_from(item);

        assert!(matches!(
            result,
            Err(Error::InvalidAttributeValue {
                attribute: "xlink:show",
                ..
            })
        ));
    }

    #[test]
    fn from_converts_href_only() {
        let reference = Reference::new(HRef::from_local("some-id"));

        let item = GmlReference::from(&reference);

        assert_eq!(item.association.href.as_deref(), Some("#some-id"));
        assert_eq!(item.ownership.owns, None);
    }

    #[test]
    fn round_trip_through_gml_and_back() {
        let mut reference = Reference::new(HRef::from_local("some-id"));
        reference.association.title = Some("Some Title".to_string());
        reference.ownership.owns = true;

        let item = GmlReference::from(&reference);
        let recovered = Reference::try_from(item).unwrap();

        assert_eq!(recovered, reference);
    }

    #[test]
    fn round_trips_full_attribute_group_through_xml_string() {
        let mut reference = Reference::new(HRef::from_local("some-id"));
        reference.association.title = Some("Some Title".to_string());
        reference.association.role = Some("http://example.com/role".to_string());
        reference.association.arcrole = Some("http://example.com/arcrole".to_string());
        reference.association.show = Some(ShowType::New);
        reference.association.actuate = Some(ActuateType::OnLoad);
        reference.ownership.owns = true;

        let item = GmlReference::from(&reference);
        let xml = quick_xml::se::to_string_with_root("tran:predecessor", &item).unwrap();

        let parsed: GmlReference = quick_xml::de::from_reader(xml.as_bytes()).unwrap();
        let recovered = Reference::try_from(parsed).unwrap();

        assert_eq!(recovered, reference, "xml was: {xml}");
    }

    #[test]
    fn round_trips_owns_false_through_xml_string() {
        let reference = Reference::new(HRef::from_local("some-id"));

        let item = GmlReference::from(&reference);
        let xml = quick_xml::se::to_string_with_root("tran:predecessor", &item).unwrap();

        let parsed: GmlReference = quick_xml::de::from_reader(xml.as_bytes()).unwrap();
        let recovered = Reference::try_from(parsed).unwrap();

        assert_eq!(recovered, reference, "xml was: {xml}");
        assert!(!recovered.owns());
    }

    #[test]
    fn round_trips_predecessor_xml() {
        let xml =
            r##"<tran:predecessor xlink:href="#UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3"/>"##;

        let parsed: GmlReference = quick_xml::de::from_reader(xml.as_bytes()).unwrap();
        let reference = Reference::try_from(parsed).unwrap();

        assert_eq!(
            reference.href(),
            Some(&HRef::from_local(
                "UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3"
            ))
        );
        assert!(!reference.owns());

        let item = GmlReference::from(&reference);
        let output = quick_xml::se::to_string_with_root("tran:predecessor", &item).unwrap();

        assert_eq!(output, xml);
    }
}
