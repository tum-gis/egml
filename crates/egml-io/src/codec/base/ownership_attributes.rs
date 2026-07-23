use egml_core::model::base::OwnershipAttributes;
use serde::{Deserialize, Deserializer, Serialize};

/// Renders the `gml:OwnershipAttributeGroup` (`gml:owns`) as `(name, value)` pairs suitable
/// for [`XmlNodeParts::attributes`](crate::util::XmlNodeParts).
///
/// `owns` defaults to `false` in the schema, so an absent attribute and `gml:owns="false"`
/// are equivalent — the attribute is only emitted when `true`, matching
/// [`GmlOwnershipAttributes::from`]'s existing omit-when-false behavior.
pub fn serialize_ownership_attributes(
    ownership_attributes: &OwnershipAttributes,
) -> Vec<(String, String)> {
    if ownership_attributes.owns {
        vec![("gml:owns".to_string(), "true".to_string())]
    } else {
        Vec::new()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub struct GmlOwnershipAttributes {
    #[serde(
        rename(deserialize = "@owns", serialize = "@gml:owns"),
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_xsd_boolean",
        default
    )]
    pub owns: Option<bool>,
}

// `#[serde(flatten)]` (used when this struct is combined with `GmlAssociationAttributes`,
// e.g. in `GmlReference`/`GmlAbstractGeometryProperty`) forces serde to buffer the whole
// input into a generic `Content` value before re-deserializing each flattened field. quick-xml
// populates that buffer with the raw attribute string, so `bool`'s own `Deserialize` impl then
// fails with "invalid type: string, expected a boolean" — the buffer never coerces strings to
// bool the way quick-xml's own deserializer does. Deserializing as a string and parsing it
// ourselves works in both the flattened and standalone case.
fn deserialize_optional_xsd_boolean<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|value| match value.as_str() {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(serde::de::Error::custom(format!(
                "invalid xsd:boolean value: {value}"
            ))),
        })
        .transpose()
}

impl From<GmlOwnershipAttributes> for OwnershipAttributes {
    fn from(item: GmlOwnershipAttributes) -> Self {
        Self {
            owns: item.owns.unwrap_or(false),
        }
    }
}

impl From<&OwnershipAttributes> for GmlOwnershipAttributes {
    fn from(item: &OwnershipAttributes) -> Self {
        Self {
            // `owns` defaults to `false` in the schema, so only emit it when `true` —
            // an absent attribute and `gml:owns="false"` are equivalent, and omitting
            // it keeps the common case free of redundant attributes.
            owns: item.owns.then_some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlOwnershipAttributes;
    use egml_core::model::base::OwnershipAttributes;
    use quick_xml::de;

    #[test]
    fn deserializes_owns_true() {
        let xml = b"<con:relatedTo gml:owns=\"true\"/>";

        let parsed: GmlOwnershipAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed.owns, Some(true));
    }

    #[test]
    fn ignores_unrelated_attributes() {
        let xml = b"<con:relatedTo xlink:href=\"#some-id\" gml:owns=\"false\"/>";

        let parsed: GmlOwnershipAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed.owns, Some(false));
    }

    #[test]
    fn defaults_to_empty() {
        let xml = b"<con:relatedTo/>";

        let parsed: GmlOwnershipAttributes = de::from_reader(xml.as_ref()).unwrap();

        assert_eq!(parsed, GmlOwnershipAttributes::default());
    }

    #[test]
    fn from_gml_defaults_missing_owns_to_false() {
        let item = GmlOwnershipAttributes { owns: None };

        let ownership = OwnershipAttributes::from(item);

        assert!(!ownership.owns);
    }

    #[test]
    fn from_gml_converts_owns_true() {
        let item = GmlOwnershipAttributes { owns: Some(true) };

        let ownership = OwnershipAttributes::from(item);

        assert!(ownership.owns);
    }

    #[test]
    fn from_ownership_attributes_round_trips() {
        let ownership = OwnershipAttributes { owns: true };

        let item = GmlOwnershipAttributes::from(&ownership);
        let recovered = OwnershipAttributes::from(item);

        assert_eq!(recovered, ownership);
    }

    #[test]
    fn from_ownership_attributes_omits_false() {
        let ownership = OwnershipAttributes { owns: false };

        let item = GmlOwnershipAttributes::from(&ownership);

        assert_eq!(item.owns, None);
    }

    #[test]
    fn serializes_owns_true_with_gml_prefix() {
        let item = GmlOwnershipAttributes { owns: Some(true) };

        let xml = quick_xml::se::to_string_with_root("con:relatedTo", &item).unwrap();

        assert_eq!(xml, r##"<con:relatedTo gml:owns="true"/>"##);
    }

    #[test]
    fn serializes_owns_false_as_empty_element() {
        let item = GmlOwnershipAttributes::from(&OwnershipAttributes { owns: false });

        let xml = quick_xml::se::to_string_with_root("con:relatedTo", &item).unwrap();

        assert_eq!(xml, "<con:relatedTo/>");
    }

    #[test]
    fn rejects_invalid_owns_value() {
        let xml = b"<con:relatedTo gml:owns=\"maybe\"/>";

        let parsed: Result<GmlOwnershipAttributes, _> = de::from_reader(xml.as_ref());

        assert!(parsed.is_err());
    }
}
