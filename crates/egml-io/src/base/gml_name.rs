use crate::Error;
use egml_core::model::base::{AbstractGml, Id};
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct GmlAbstractGml {
    #[serde(rename = "$value")]
    content: Vec<GmlAbstractGmlContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum GmlAbstractGmlContent {
    #[serde(rename = "name")]
    Name(GmlName),

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct GmlName {
    #[serde(rename = "$value", default)]
    value: String,
}

pub fn parse_abstract_gml<R: BufRead>(reader: R, id: Id) -> Result<AbstractGml, Error> {
    let parsed_abstract_gml: GmlAbstractGml = de::from_reader(reader)?;

    let names: Vec<String> = parsed_abstract_gml
        .content
        .into_iter()
        .filter_map(|content| match content {
            GmlAbstractGmlContent::Name(name) => Some(name.value),
            _ => None,
        })
        .collect();

    Ok(AbstractGml {
        id: Some(id),
        name: names,
    })
}

#[cfg(test)]
mod tests {
    use crate::base::gml_name::parse_abstract_gml;
    use egml_core::model::base::Id;

    #[test]
    fn parsing_abstract_gml_with_two_names() {
        let id = Id::from_hashed_string("test_id");
        let xml_document = b"<gml:name>my_name_1</gml:name><gml:name>my_name_2</gml:name>";

        let abstract_gml = parse_abstract_gml(xml_document.as_ref(), id).expect("");
        assert_eq!(abstract_gml.name, vec!["my_name_1", "my_name_2"]);
    }

    #[test]
    fn parsing_abstract_gml_with_other_elements() {
        let id = Id::from_hashed_string("test_id");
        let xml_document = b"
      <gml:name>0507</gml:name>
      <gml:boundedBy>
        <gml:Envelope srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\">
          <gml:lowerCorner>690984.6702564997 5336061.499479238 507.40999999999997</gml:lowerCorner>
          <gml:upperCorner>691088.7286507211 5336156.516425779 537.924940696475</gml:upperCorner>
        </gml:Envelope>
      </gml:boundedBy>";

        let abstract_gml = parse_abstract_gml(xml_document.as_ref(), id).expect("");
        assert_eq!(abstract_gml.name, vec!["0507"]);
    }

    #[test]
    fn parsing_nested_abstract_gml() {
        let id = Id::from_hashed_string("test_id");
        let xml_document = b"
      <gml:name>0507</gml:name>
      <con:Window>
        <gml:name>window 23</gml:name>
      </con:Window>";

        let abstract_gml = parse_abstract_gml(xml_document.as_ref(), id).expect("");
        assert_eq!(abstract_gml.name.len(), 1);
    }

    #[test]
    fn parsing_abstract_gml_with_empty_name() {
        let id = Id::from_hashed_string("test_id");
        let xml_document = b"<gml:name/>";

        let abstract_gml = parse_abstract_gml(xml_document.as_ref(), id).expect("");
        assert_eq!(abstract_gml.name.len(), 1);
    }
}
