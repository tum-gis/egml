use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use quick_xml::de;
use std::io::Write;

pub fn deserialize_i64(xml_document: &[u8]) -> Result<i64, Error> {
    de::from_reader(xml_document).map_err(Error::from)
}

pub fn serialize_i64<N: XmlNamespace, E: XmlElement, W: Write>(
    value: i64,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let content = value.to_string();

    xml_fragment_writer.write_leaf_element(target_xml_namespace, target_xml_element, &content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, XmlFragmentWriter};

    /// Minimal element fixture standing in for a real vocabulary's element
    /// type (e.g. a future `bldg:storeysAboveGround`), with no namespace
    /// prefix — matching the unqualified elements in the test XML below.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct NoNamespace;

    impl XmlNamespace for NoNamespace {
        fn uri(&self) -> &'static str {
            "urn:x-none"
        }

        fn default_prefix(&self) -> Option<&'static str> {
            None
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct StoreysAboveGround;

    impl XmlElement for StoreysAboveGround {
        fn from_local_name(local_name: &str) -> Option<Self> {
            (local_name == "storeysAboveGround").then_some(Self)
        }

        fn local_name(&self) -> &'static str {
            "storeysAboveGround"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct StoreysBelowGround;

    impl XmlElement for StoreysBelowGround {
        fn from_local_name(local_name: &str) -> Option<Self> {
            (local_name == "storeysBelowGround").then_some(Self)
        }

        fn local_name(&self) -> &'static str {
            "storeysBelowGround"
        }
    }

    #[test]
    fn test_deserialize_i64() {
        let xml_document = b"<storeysAboveGround>4</storeysAboveGround>";

        let value = deserialize_i64(xml_document).expect("should work");

        assert_eq!(value, 4);
    }

    #[test]
    fn test_deserialize_i64_negative() {
        let xml_document = b"<value>-3</value>";

        let value = deserialize_i64(xml_document).expect("should work");

        assert_eq!(value, -3);
    }

    #[test]
    fn test_serialize_i64() {
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_i64(4, &mut xml_fragment_writer, NoNamespace, StoreysAboveGround)
            .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert_eq!(xml, "<storeysAboveGround>4</storeysAboveGround>");
    }

    #[test]
    fn test_round_trip_i64() {
        let xml_document = b"<storeysBelowGround>2</storeysBelowGround>";

        let value = deserialize_i64(xml_document).expect("should work");

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_i64(
            value,
            &mut xml_fragment_writer,
            NoNamespace,
            StoreysBelowGround,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        let round_tripped = deserialize_i64(xml.as_bytes()).expect("should work");

        assert_eq!(value, round_tripped);
    }
}
