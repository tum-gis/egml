use egml_core::model::basic::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlCode {
    #[serde(rename = "@codeSpace")]
    pub code_space: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

impl From<GmlCode> for Code {
    fn from(item: GmlCode) -> Self {
        Self {
            code_space: item.code_space,
            value: item.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::basic::Code;
    use quick_xml::de;

    #[test]
    fn test_parse_code() {
        let xml_document = b"
          <tran:function>2</tran:function>";

        let gml_code: GmlCode = de::from_reader(xml_document.as_ref()).expect("should work");
        let code = Code::from(gml_code);

        assert!(code.code_space.is_none());
        assert_eq!(code.value, "2");
    }

    #[test]
    fn test_parse_code_with_code_space() {
        let xml_document = b"
          <bldg:class codeSpace=\"http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml\">1000</bldg:class>";

        let gml_code: GmlCode = de::from_reader(xml_document.as_ref()).expect("should work");
        let code = Code::from(gml_code);

        assert_eq!(
            code.code_space,
            Some(
                "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml".to_string()
            )
        );
        assert_eq!(code.value, "1000");
    }
}
