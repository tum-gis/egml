use crate::error::GmlIoError;
use egml_core::DirectPosition;
use quick_xml::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:Point")]
struct Point {
    #[serde(rename = "@id")]
    id: String,
    pos: Pos,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "gml:pos")]
struct Pos {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl From<Pos> for DirectPosition {
    fn from(item: Pos) -> Self {
        assert_eq!(item.srs_dimension.unwrap_or(3), 3);

        let parsed_values: Vec<f64> = item
            .value
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(parsed_values.len(), 3);

        Self::new(parsed_values[0], parsed_values[1], parsed_values[2]).unwrap()
    }
}

pub fn parse_point(source_text: &str) -> Result<DirectPosition, GmlIoError> {
    let parsed_point: Point = de::from_str(source_text)?;

    let point: DirectPosition = parsed_point.pos.into();
    Ok(point)
}
