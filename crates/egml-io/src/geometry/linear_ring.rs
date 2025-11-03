use std::hash::{DefaultHasher, Hash, Hasher};

use crate::error::Error;
use crate::error::Error::{MissingElements, Only3DSupported};
use egml_core::model::base::{AbstractGml, Id};
use egml_core::model::geometry::{DirectPosition, LinearRing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:exterior")]
pub struct GmlLinearRing {
    #[serde(rename = "@id", default)]
    pub id: String,

    #[serde(rename = "$value")]
    pub pos_list: Option<GmlPosList>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:posList")]
pub struct GmlPosList {
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u32>,
    #[serde(rename = "$value")]
    value: String,
}

impl TryFrom<GmlPosList> for Vec<DirectPosition> {
    type Error = Error;

    fn try_from(value: GmlPosList) -> Result<Self, Self::Error> {
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

        let mut points: Vec<DirectPosition> = Vec::new();
        for chunk in parsed_values.chunks(3) {
            let point = DirectPosition::new(chunk[0], chunk[1], chunk[2]).unwrap();
            points.push(point);
        }

        points.dedup();
        if points.first().unwrap() == points.last().unwrap() {
            points.pop();
        }

        Ok(points)
    }
}

impl TryFrom<GmlLinearRing> for LinearRing {
    type Error = Error;

    fn try_from(value: GmlLinearRing) -> Result<Self, Self::Error> {
        let id: Id = value.id.clone().try_into().ok().unwrap_or_else(|| {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            Id::from_hashed_u64(hasher.finish())
        });
        let abstract_gml = AbstractGml::new(id);

        let pos_list: GmlPosList = value
            .pos_list
            .ok_or(Error::ElementNotFound("No element found".to_string()))
            .unwrap();
        let points: Vec<DirectPosition> = pos_list.try_into()?;

        let linear_ring = LinearRing::new(abstract_gml, points)?;
        Ok(linear_ring)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, GmlLinearRing};
    use egml_core::model::geometry::LinearRing;
    use quick_xml::de;

    #[test]
    fn parsing_linear_ring() {
        let source_text = "<gml:LinearRing gml:id=\"4018115_LR.d2yyBHNssydL8g3B8MvW\">
<gml:posList srsDimension=\"3\">
678058.447 5403817.567 424.209
678058.275 5403817.484 424.209
678058.689 5403816.628 424.209
678058.871 5403816.718 424.209
678058.447 5403817.567 424.209
</gml:posList>
</gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_str(source_text).expect("");
        let l: LinearRing = parsed_geometry.try_into().unwrap();
    }

    #[test]
    fn parsing_linear_ring_with_duplicates() {
        let source_text = "<gml:LinearRing gml:id=\"DEBY_LOD2_4959457_LR.EEAbfUPItTlOGZGH7VDv\">
                      <gml:posList>691040.851 5336002.449 529.908 691040.741 5336002.172 529.908 691040.851 5336002.449 529.908 691040.851 5336002.449 529.908</gml:posList>
                    </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_str(source_text).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_err());
    }
}
