use egml_core::model::basic::Measure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlMeasure {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl From<GmlMeasure> for Measure {
    fn from(item: GmlMeasure) -> Self {
        Self {
            uom: item.uom,
            value: item.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de;

    #[test]
    fn test_parse_measure() {
        let xml_document = b"
          <gen:value uom=\"m2\">120.0</gen:value>";

        let gml_measure: GmlMeasure = de::from_reader(xml_document.as_ref()).expect("should work");
        let measure = Measure::from(gml_measure);

        assert_eq!(measure.uom, "m2");
        assert_eq!(measure.value, 120.0);
    }
}
