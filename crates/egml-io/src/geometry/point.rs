use crate::error::Error;
use crate::error::Error::{MissingElements, Only3DSupported};
use egml_core::DirectPosition;
use quick_xml::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:Point")]
struct GmlPoint {
    #[serde(rename = "@id", default)]
    id: String,
    pos: GmlPos,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:pos")]
struct GmlPos {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl GmlPos {
    pub fn to_direct_position(self) -> Result<DirectPosition, Error> {
        if self.srs_dimension.unwrap_or(3) != 3 {
            return Err(Only3DSupported());
        }

        let parsed_values: Vec<f64> = self
            .value
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if parsed_values.len() % 3 != 0 {
            return Err(MissingElements());
        }

        let position = DirectPosition::new(parsed_values[0], parsed_values[1], parsed_values[2])?;
        Ok(position)
    }
}

pub fn parse_point(source_text: &str) -> Result<DirectPosition, Error> {
    let parsed_point: GmlPoint = de::from_str(source_text)?;

    let point: DirectPosition = parsed_point.pos.to_direct_position()?;
    Ok(point)
}

#[cfg(test)]
mod tests {
    use crate::parse_point;

    #[test]
    fn parsing_point() {
        let source_text = "<gml:Point>
              <gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let _result = parse_point(source_text).unwrap();
    }

    #[test]
    fn parsing_point_without_id() {
        let source_text = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let _result = parse_point(source_text).unwrap();
    }
}
