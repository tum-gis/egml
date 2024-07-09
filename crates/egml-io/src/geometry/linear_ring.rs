use egml_core::{DirectPosition, LinearRing};

use crate::error::Error;
use crate::error::Error::{MissingElements, Only3DSupported};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "gml:exterior")]
pub struct GmlLinearRing {
    #[serde(rename = "$value")]
    pub pos_list: Option<GmlPosList>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "gml:posList")]
pub struct GmlPosList {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl GmlPosList {
    pub fn to_linear_ring(self) -> Result<LinearRing, Error> {
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

        let mut points: Vec<DirectPosition> = Vec::new();
        for chunk in parsed_values.chunks(3) {
            let point = DirectPosition::new(chunk[0], chunk[1], chunk[2]).unwrap();
            points.push(point);
        }

        if points.first().unwrap() == points.last().unwrap() {
            points.pop();
        }

        let linear_ring = LinearRing::new(points)?;
        Ok(linear_ring)
    }
}

impl GmlLinearRing {
    pub fn to_linear_ring(self) -> Result<LinearRing, Error> {
        let pos_list: GmlPosList = self
            .pos_list
            .ok_or(Error::ElementNotFound("No element found".to_string()))
            .unwrap();
        let linear_ring: LinearRing = pos_list.to_linear_ring()?;
        Ok(linear_ring)
    }
}
