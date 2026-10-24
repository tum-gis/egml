use crate::Error;
use crate::codec::xs::lexical;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::xs::AnyUri;
use quick_xml::de;
use std::io::Write;

/// Reads an `xs:anyURI` element. Whitespace is collapsed as required by the
/// datatype's `whiteSpace=collapse` facet; any other value is accepted.
///
/// # Errors
///
/// Returns an error if the XML is not well-formed.
pub fn deserialize_any_uri(xml_document: &[u8]) -> Result<AnyUri, Error> {
    let text: String = de::from_reader(xml_document)?;

    Ok(any_uri_from_lexical(&text))
}

/// Converts an `xs:anyURI` lexical value, such as an `srsName` or `codeSpace`
/// attribute value, by collapsing its whitespace.
pub fn any_uri_from_lexical(lexical: &str) -> AnyUri {
    AnyUri::new(lexical::collapse_whitespace(lexical))
}

/// Writes an `xs:anyURI` element with the value as stored.
pub fn serialize_any_uri<N: XmlNamespace, E: XmlElement, W: Write>(
    any_uri: &AnyUri,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_leaf_element(
        target_xml_namespace,
        target_xml_element,
        any_uri.as_str(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};

    const URI: &str = "http://www.opengis.net/def/crs/EPSG/0/25832";

    /// Reads `text` as the content of an `xs:anyURI` element.
    fn read(text: &str) -> AnyUri {
        let xml_document = format!("<core:externalReference>{text}</core:externalReference>");
        deserialize_any_uri(xml_document.as_bytes()).expect("should deserialize")
    }

    /// Writes `any_uri` and returns the whole element.
    fn write(any_uri: &AnyUri) -> String {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_any_uri(
            any_uri,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        String::from_utf8(writer.into_bytes()).expect("valid UTF-8")
    }

    #[test]
    fn test_deserialize_any_uri() {
        assert_eq!(read(URI), AnyUri::new(URI));
        assert_eq!(read("#UUID_abc"), AnyUri::new("#UUID_abc"));
    }

    #[test]
    fn test_deserialize_collapses_whitespace() {
        assert_eq!(read(&format!("\n    {URI}\n  ")), AnyUri::new(URI));
        assert_eq!(read("a \t\n  b"), AnyUri::new("a b"));
    }

    #[test]
    fn test_deserialize_empty_element() {
        assert_eq!(read(""), AnyUri::default());
        assert_eq!(
            deserialize_any_uri(b"<core:externalReference/>").expect("should deserialize"),
            AnyUri::default()
        );
    }

    #[test]
    fn test_deserialize_unescapes_entities() {
        assert_eq!(
            read("http://a.org/?x=1&amp;y=2"),
            AnyUri::new("http://a.org/?x=1&y=2")
        );
    }

    #[test]
    fn test_any_uri_from_lexical() {
        assert_eq!(any_uri_from_lexical(URI), AnyUri::new(URI));
        assert_eq!(any_uri_from_lexical(" urn:a  b "), AnyUri::new("urn:a b"));
    }

    #[test]
    fn test_serialize_any_uri() {
        assert_eq!(
            write(&AnyUri::new(URI)),
            format!("<gml:name>{URI}</gml:name>")
        );
    }

    #[test]
    fn test_serialize_escapes_markup() {
        assert_eq!(
            write(&AnyUri::new("http://a.org/?x=1&y=<2>")),
            "<gml:name>http://a.org/?x=1&amp;y=&lt;2&gt;</gml:name>"
        );
    }

    #[test]
    fn test_round_trip_through_xml() {
        for text in [URI, "#UUID_abc", "http://a.org/?x=1&amp;y=2", "a b"] {
            let any_uri = read(text);

            let round_tripped =
                deserialize_any_uri(write(&any_uri).as_bytes()).expect("should deserialize");

            assert_eq!(round_tripped, any_uri);
        }
    }
}
