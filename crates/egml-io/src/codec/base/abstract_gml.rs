use crate::Error;
use crate::codec::abstract_object::{
    deserialize_abstract_object, serialize_abstract_object_attributes,
};
use crate::codec::basic::{deserialize_code, serialize_code};
use crate::util::{
    DeserializationConfig, GmlAttribute, GmlElement, GmlNamespace, XmlDocumentIndex, XmlElement,
    XmlFragmentWriter, deserialize_gml_attributes,
};
use egml_core::model::AsAbstractObject;
use egml_core::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut, Id};
use egml_core::model::basic_types::Code;
use std::io::Write;

pub fn deserialize_abstract_gml(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractGml, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_gml received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_object = deserialize_abstract_object(xml_document, index, config)?;
    let mut abstract_gml = AbstractGml::from_abstract_object(abstract_object);

    let attributes = deserialize_gml_attributes(xml_document)?;
    let id = attributes
        .get(&GmlAttribute::Id)
        .map(|s| Id::try_from(s.as_str()))
        .transpose()?;
    abstract_gml.set_id_opt(id);

    let names: Vec<Code> = index
        .get(GmlElement::NameProperty)
        .iter()
        .map(|x| deserialize_code(&xml_document[x.range()]))
        .collect::<Result<_, _>>()?;
    abstract_gml.set_names(names);

    Ok(abstract_gml)
}

pub fn serialize_abstract_gml<W: Write>(
    abstract_gml: &AbstractGml,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    for name in abstract_gml.names() {
        serialize_code(
            name,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )?;
    }

    Ok(())
}

pub fn serialize_abstract_gml_attributes(abstract_gml: &AbstractGml) -> Vec<(String, String)> {
    let mut attributes = serialize_abstract_object_attributes(abstract_gml.abstract_object());

    if let Some(id) = abstract_gml.id() {
        attributes.push((GmlAttribute::Id.local_name().to_string(), id.to_string()));
    }

    attributes
}

#[cfg(test)]
mod tests {
    use crate::codec::base::abstract_gml::{
        deserialize_abstract_gml, serialize_abstract_gml, serialize_abstract_gml_attributes,
    };
    use crate::util::{Formatting, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut, Id};

    fn render_content(gml: &AbstractGml) -> String {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_gml(gml, &mut writer).unwrap();
        String::from_utf8(writer.into_bytes()).unwrap()
    }

    #[test]
    fn deserialize_simple_abstract_gml() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
            <gml:name>Name1</gml:name>
            <gml:name>Name2</gml:name>
        </ExampleFeature>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let parsed_gml = deserialize_abstract_gml(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(
            parsed_gml.id().unwrap().to_string(),
            "UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86"
        );
        assert_eq!(parsed_gml.names().len(), 2);
        assert_eq!(parsed_gml.names()[0], "Name1".into());
        assert_eq!(parsed_gml.names()[1], "Name2".into());
    }

    #[test]
    fn deserialize_abstract_gml_with_two_names() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
            <gml:name>my_name_1</gml:name>
            <gml:name>my_name_2</gml:name>
        </ExampleFeature>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_gml = deserialize_abstract_gml(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("");
        assert_eq!(
            abstract_gml.names(),
            vec!["my_name_1".into(), "my_name_2".into()]
        );
    }

    #[test]
    fn deserialize_abstract_gml_ignores_non_name_elements() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
          <gml:name>0507</gml:name>
          <gml:boundedBy>
            <gml:Envelope srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\">
              <gml:lowerCorner>690984.6702564997 5336061.499479238 507.40999999999997</gml:lowerCorner>
              <gml:upperCorner>691088.7286507211 5336156.516425779 537.924940696475</gml:upperCorner>
            </gml:Envelope>
          </gml:boundedBy>
        </ExampleFeature>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_gml = deserialize_abstract_gml(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("");
        assert_eq!(abstract_gml.names(), vec!["0507".into()]);
    }

    #[test]
    fn deserialize_abstract_gml_ignores_nested_names() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
          <gml:name>0507</gml:name>
          <con:Window>
            <gml:name>window 23</gml:name>
          </con:Window>
        </ExampleFeature>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_gml = deserialize_abstract_gml(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("");
        assert_eq!(abstract_gml.names().len(), 1);
    }

    #[test]
    fn deserialize_abstract_gml_with_empty_name() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
            <gml:name/>
        </ExampleFeature>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_gml = deserialize_abstract_gml(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("");
        assert_eq!(abstract_gml.names().len(), 1);
    }

    #[test]
    fn serialize_abstract_gml_attributes_empty_without_id() {
        let gml = AbstractGml::new();
        let attributes = serialize_abstract_gml_attributes(&gml);
        assert!(attributes.is_empty());
    }

    #[test]
    fn serialize_abstract_gml_attributes_includes_id() {
        let id = Id::try_from("UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86").unwrap();
        let gml = AbstractGml::with_id(id);
        let attributes = serialize_abstract_gml_attributes(&gml);
        assert_eq!(
            attributes,
            vec![(
                "gml:id".to_string(),
                "UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86".to_string()
            )]
        );
    }

    #[test]
    fn serialize_abstract_gml_writes_no_content_without_names() {
        let gml = AbstractGml::new();
        assert_eq!(render_content(&gml), "");
    }

    #[test]
    fn serialize_abstract_gml_writes_name_elements() {
        let mut gml = AbstractGml::new();
        gml.push_name("Name1".into());
        gml.push_name("Name2".into());

        let xml = render_content(&gml);
        assert!(xml.contains("<gml:name>Name1</gml:name>"));
        assert!(xml.contains("<gml:name>Name2</gml:name>"));
    }
}
