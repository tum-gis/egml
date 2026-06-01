use crate::Error;
use egml_core::model::base::{AbstractGml, Id};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_gml(xml_document: &[u8]) -> Result<AbstractGml, Error> {
    let parsed_gml: GmlAbstractGml = de::from_reader(xml_document)?;

    let id = match parsed_gml.id.as_ref() {
        Some(s) => Some(Id::try_from(s)?),
        None => None,
    };
    let mut abstract_gml = AbstractGml::new();
    abstract_gml.id = id;
    abstract_gml.name = parsed_gml.name;

    Ok(abstract_gml)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractGml {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "gml:name", deserialize = "name"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name: Vec<String>,
}

impl From<&AbstractGml> for GmlAbstractGml {
    fn from(item: &AbstractGml) -> Self {
        Self {
            id: item.id.as_ref().map(|x| x.to_string()),
            name: item.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::abstract_gml::deserialize_abstract_gml;

    #[test]
    fn deserialize_simple_abstract_gml() {
        let xml_document = "<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
            <gml:name>Name1</gml:name>
            <gml:name>Name2</gml:name>
        </ExampleFeature>";

        let parsed_gml = deserialize_abstract_gml(xml_document.as_ref()).unwrap();

        assert_eq!(
            parsed_gml.id.unwrap().to_string(),
            "UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86"
        );
        assert_eq!(parsed_gml.name.len(), 2);
        assert_eq!(parsed_gml.name[0], "Name1");
        assert_eq!(parsed_gml.name[1], "Name2");
    }

    #[test]
    fn deserialize_abstract_gml_with_two_names() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
            <gml:name>my_name_1</gml:name>
            <gml:name>my_name_2</gml:name>
        </ExampleFeature>";

        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref()).expect("");
        assert_eq!(abstract_gml.name, vec!["my_name_1", "my_name_2"]);
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

        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref()).expect("");
        assert_eq!(abstract_gml.name, vec!["0507"]);
    }

    #[test]
    fn deserialize_abstract_gml_ignores_nested_names() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
          <gml:name>0507</gml:name>
          <con:Window>
            <gml:name>window 23</gml:name>
          </con:Window>
        </ExampleFeature>";

        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref()).expect("");
        assert_eq!(abstract_gml.name.len(), 1);
    }

    #[test]
    fn deserialize_abstract_gml_with_empty_name() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7\">
            <gml:name/>
        </ExampleFeature>";

        let abstract_gml = deserialize_abstract_gml(xml_document.as_ref()).expect("");
        assert_eq!(abstract_gml.name.len(), 1);
    }
}
