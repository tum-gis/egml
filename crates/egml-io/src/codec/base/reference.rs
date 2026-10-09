use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes, Reference};
use std::io::Write;

pub fn deserialize_reference(xml_document: &[u8]) -> Result<Reference, Error> {
    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;
    Ok(Reference {
        association,
        ownership,
    })
}

pub fn serialize_reference<N: XmlNamespace, E: XmlElement, W: Write>(
    reference: &Reference,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        reference.association(),
        reference.ownership(),
    );

    xml_fragment_writer.write_empty_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        attributes,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_reference, serialize_reference};
    use crate::Error;
    use crate::util::{Formatting, XmlElement, XmlFragmentWriter, XmlNamespace};
    use egml_core::model::base::{
        AssociationAttributes, HasAssociationAttributes, HasOwnershipAttributes,
        OwnershipAttributes, Reference,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    /// Minimal `tran:predecessor`-shaped fixture, standing in for a future
    /// `CitygmlNamespace`/Transportation-module element type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TranNamespace;

    impl XmlNamespace for TranNamespace {
        fn uri(&self) -> &'static str {
            "urn:x-tran"
        }

        fn default_prefix(&self) -> Option<&'static str> {
            Some("tran")
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct PredecessorElement;

    impl XmlElement for PredecessorElement {
        fn from_local_name(local_name: &str) -> Option<Self> {
            (local_name == "predecessor").then_some(Self)
        }

        fn local_name(&self) -> &'static str {
            "predecessor"
        }
    }

    fn serialize(reference: &Reference) -> String {
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_reference(
            reference,
            &mut xml_fragment_writer,
            TranNamespace,
            PredecessorElement,
        )
        .unwrap();
        String::from_utf8(xml_fragment_writer.into_bytes()).unwrap()
    }

    #[test]
    fn deserializes_full_attribute_group() {
        let xml = br##"<tran:predecessor
            xlink:href="#some-id"
            xlink:title="Some Title"
            xlink:role="http://example.com/role"
            xlink:arcrole="http://example.com/arcrole"
            xlink:show="new"
            xlink:actuate="onLoad"
            gml:owns="true"/>"##;

        let reference = deserialize_reference(xml).unwrap();

        assert_eq!(reference.href(), Some(&HRef::from_local("some-id")));
        assert_eq!(reference.title(), Some("Some Title"));
        assert_eq!(reference.role(), Some("http://example.com/role"));
        assert_eq!(reference.arcrole(), Some("http://example.com/arcrole"));
        assert_eq!(reference.show(), Some(&ShowType::New));
        assert_eq!(reference.actuate(), Some(&ActuateType::OnLoad));
        assert!(reference.owns());
    }

    #[test]
    fn rejects_invalid_show_value() {
        let xml = br#"<tran:predecessor xlink:show="bogus"/>"#;

        let result = deserialize_reference(xml);

        assert!(matches!(
            result,
            Err(Error::EgmlError(egml_core::Error::InvalidAttributeValue {
                attribute: "xlink:show",
                ..
            }))
        ));
    }

    #[test]
    fn serializes_href_only() {
        let reference = Reference::new(HRef::from_local("some-id"));

        let xml = serialize(&reference);

        assert_eq!(xml, r##"<tran:predecessor xlink:href="#some-id"/>"##);
    }

    #[test]
    fn round_trips_full_attribute_group() {
        let reference = Reference {
            association: AssociationAttributes {
                href: Some(HRef::from_local("some-id")),
                nil_reason: None,
                title: Some("Some Title".to_string()),
                role: Some("http://example.com/role".to_string()),
                arcrole: Some("http://example.com/arcrole".to_string()),
                show: Some(ShowType::New),
                actuate: Some(ActuateType::OnLoad),
            },
            ownership: OwnershipAttributes { owns: true },
        };

        let xml = serialize(&reference);
        let recovered = deserialize_reference(xml.as_bytes()).unwrap();

        assert_eq!(recovered, reference, "xml was: {xml}");
    }

    #[test]
    fn round_trips_owns_false() {
        let reference = Reference::new(HRef::from_local("some-id"));

        let xml = serialize(&reference);
        let recovered = deserialize_reference(xml.as_bytes()).unwrap();

        assert_eq!(recovered, reference, "xml was: {xml}");
        assert!(!recovered.owns());
    }

    #[test]
    fn round_trips_predecessor_xml() {
        let xml =
            r##"<tran:predecessor xlink:href="#UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3"/>"##;

        let reference = deserialize_reference(xml.as_bytes()).unwrap();

        assert_eq!(
            reference.href(),
            Some(&HRef::from_local(
                "UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3"
            ))
        );
        assert!(!reference.owns());

        let output = serialize(&reference);

        assert_eq!(output, xml);
    }
}
