use crate::Error;
use egml_core::model::base::{AssociationAttributes, OwnershipAttributes};
use egml_core::model::xlink::{ActuateType, HRef, ShowType};
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};

/// Reads the `gml:AssociationAttributeGroup` (`xlink:*`) and `gml:OwnershipAttributeGroup`
/// (`gml:owns`) attributes off the first start or empty element in `xml_document`.
///
/// Reads attributes directly via [`quick_xml::Reader`] instead of deserializing the
/// surrounding element through serde, so it can be reused on fragments whose element
/// name and shape aren't known up front.
///
/// # Errors
///
/// Returns an error if `xml_document` is not well-formed XML, contains no element at
/// all, or has an `xlink:show`/`xlink:actuate` value that is not one of the values
/// defined by the XLink schema.
pub fn deserialize_association_and_ownership_attributes(
    xml_document: &[u8],
) -> Result<(AssociationAttributes, OwnershipAttributes), Error> {
    let mut reader = Reader::from_reader(xml_document);
    loop {
        match reader.read_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let mut association = AssociationAttributes::default();
                let mut ownership = OwnershipAttributes::default();
                for attr in e.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
                    match attr.key.local_name().as_ref() {
                        "href" => association.href = Some(HRef::from(value)),
                        "title" => association.title = Some(value),
                        "role" => association.role = Some(value),
                        "arcrole" => association.arcrole = Some(value),
                        "show" => {
                            let show = value.parse::<ShowType>().map_err(|_| {
                                egml_core::Error::InvalidAttributeValue {
                                    attribute: "xlink:show",
                                    value,
                                }
                            })?;
                            association.show = Some(show);
                        }
                        "actuate" => {
                            let actuate = value.parse::<ActuateType>().map_err(|_| {
                                egml_core::Error::InvalidAttributeValue {
                                    attribute: "xlink:actuate",
                                    value,
                                }
                            })?;
                            association.actuate = Some(actuate);
                        }
                        "owns" => {
                            ownership.owns = match value.as_str() {
                                "true" | "1" => true,
                                "false" | "0" => false,
                                _ => {
                                    return Err(egml_core::Error::InvalidAttributeValue {
                                        attribute: "gml:owns",
                                        value,
                                    }
                                    .into());
                                }
                            };
                        }
                        _ => {}
                    }
                }
                return Ok((association, ownership));
            }
            Event::Eof => return Err(Error::XmlDe(quick_xml::DeError::UnexpectedEof)),
            _ => {}
        }
    }
}

/// Renders the `gml:AssociationAttributeGroup` (`xlink:*`) and `gml:OwnershipAttributeGroup`
/// (`gml:owns`) attributes as `(name, value)` pairs.
///
/// Returns a `Vec` rather than a `HashMap` so attribute order is stable and deterministic —
/// XML serialization output should not vary between runs. `owns` defaults to `false` in the
/// schema, so an absent attribute and `gml:owns="false"` are equivalent — the attribute is
/// only emitted when `true`.
pub fn serialize_association_and_ownership_attributes(
    association: &AssociationAttributes,
    ownership: &OwnershipAttributes,
) -> Vec<(String, String)> {
    let mut attributes = Vec::new();

    if let Some(href) = &association.href {
        attributes.push(("xlink:href".to_string(), href.to_string()));
    }
    if let Some(title) = &association.title {
        attributes.push(("xlink:title".to_string(), title.clone()));
    }
    if let Some(role) = &association.role {
        attributes.push(("xlink:role".to_string(), role.clone()));
    }
    if let Some(arcrole) = &association.arcrole {
        attributes.push(("xlink:arcrole".to_string(), arcrole.clone()));
    }
    if let Some(show) = &association.show {
        attributes.push(("xlink:show".to_string(), show.to_string()));
    }
    if let Some(actuate) = &association.actuate {
        attributes.push(("xlink:actuate".to_string(), actuate.to_string()));
    }
    if ownership.owns {
        attributes.push(("gml:owns".to_string(), "true".to_string()));
    }

    attributes
}

/// Reads only the `gml:OwnershipAttributeGroup` (`gml:owns`) off the first element in
/// `xml_document`, for property types that don't declare `gml:AssociationAttributeGroup`.
///
/// # Errors
///
/// Same as [`deserialize_association_and_ownership_attributes`].
pub fn deserialize_ownership_attributes(xml_document: &[u8]) -> Result<OwnershipAttributes, Error> {
    deserialize_association_and_ownership_attributes(xml_document).map(|(_, ownership)| ownership)
}

/// Builds the attribute list for the `gml:OwnershipAttributeGroup` alone.
pub fn serialize_ownership_attributes(ownership: &OwnershipAttributes) -> Vec<(String, String)> {
    serialize_association_and_ownership_attributes(&AssociationAttributes::default(), ownership)
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_association_and_ownership_attributes, deserialize_ownership_attributes,
        serialize_association_and_ownership_attributes, serialize_ownership_attributes,
    };
    use egml_core::model::base::{AssociationAttributes, OwnershipAttributes};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn deserializes_local_href_and_owns() {
        let xml = br##"<con:relatedTo xlink:href="#some-id" gml:owns="true"/>"##;

        let (association, ownership) =
            deserialize_association_and_ownership_attributes(xml).unwrap();

        assert_eq!(association.href, Some(HRef::from_local("some-id")));
        assert!(ownership.owns);
    }

    #[test]
    fn deserializes_remote_href_and_defaults_owns_to_false() {
        let xml = br#"<con:relatedTo xlink:href="https://example.com/feature/1"/>"#;

        let (association, ownership) =
            deserialize_association_and_ownership_attributes(xml).unwrap();

        assert_eq!(
            association.href,
            Some(HRef::from_remote("https://example.com/feature/1"))
        );
        assert!(!ownership.owns);
    }

    #[test]
    fn deserializes_title_role_arcrole_show_and_actuate() {
        let xml = br#"<con:relatedTo
            xlink:title="Some Title"
            xlink:role="urn:role"
            xlink:arcrole="urn:arcrole"
            xlink:show="new"
            xlink:actuate="onLoad"/>"#;

        let (association, _) = deserialize_association_and_ownership_attributes(xml).unwrap();

        assert_eq!(association.title, Some("Some Title".to_string()));
        assert_eq!(association.role, Some("urn:role".to_string()));
        assert_eq!(association.arcrole, Some("urn:arcrole".to_string()));
        assert_eq!(association.show, Some(ShowType::New));
        assert_eq!(association.actuate, Some(ActuateType::OnLoad));
    }

    #[test]
    fn owns_accepts_numeric_boolean() {
        let xml = br#"<con:relatedTo gml:owns="1"/>"#;

        let (_, ownership) = deserialize_association_and_ownership_attributes(xml).unwrap();

        assert!(ownership.owns);
    }

    #[test]
    fn ignores_unrelated_attributes() {
        let xml = br#"<con:relatedTo foo:bar="baz"/>"#;

        let (association, ownership) =
            deserialize_association_and_ownership_attributes(xml).unwrap();

        assert_eq!(association, Default::default());
        assert!(!ownership.owns);
    }

    #[test]
    fn defaults_to_empty_when_no_attributes() {
        let xml = b"<con:relatedTo/>";

        let (association, ownership) =
            deserialize_association_and_ownership_attributes(xml).unwrap();

        assert_eq!(association, Default::default());
        assert!(!ownership.owns);
    }

    #[test]
    fn owns_accepts_false_and_zero() {
        let xml = br#"<con:relatedTo gml:owns="false"/>"#;
        let (_, ownership) = deserialize_association_and_ownership_attributes(xml).unwrap();
        assert!(!ownership.owns);

        let xml = br#"<con:relatedTo gml:owns="0"/>"#;
        let (_, ownership) = deserialize_association_and_ownership_attributes(xml).unwrap();
        assert!(!ownership.owns);
    }

    #[test]
    fn rejects_invalid_owns_value() {
        let xml = br#"<con:relatedTo gml:owns="maybe"/>"#;

        let result = deserialize_association_and_ownership_attributes(xml);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_show_value() {
        let xml = br#"<con:relatedTo xlink:show="maybe"/>"#;

        let result = deserialize_association_and_ownership_attributes(xml);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_actuate_value() {
        let xml = br#"<con:relatedTo xlink:actuate="maybe"/>"#;

        let result = deserialize_association_and_ownership_attributes(xml);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_document() {
        let xml = b"";

        let result = deserialize_association_and_ownership_attributes(xml);

        assert!(result.is_err());
    }

    #[test]
    fn serializes_local_href_and_owns() {
        let association = AssociationAttributes {
            href: Some(HRef::from_local("some-id")),
            ..Default::default()
        };
        let ownership = OwnershipAttributes { owns: true };

        let attributes = serialize_association_and_ownership_attributes(&association, &ownership);

        assert_eq!(
            attributes,
            vec![
                ("xlink:href".to_string(), "#some-id".to_string()),
                ("gml:owns".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn serializes_omits_owns_when_false() {
        let association = AssociationAttributes {
            title: Some("Some Title".to_string()),
            ..Default::default()
        };
        let ownership = OwnershipAttributes { owns: false };

        let attributes = serialize_association_and_ownership_attributes(&association, &ownership);

        assert_eq!(
            attributes,
            vec![("xlink:title".to_string(), "Some Title".to_string())]
        );
    }

    #[test]
    fn serializes_full_attribute_group() {
        let association = AssociationAttributes {
            href: Some(HRef::from_local("some-id")),
            title: Some("Some Title".to_string()),
            role: Some("http://example.com/role".to_string()),
            arcrole: Some("http://example.com/arcrole".to_string()),
            show: Some(ShowType::New),
            actuate: Some(ActuateType::OnLoad),
            nil_reason: None,
        };
        let ownership = OwnershipAttributes { owns: true };

        let attributes = serialize_association_and_ownership_attributes(&association, &ownership);

        assert_eq!(
            attributes,
            vec![
                ("xlink:href".to_string(), "#some-id".to_string()),
                ("xlink:title".to_string(), "Some Title".to_string()),
                (
                    "xlink:role".to_string(),
                    "http://example.com/role".to_string()
                ),
                (
                    "xlink:arcrole".to_string(),
                    "http://example.com/arcrole".to_string()
                ),
                ("xlink:show".to_string(), "new".to_string()),
                ("xlink:actuate".to_string(), "onLoad".to_string()),
                ("gml:owns".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn serializes_empty_when_both_default() {
        let attributes = serialize_association_and_ownership_attributes(
            &AssociationAttributes::default(),
            &OwnershipAttributes::default(),
        );

        assert!(attributes.is_empty());
    }

    #[test]
    fn round_trips_through_deserialize() {
        let association = AssociationAttributes {
            href: Some(HRef::from_remote("https://example.com/feature/1")),
            title: Some("Some Title".to_string()),
            role: Some("urn:role".to_string()),
            arcrole: Some("urn:arcrole".to_string()),
            show: Some(ShowType::New),
            actuate: Some(ActuateType::OnLoad),
            nil_reason: None,
        };
        let ownership = OwnershipAttributes { owns: true };

        let rendered = serialize_association_and_ownership_attributes(&association, &ownership);
        let attribute_string = rendered
            .iter()
            .map(|(name, value)| format!(r#"{name}="{value}""#))
            .collect::<Vec<_>>()
            .join(" ");
        let xml = format!("<con:relatedTo {attribute_string}/>");

        let (parsed_association, parsed_ownership) =
            deserialize_association_and_ownership_attributes(xml.as_bytes()).unwrap();

        assert_eq!(parsed_association, association);
        assert_eq!(parsed_ownership, ownership);
    }

    #[test]
    fn deserialize_ownership_attributes_ignores_xlink() {
        let ownership = deserialize_ownership_attributes(
            br##"<gml:pointMembers xlink:href="#id" gml:owns="true"/>"##,
        )
        .unwrap();
        assert!(ownership.owns);
    }

    #[test]
    fn serialize_ownership_attributes_writes_only_owns() {
        let attributes = serialize_ownership_attributes(&OwnershipAttributes { owns: true });
        assert_eq!(
            attributes,
            vec![("gml:owns".to_string(), "true".to_string())]
        );
    }
}
