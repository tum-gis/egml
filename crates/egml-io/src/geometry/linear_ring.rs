use egml_core::{DirectPosition, LinearRing};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "gml:exterior")]
pub struct LinearRingElement {
    #[serde(rename = "$value")]
    pub pos_list: Option<PosListElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "gml:posList")]
pub struct PosListElement {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl From<&PosListElement> for LinearRing {
    fn from(item: &PosListElement) -> Self {
        assert_eq!(item.srs_dimension.unwrap_or(3), 3);

        let parsed_values: Vec<f64> = item
            .value
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(parsed_values.len() % 3, 0);

        let mut points: Vec<DirectPosition> = Vec::new();
        for chunk in parsed_values.chunks(3) {
            let point = DirectPosition::new(chunk[0], chunk[1], chunk[2]).unwrap();
            points.push(point);
        }

        // TODO: handle error during into
        LinearRing::new(points).unwrap()
    }
}
