use crate::Error;
use crate::Error::{MissingElements, Only3DSupported};
use crate::util::deserialize_space_separated_f64;
use egml_core::model::geometry::DirectPosition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlDirectPosition {
    #[serde(rename = "@srsDimension")]
    pub(crate) srs_dimension: Option<u32>,

    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_space_separated_f64"
    )]
    value: Vec<f64>,
}

impl TryFrom<GmlDirectPosition> for DirectPosition {
    type Error = Error;

    fn try_from(item: GmlDirectPosition) -> Result<Self, Self::Error> {
        if item.srs_dimension.unwrap_or(3) != 3 {
            return Err(Only3DSupported());
        }

        if item.value.len() != 3 {
            return Err(MissingElements("not a multiple of 3".to_string()));
        }

        let position = DirectPosition::new(item.value[0], item.value[1], item.value[2])?;
        Ok(position)
    }
}

#[cfg(test)]
mod tests {
    use crate::GmlDirectPosition;
    use egml_core::model::geometry::DirectPosition;
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_point() {
        let xml_document = "<gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>";

        let parsed_result: Result<GmlDirectPosition, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let direct_position: DirectPosition = parsed_gml.try_into().unwrap();

        assert_eq!(direct_position.x(), 678000.9484065345);
        assert_eq!(direct_position.y(), 5403659.060043676);
        assert_eq!(direct_position.z(), 417.3802376791456);
    }
}
