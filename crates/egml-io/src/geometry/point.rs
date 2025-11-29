use crate::error::Error;
use crate::error::Error::{MissingElements, Only3DSupported};
use egml_core::model::geometry::DirectPosition;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:Point")]
struct GmlPoint {
    #[serde(rename = "@id", default)]
    id: String,
    pos: GmlPos,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:pos")]
struct GmlPos {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl TryFrom<GmlPos> for DirectPosition {
    type Error = Error;

    fn try_from(value: GmlPos) -> Result<Self, Self::Error> {
        if value.srs_dimension.unwrap_or(3) != 3 {
            return Err(Only3DSupported());
        }

        let parsed_values: Vec<f64> = value
            .value
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if !parsed_values.len().is_multiple_of(3) {
            return Err(MissingElements());
        }

        let position = DirectPosition::new(parsed_values[0], parsed_values[1], parsed_values[2])?;
        Ok(position)
    }
}

pub fn parse_point<T: BufRead>(source: T) -> Result<DirectPosition, Error> {
    let parsed_point: GmlPoint = de::from_reader(source)?;
    parsed_point.pos.try_into()
}

#[cfg(test)]
mod tests {
    use crate::parse_point;

    #[test]
    fn parsing_point() {
        let source_text = "<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = parse_point(source_text.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }

    #[test]
    fn parsing_point_without_id() {
        let source_text = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let result = parse_point(source_text.as_ref()).unwrap();

        assert_eq!(result.x(), 678000.9484065345);
        assert_eq!(result.y(), 5403659.060043676);
        assert_eq!(result.z(), 417.3802376791456);
    }
}
