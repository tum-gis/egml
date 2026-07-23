use egml_core::model::basic_types::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlCode {
    #[serde(rename = "@codeSpace", skip_serializing_if = "Option::is_none")]
    pub code_space: Option<String>,

    #[serde(rename = "$value", default)]
    pub value: String,
}

impl From<GmlCode> for Code {
    fn from(item: GmlCode) -> Self {
        Self::from_parts(item.code_space, item.value)
    }
}

impl From<&Code> for GmlCode {
    fn from(code: &Code) -> Self {
        Self {
            code_space: code.code_space().map(str::to_string),
            value: code.value().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::basic_types::Code;
    use quick_xml::{de, se};

    #[test]
    fn deserialize_code() {
        let xml = b"<tran:function>2</tran:function>";

        let gml_code: GmlCode = de::from_reader(xml.as_ref()).expect("should work");
        let code = Code::from(gml_code);

        assert!(code.code_space().is_none());
        assert_eq!(code.value(), "2");
    }

    #[test]
    fn deserialize_code_with_code_space() {
        let xml = b"<bldg:class codeSpace=\"http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml\">1000</bldg:class>";

        let gml_code: GmlCode = de::from_reader(xml.as_ref()).expect("should work");
        let code = Code::from(gml_code);

        assert_eq!(
            code.code_space(),
            Some(
                "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml"
            )
        );
        assert_eq!(code.value(), "1000");
    }

    #[test]
    fn serialize_code_without_code_space() {
        let gml_code = GmlCode {
            code_space: None,
            value: "2".to_string(),
        };
        let xml = se::to_string_with_root("tran:function", &gml_code).unwrap();

        assert!(xml.contains("2"));
        assert!(!xml.contains("codeSpace"));
    }

    #[test]
    fn serialize_code_with_code_space() {
        let gml_code = GmlCode {
            code_space: Some("http://example.org/codes.xml".to_string()),
            value: "1000".to_string(),
        };
        let xml = se::to_string_with_root("bldg:class", &gml_code).unwrap();

        assert!(xml.contains("1000"));
        assert!(xml.contains("codeSpace=\"http://example.org/codes.xml\""));
    }

    #[test]
    fn round_trip_without_code_space() {
        let original = GmlCode {
            code_space: None,
            value: "42".to_string(),
        };
        let xml = se::to_string_with_root("tran:function", &original).unwrap();
        let parsed: GmlCode = de::from_str(&xml).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn round_trip_with_code_space() {
        let original = GmlCode {
            code_space: Some("http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml".to_string()),
            value: "1000".to_string(),
        };
        let xml = se::to_string_with_root("bldg:class", &original).unwrap();
        let parsed: GmlCode = de::from_str(&xml).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn from_code_conversion() {
        let code = Code::with_code_space("http://example.org/codes.xml", "residential");
        let gml_code = GmlCode::from(&code);

        assert_eq!(gml_code.code_space.as_deref(), code.code_space());
        assert_eq!(gml_code.value, code.value());
    }
}
