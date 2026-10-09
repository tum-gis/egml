use crate::Error;
use crate::codec::xal::enums::{
    identifier_element_type_str, parse_identifier_element_type, parse_thoroughfare_name_type,
    thoroughfare_name_type_str,
};
use crate::util::{
    DeserializationConfig, XalElement, XalNamespace, XmlDocumentIndex, XmlFragmentWriter,
};
use egml_core::model::xal::enums::{IdentifierElementType, ThoroughfareNameType};
use egml_core::model::xal::{Identifier, Thoroughfare, ThoroughfareName, ThoroughfareNameOrNumber};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;
use std::ops::Range;

/// `ThoroughfareType`'s content model is `<xs:choice maxOccurs="unbounded">`
/// of `NameElement`/`Number` — the two kinds are interleaved in document
/// order, unlike `Country`/`Locality`'s flat `NameElement` list. `index`
/// gives each match's own span regardless of which of the two it is, so
/// sorting the combined spans by start offset reconstructs that order
/// instead of grouping same-typed matches together the way
/// [`collect_children`](crate::util::collect_children) would.
pub fn deserialize_thoroughfare(
    xml_document: &[u8],
    index: &XmlDocumentIndex<XalElement>,
    config: &DeserializationConfig,
) -> Result<Thoroughfare, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let mut spans: Vec<(Range<usize>, XalElement)> = index
        .get(XalElement::NameElement)
        .iter()
        .map(|child| (child.range(), XalElement::NameElement))
        .chain(
            index
                .get(XalElement::Number)
                .iter()
                .map(|child| (child.range(), XalElement::Number)),
        )
        .collect();
    spans.sort_by_key(|(range, _)| range.start);

    let mut elements = Vec::with_capacity(spans.len());
    for (range, kind) in spans {
        let slice = &xml_document[range];
        match kind {
            XalElement::NameElement => {
                elements.push(ThoroughfareNameOrNumber::Name(
                    deserialize_thoroughfare_name(slice)?,
                ));
            }
            XalElement::Number => {
                elements.push(ThoroughfareNameOrNumber::Number(deserialize_number(slice)?));
            }
            _ => unreachable!("only NameElement/Number spans were collected"),
        }
    }

    let mut thoroughfare = Thoroughfare::new(elements);
    let (thoroughfare_type, type_code) = read_thoroughfare_attributes(xml_document)?;
    if let Some(thoroughfare_type) = thoroughfare_type {
        thoroughfare.set_thoroughfare_type(thoroughfare_type);
    }
    if let Some(type_code) = type_code {
        thoroughfare.set_type_code(type_code);
    }

    Ok(thoroughfare)
}

fn read_thoroughfare_attributes(
    xml_document: &[u8],
) -> Result<(Option<String>, Option<String>), Error> {
    let mut reader = Reader::from_reader(xml_document);
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                let mut thoroughfare_type = None;
                let mut type_code = None;
                for attr in start.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
                    match attr.key.local_name().as_ref() {
                        "Type" => thoroughfare_type = Some(value),
                        "TypeCode" => type_code = Some(value),
                        _ => {}
                    }
                }
                return Ok((thoroughfare_type, type_code));
            }
            Event::Eof => return Ok((None, None)),
            _ => {}
        }
    }
}

fn deserialize_thoroughfare_name(xml_document: &[u8]) -> Result<ThoroughfareName, Error> {
    let mut reader = Reader::from_reader(xml_document);
    let mut name_type = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => {
                name_type = read_name_type_attribute(&start)?;
            }
            Event::Empty(start) => {
                name_type = read_name_type_attribute(&start)?;
                return Ok(ThoroughfareName::with_optional_name_type(
                    String::new(),
                    name_type,
                ));
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return Ok(ThoroughfareName::with_optional_name_type(
                    raw.into_owned(),
                    name_type,
                ));
            }
            Event::CData(cdata) => {
                return Ok(ThoroughfareName::with_optional_name_type(
                    cdata.as_ref().to_string(),
                    name_type,
                ));
            }
            Event::End(_) => {
                return Ok(ThoroughfareName::with_optional_name_type(
                    String::new(),
                    name_type,
                ));
            }
            Event::Eof => {
                return Err(Error::ElementNotFound(
                    "xAL:NameElement text content".to_string(),
                ));
            }
            _ => {}
        }
    }
}

fn read_name_type_attribute(start: &BytesStart) -> Result<Option<ThoroughfareNameType>, Error> {
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() != "NameType" {
            continue;
        }
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?;
        return Ok(parse_thoroughfare_name_type(value.as_ref()));
    }
    Ok(None)
}

fn deserialize_number(xml_document: &[u8]) -> Result<Identifier, Error> {
    let mut reader = Reader::from_reader(xml_document);
    let mut identifier_type = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => {
                identifier_type = read_identifier_type_attribute(&start)?;
            }
            Event::Empty(start) => {
                identifier_type = read_identifier_type_attribute(&start)?;
                return Ok(Identifier::with_optional_identifier_type(
                    String::new(),
                    identifier_type,
                ));
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return Ok(Identifier::with_optional_identifier_type(
                    raw.into_owned(),
                    identifier_type,
                ));
            }
            Event::CData(cdata) => {
                return Ok(Identifier::with_optional_identifier_type(
                    cdata.as_ref().to_string(),
                    identifier_type,
                ));
            }
            Event::End(_) => {
                return Ok(Identifier::with_optional_identifier_type(
                    String::new(),
                    identifier_type,
                ));
            }
            Event::Eof => {
                return Err(Error::ElementNotFound(
                    "xAL:Number text content".to_string(),
                ));
            }
            _ => {}
        }
    }
}

fn read_identifier_type_attribute(
    start: &BytesStart,
) -> Result<Option<IdentifierElementType>, Error> {
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() != "Type" {
            continue;
        }
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?;
        return Ok(parse_identifier_element_type(value.as_ref()));
    }
    Ok(None)
}

pub fn serialize_thoroughfare<W: Write>(
    thoroughfare: &Thoroughfare,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    if let Some(thoroughfare_type) = thoroughfare.thoroughfare_type() {
        attributes.push(("xAL:Type", thoroughfare_type));
    }
    if let Some(type_code) = thoroughfare.type_code() {
        attributes.push(("xAL:TypeCode", type_code));
    }
    xml_fragment_writer.write_start_event_with_attributes(
        XalNamespace::Xal,
        XalElement::Thoroughfare,
        attributes,
    )?;

    for element in thoroughfare.elements() {
        match element {
            ThoroughfareNameOrNumber::Name(name) => match name.name_type() {
                Some(name_type) => xml_fragment_writer.write_leaf_element_with_attributes(
                    XalNamespace::Xal,
                    XalElement::NameElement,
                    [("xAL:NameType", thoroughfare_name_type_str(name_type))],
                    name.content(),
                )?,
                None => xml_fragment_writer.write_leaf_element_with_attributes(
                    XalNamespace::Xal,
                    XalElement::NameElement,
                    [] as [(&str, &str); 0],
                    name.content(),
                )?,
            },
            ThoroughfareNameOrNumber::Number(identifier) => match identifier.identifier_type() {
                Some(identifier_type) => xml_fragment_writer.write_leaf_element_with_attributes(
                    XalNamespace::Xal,
                    XalElement::Number,
                    [("xAL:Type", identifier_element_type_str(identifier_type))],
                    identifier.content(),
                )?,
                None => xml_fragment_writer.write_leaf_element_with_attributes(
                    XalNamespace::Xal,
                    XalElement::Number,
                    [] as [(&str, &str); 0],
                    identifier.content(),
                )?,
            },
        }
    }

    xml_fragment_writer.write_end_event(XalNamespace::Xal, XalElement::Thoroughfare)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_thoroughfare, serialize_thoroughfare};
    use crate::util::{
        DeserializationConfig, Formatting, XalElement, XmlDocumentIndex, XmlFragmentWriter,
    };
    use egml_core::model::xal::enums::ThoroughfareNameType;
    use egml_core::model::xal::{
        Identifier, Thoroughfare, ThoroughfareName, ThoroughfareNameOrNumber,
    };

    fn baker_street() -> Thoroughfare {
        let mut thoroughfare = Thoroughfare::new(vec![
            ThoroughfareNameOrNumber::Number(Identifier::new("39".to_string())),
            ThoroughfareNameOrNumber::Name(ThoroughfareName::with_name_type(
                "Baker".to_string(),
                ThoroughfareNameType::NameOnly,
            )),
        ]);
        thoroughfare.set_thoroughfare_type("Street");
        thoroughfare
    }

    #[test]
    fn deserialize_thoroughfare_preserves_document_order() {
        let xml_document = br#"<xAL:Thoroughfare xAL:Type="Street"><xAL:Number>39</xAL:Number><xAL:NameElement xAL:NameType="NameOnly">Baker</xAL:NameElement></xAL:Thoroughfare>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let thoroughfare =
            deserialize_thoroughfare(xml_document, &index, &DeserializationConfig::default())
                .expect("should deserialize");

        assert_eq!(thoroughfare, baker_street());
    }

    #[test]
    fn serialize_thoroughfare_writes_elements_in_order() {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_thoroughfare(&baker_street(), &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Thoroughfare xAL:Type="Street"><xAL:Number>39</xAL:Number><xAL:NameElement xAL:NameType="NameOnly">Baker</xAL:NameElement></xAL:Thoroughfare>"#
        );
    }

    #[test]
    fn round_trip_thoroughfare() {
        let original = baker_street();

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_thoroughfare(&original, &mut writer).unwrap();
        let xml = writer.into_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(&xml, None).unwrap();
        let recovered =
            deserialize_thoroughfare(&xml, &index, &DeserializationConfig::default()).unwrap();

        assert_eq!(recovered, original);
    }
}
