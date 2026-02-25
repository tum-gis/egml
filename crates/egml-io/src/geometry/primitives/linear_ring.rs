use std::hash::Hasher;

use crate::GmlDirectPosition;
use crate::error::Error;
use crate::geometry::direct_position_list::GmlDirectPositionList;
use egml_core::model::base::AsAbstractGmlMut;
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractRing, LinearRing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRing {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "$value")]
    pub content: Option<GmlLinearRingContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlLinearRingContent {
    #[serde(rename = "posList")]
    PosList(GmlDirectPositionList),

    #[serde(rename = "pos")]
    Pos(Vec<GmlDirectPosition>),
}

impl TryFrom<GmlLinearRingContent> for Vec<DirectPosition> {
    type Error = Error;

    fn try_from(value: GmlLinearRingContent) -> Result<Self, Self::Error> {
        match value {
            GmlLinearRingContent::PosList(x) => x.try_into(),
            GmlLinearRingContent::Pos(x) => {
                let points: Vec<DirectPosition> = x
                    .into_iter()
                    .map(|p| p.try_into())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(points)
            }
        }
    }
}

impl TryFrom<GmlLinearRing> for LinearRing {
    type Error = Error;

    fn try_from(value: GmlLinearRing) -> Result<Self, Self::Error> {
        let id = value.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_ring = AbstractRing::default();
        abstract_ring.set_id(id);

        let mut points: Vec<DirectPosition> = value
            .content
            .ok_or(Error::ElementNotFound("No element found".to_string()))?
            .try_into()?;

        points.dedup();
        if points.first().unwrap() == points.last().unwrap() {
            points.pop();
        }

        let linear_ring = LinearRing::new(abstract_ring, points)?;
        Ok(linear_ring)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::primitives::GmlLinearRing;
    use egml_core::model::geometry::primitives::LinearRing;
    use quick_xml::de;

    #[test]
    fn parsing_linear_ring() {
        let xml_document = b"<gml:LinearRing gml:id=\"4018115_LR.d2yyBHNssydL8g3B8MvW\">
<gml:posList srsDimension=\"3\">
678058.447 5403817.567 424.209
678058.275 5403817.484 424.209
678058.689 5403816.628 424.209
678058.871 5403816.718 424.209
678058.447 5403817.567 424.209
</gml:posList>
</gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let l: LinearRing = parsed_geometry.try_into().unwrap();
    }

    #[test]
    fn parsing_basic_linear_ring() {
        let xml_document = b"<gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_ok());
        let linear_ring = result.unwrap();
        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn parsing_linear_ring_with_duplicates() {
        let xml_document = b"<gml:LinearRing gml:id=\"DEBY_LOD2_4959457_LR.EEAbfUPItTlOGZGH7VDv\">
                      <gml:posList>691040.851 5336002.449 529.908 691040.741 5336002.172 529.908 691040.851 5336002.449 529.908 691040.851 5336002.449 529.908</gml:posList>
                    </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn parsing_linear_ring_with_pos_list() {
        let xml_document = b"<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                      <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                    </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_ok());
        let linear_ring = result.unwrap();
        assert_eq!(linear_ring.points().len(), 5);
    }
}
