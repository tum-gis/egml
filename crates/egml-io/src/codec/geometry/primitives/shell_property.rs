use crate::Error;
use crate::codec::geometry::primitives::serialize_shell;
use crate::codec::geometry::primitives::shell::deserialize_shell;
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::geometry::primitives::Shell;
use std::io::Write;

/// Deserializes a `gml:ShellPropertyType` element (e.g. `gml:exterior`) into
/// the shell it wraps.
///
/// The property type carries no attributes and its shell is mandatory.
///
/// # Errors
///
/// Returns [`Error::ElementNotFound`] if the element contains no `gml:Shell`.
pub fn deserialize_shell_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Shell, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let node = index
        .first(GmlElement::Shell)
        .ok_or_else(|| Error::ElementNotFound("gml:Shell".to_string()))?;
    deserialize_shell(&xml_document[node.range()], node, config)
}

/// Serializes `shell` wrapped in a `gml:ShellPropertyType` element.
pub fn serialize_shell_property<N: XmlNamespace, E: XmlElement, W: Write>(
    shell: &Shell,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(target_xml_namespace, target_xml_element)?;
    serialize_shell(shell, xml_fragment_writer)?;
    xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::codec::geometry::primitives::shell_property::{
        deserialize_shell_property, serialize_shell_property,
    };
    use crate::util::{
        DeserializationConfig, Formatting, GmlElement, GmlNamespace, XmlDocumentIndex,
        XmlFragmentWriter,
    };
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfaceKind, AbstractSurfaceProperty, LinearRing, Polygon, Shell,
    };

    const SHELL_XML: &[u8] = b"<gml:exterior>\
        <gml:Shell>\
        <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
        <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
        </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
        </gml:Shell>\
        </gml:exterior>";

    fn make_shell() -> Shell {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(Some(AbstractRingKind::LinearRing(ring)), []).unwrap();
        let member = AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        Shell::new([member]).unwrap()
    }

    fn deserialize(xml_document: &[u8]) -> Result<Shell, Error> {
        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should index");
        deserialize_shell_property(xml_document, &index, &DeserializationConfig::default())
    }

    fn serialize(shell: &Shell) -> String {
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_shell_property(
            shell,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::ExteriorProperty,
        )
        .expect("should serialize");
        String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8")
    }

    #[test]
    fn deserialize_shell_property_test() {
        let shell = deserialize(SHELL_XML).expect("should deserialize");
        assert_eq!(shell.members().len(), 1);
    }

    #[test]
    fn deserialize_empty_property_is_an_error() {
        assert!(matches!(
            deserialize(b"<gml:exterior/>"),
            Err(Error::ElementNotFound(_))
        ));
    }

    #[test]
    fn deserialize_ignores_xlink_and_ownership_attributes() {
        // gml:ShellPropertyType declares neither attribute group; tolerate them in input.
        let xml_document = std::str::from_utf8(SHELL_XML).unwrap().replacen(
            "<gml:exterior>",
            "<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" gml:owns=\"true\">",
            1,
        );

        let shell = deserialize(xml_document.as_bytes()).expect("should deserialize");

        assert_eq!(shell.members().len(), 1);
        assert!(!serialize(&shell).contains("xlink"));
    }

    #[test]
    fn serialize_shell_property_writes_gml_tags() {
        let xml = serialize(&make_shell());

        assert!(xml.starts_with("<gml:exterior><gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_shell_property_preserves_members() {
        let shell = deserialize(SHELL_XML).expect("should deserialize");

        let recovered = deserialize(serialize(&shell).as_bytes()).expect("should deserialize");

        assert_eq!(recovered.members().len(), shell.members().len());
    }
}
