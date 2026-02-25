use crate::Error;
use crate::Error::{MissingElements, Only3DSupported};
use crate::util::deserialize_space_separated_f64;
use egml_core::model::geometry::DirectPosition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlDirectPositionList {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,

    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_space_separated_f64"
    )]
    values: Vec<f64>,
}

impl TryFrom<GmlDirectPositionList> for Vec<DirectPosition> {
    type Error = Error;

    fn try_from(item: GmlDirectPositionList) -> Result<Self, Self::Error> {
        if item.srs_dimension.unwrap_or(3) != 3 {
            return Err(Only3DSupported());
        }

        if !item.values.len().is_multiple_of(3) {
            return Err(MissingElements("not a multiple of 3".to_string()));
        }

        let mut points: Vec<DirectPosition> = Vec::new();
        for chunk in item.values.chunks_exact(3) {
            let point =
                DirectPosition::new(chunk[0], chunk[1], chunk[2]).expect("should be valid point");
            points.push(point);
        }

        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::direct_position_list::GmlDirectPositionList;
    use egml_core::model::geometry::DirectPosition;
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_position_list() {
        let xml_document = b"<gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>";

        let parsed_result: Result<GmlDirectPositionList, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let position_list: Vec<DirectPosition> = parsed_gml.try_into().unwrap();

        assert_eq!(position_list.len(), 4);
    }
}
