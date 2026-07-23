use crate::Error;
use crate::codec::abstract_object::{deserialize_abstract_object, serialize_abstract_object};
use crate::codec::basic::GmlCode;
use crate::util::{
    Formatting, GmlElement, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner,
};
use egml_core::model::AsAbstractObject;
use egml_core::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut, Id};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_gml(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractGml, Error> {
    let parsed: GmlAbstractGml = de::from_reader(xml_document)?;
    let abstract_object = deserialize_abstract_object(xml_document, spans)?;
    let mut abstract_gml = AbstractGml::from_abstract_object(abstract_object);

    let id = match parsed.id.as_ref() {
        Some(s) => Some(Id::try_from(s.as_str())?),
        None => None,
    };
    abstract_gml.set_id_opt(id);
    abstract_gml.set_names(parsed.names.into_iter().map(Into::into).collect());

    Ok(abstract_gml)
}

pub fn serialize_abstract_gml(
    abstract_gml: &AbstractGml,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_object(abstract_gml.abstract_object(), formatting)?;

    if let Some(id) = abstract_gml.id() {
        xml_node_parts
            .attributes
            .push(("gml:id".to_string(), id.to_string()));
    }

    if let Some(raw) = serialize_inner(GmlAbstractGml::from(abstract_gml), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractGml {
    #[serde(rename(deserialize = "@id"), skip_serializing)]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "gml:name", deserialize = "name"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub names: Vec<GmlCode>,
}

impl From<&AbstractGml> for GmlAbstractGml {
    fn from(item: &AbstractGml) -> Self {
        Self {
            id: item.id().as_ref().map(|x| x.to_string()),
            names: item.names().iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::base::abstract_gml::{deserialize_abstract_gml, serialize_abstract_gml};
    use crate::util::{Formatting, GmlElement, XmlNode, XmlNodeParts, extract_xml_element_spans};
    use egml_core::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut, Id};

    fn render(parts: XmlNodeParts) -> String {
        XmlNode::new(GmlElement::Polygon.into(), parts)
            .to_string(Formatting::Compact)
            .unwrap()
    }

    #[test]
    fn deserialize_simple_abstract_gml() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
            <gml:name>Name1</gml:name>
            <gml:name>Name2</gml:name>
        </ExampleFeature>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let parsed_gml = deserialize_abstract_gml(xml_document.as_ref(), &spans).unwrap();

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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref(), &spans).expect("");
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref(), &spans).expect("");
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref(), &spans).expect("");
        assert_eq!(abstract_gml.names().len(), 1);
    }

    #[test]
    fn deserialize_abstract_gml_with_empty_name() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
            <gml:name/>
        </ExampleFeature>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref(), &spans).expect("");
        assert_eq!(abstract_gml.names().len(), 1);
    }

    #[test]
    fn serialize_abstract_gml_empty() {
        let gml = AbstractGml::new();
        let xml_node_parts = serialize_abstract_gml(&gml, Formatting::Compact).unwrap();
        assert_eq!(render(xml_node_parts), "<gml:Polygon/>");
    }

    #[test]
    fn serialize_abstract_gml_with_id() {
        let id = Id::try_from("UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86").unwrap();
        let gml = AbstractGml::with_id(id);
        let xml_node_parts = serialize_abstract_gml(&gml, Formatting::Compact).unwrap();
        assert_eq!(
            render(xml_node_parts),
            r#"<gml:Polygon gml:id="UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86"/>"#,
        );
    }

    #[test]
    fn serialize_abstract_gml_with_name() {
        let mut gml = AbstractGml::new();
        gml.push_name("Name1".into());
        let xml_node_parts = serialize_abstract_gml(&gml, Formatting::Compact).unwrap();
        assert!(render(xml_node_parts).contains("<gml:name>Name1</gml:name>"));
    }

    #[test]
    fn serialize_abstract_gml_with_id_and_names() {
        let id = Id::try_from("UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86").unwrap();
        let mut gml = AbstractGml::with_id(id);
        gml.push_name("Name1".into());
        gml.push_name("Name2".into());
        let xml_node_parts = serialize_abstract_gml(&gml, Formatting::Compact).unwrap();
        let xml = render(xml_node_parts);
        assert!(xml.contains(r#"gml:id="UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86""#));
        assert!(xml.contains("<gml:name>Name1</gml:name>"));
        assert!(xml.contains("<gml:name>Name2</gml:name>"));
    }
}
