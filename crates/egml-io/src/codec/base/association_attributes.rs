use egml_core::Error;
use egml_core::model::base::AssociationAttributes;
use egml_core::model::xlink::{ActuateType, HRef, ShowType};
use serde::{Deserialize, Serialize};

/// Renders the `gml:AssociationAttributeGroup` (`xlink:href`, `xlink:title`, ...) as
/// `(name, value)` pairs suitable for [`XmlNodeParts::attributes`](crate::util::XmlNodeParts).
///
/// Returns a `Vec` rather than a `HashMap` so attribute order is stable and deterministic —
/// XML serialization output should not vary between runs.
pub fn serialize_association_attributes(
    association_attributes: &AssociationAttributes,
) -> Vec<(String, String)> {
    let mut attributes = Vec::new();

    if let Some(href) = &association_attributes.href {
        attributes.push(("xlink:href".to_string(), href.to_string()));
    }
    if let Some(title) = &association_attributes.title {
        attributes.push(("xlink:title".to_string(), title.clone()));
    }
    if let Some(role) = &association_attributes.role {
        attributes.push(("xlink:role".to_string(), role.clone()));
    }
    if let Some(arcrole) = &association_attributes.arcrole {
        attributes.push(("xlink:arcrole".to_string(), arcrole.clone()));
    }
    if let Some(show) = &association_attributes.show {
        attributes.push(("xlink:show".to_string(), show.to_string()));
    }
    if let Some(actuate) = &association_attributes.actuate {
        attributes.push(("xlink:actuate".to_string(), actuate.to_string()));
    }

    attributes
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAssociationAttributes {
    #[serde(
        rename(deserialize = "@href", serialize = "@xlink:href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(deserialize = "@title", serialize = "@xlink:title"),
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,

    #[serde(
        rename(deserialize = "@role", serialize = "@xlink:role"),
        skip_serializing_if = "Option::is_none"
    )]
    pub role: Option<String>,

    #[serde(
        rename(deserialize = "@arcrole", serialize = "@xlink:arcrole"),
        skip_serializing_if = "Option::is_none"
    )]
    pub arcrole: Option<String>,

    #[serde(
        rename(deserialize = "@show", serialize = "@xlink:show"),
        skip_serializing_if = "Option::is_none"
    )]
    pub show: Option<String>,

    #[serde(
        rename(deserialize = "@actuate", serialize = "@xlink:actuate"),
        skip_serializing_if = "Option::is_none"
    )]
    pub actuate: Option<String>,
}

impl TryFrom<GmlAssociationAttributes> for AssociationAttributes {
    type Error = Error;

    fn try_from(item: GmlAssociationAttributes) -> Result<Self, Self::Error> {
        let show = item
            .show
            .map(|value| {
                value
                    .parse::<ShowType>()
                    .map_err(|_| Error::InvalidAttributeValue {
                        attribute: "xlink:show",
                        value,
                    })
            })
            .transpose()?;
        let actuate = item
            .actuate
            .map(|value| {
                value
                    .parse::<ActuateType>()
                    .map_err(|_| Error::InvalidAttributeValue {
                        attribute: "xlink:actuate",
                        value,
                    })
            })
            .transpose()?;

        Ok(Self {
            href: item.href.map(HRef::from),
            nil_reason: None,
            title: item.title,
            role: item.role,
            arcrole: item.arcrole,
            show,
            actuate,
        })
    }
}

impl From<&AssociationAttributes> for GmlAssociationAttributes {
    fn from(item: &AssociationAttributes) -> Self {
        Self {
            href: item.href.as_ref().map(|href| href.to_string()),
            title: item.title.clone(),
            role: item.role.clone(),
            arcrole: item.arcrole.clone(),
            show: item.show.as_ref().map(|show| show.to_string()),
            actuate: item.actuate.as_ref().map(|actuate| actuate.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlAssociationAttributes;
    use egml_core::Error;
    use egml_core::model::base::AssociationAttributes;
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};
    use quick_xml::de;

    #[test]
    fn deserializes_full_attribute_group() {
        let xml = b"<con:relatedTo xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\"/>";

        let parsed: GmlAssociationAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed.href.as_deref(), Some("#some-id"));
        assert_eq!(parsed.title.as_deref(), Some("Some Title"));
        assert_eq!(parsed.role.as_deref(), Some("http://example.com/role"));
        assert_eq!(
            parsed.arcrole.as_deref(),
            Some("http://example.com/arcrole")
        );
        assert_eq!(parsed.show.as_deref(), Some("new"));
        assert_eq!(parsed.actuate.as_deref(), Some("onLoad"));
    }

    #[test]
    fn ignores_unrelated_attributes() {
        let xml = b"<con:relatedTo gml:id=\"UUID_1\" gml:owns=\"true\" xlink:href=\"#some-id\"/>";

        let parsed: GmlAssociationAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed.href.as_deref(), Some("#some-id"));
    }

    #[test]
    fn defaults_to_empty() {
        let xml = b"<con:relatedTo/>";

        let parsed: GmlAssociationAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed, GmlAssociationAttributes::default());
    }

    #[test]
    fn try_from_converts_full_attribute_group() {
        let item = GmlAssociationAttributes {
            href: Some("#some-id".to_string()),
            title: Some("Some Title".to_string()),
            role: Some("http://example.com/role".to_string()),
            arcrole: Some("http://example.com/arcrole".to_string()),
            show: Some("new".to_string()),
            actuate: Some("onLoad".to_string()),
        };

        let association = AssociationAttributes::try_from(item).unwrap();

        assert_eq!(association.href, Some(HRef::from_local("some-id")));
        assert_eq!(association.title.as_deref(), Some("Some Title"));
        assert_eq!(association.role.as_deref(), Some("http://example.com/role"));
        assert_eq!(
            association.arcrole.as_deref(),
            Some("http://example.com/arcrole")
        );
        assert_eq!(association.show, Some(ShowType::New));
        assert_eq!(association.actuate, Some(ActuateType::OnLoad));
    }

    #[test]
    fn from_converts_full_attribute_group() {
        let association = AssociationAttributes {
            href: Some(HRef::from_local("some-id")),
            nil_reason: None,
            title: Some("Some Title".to_string()),
            role: Some("http://example.com/role".to_string()),
            arcrole: Some("http://example.com/arcrole".to_string()),
            show: Some(ShowType::New),
            actuate: Some(ActuateType::OnLoad),
        };

        let item = GmlAssociationAttributes::from(&association);

        assert_eq!(item.href.as_deref(), Some("#some-id"));
        assert_eq!(item.title.as_deref(), Some("Some Title"));
        assert_eq!(item.role.as_deref(), Some("http://example.com/role"));
        assert_eq!(item.arcrole.as_deref(), Some("http://example.com/arcrole"));
        assert_eq!(item.show.as_deref(), Some("new"));
        assert_eq!(item.actuate.as_deref(), Some("onLoad"));
    }

    #[test]
    fn round_trip_through_gml_and_back() {
        let association = AssociationAttributes {
            href: Some(HRef::from_local("some-id")),
            nil_reason: None,
            title: Some("Some Title".to_string()),
            role: Some("http://example.com/role".to_string()),
            arcrole: Some("http://example.com/arcrole".to_string()),
            show: Some(ShowType::New),
            actuate: Some(ActuateType::OnLoad),
        };

        let item = GmlAssociationAttributes::from(&association);
        let recovered = AssociationAttributes::try_from(item).unwrap();

        assert_eq!(recovered, association);
    }

    #[test]
    fn try_from_rejects_invalid_show_value() {
        let item = GmlAssociationAttributes {
            show: Some("bogus".to_string()),
            ..Default::default()
        };

        let result = AssociationAttributes::try_from(item);

        assert!(matches!(
            result,
            Err(Error::InvalidAttributeValue {
                attribute: "xlink:show",
                ..
            })
        ));
    }
}
