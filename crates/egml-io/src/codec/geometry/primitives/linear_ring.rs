use crate::codec::geometry::GmlDirectPosition;
use crate::codec::geometry::direct_position_list::GmlDirectPositionList;
use crate::codec::geometry::primitives::abstract_ring::{
    deserialize_abstract_ring, serialize_abstract_ring,
};
use crate::error::Error;
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, dedup_adjacent_positions,
    extract_xml_element_spans, serialize_inner,
};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AsAbstractRing, LinearRing};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_linear_ring(xml_document: &[u8]) -> Result<LinearRing, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_ring = deserialize_abstract_ring(xml_document, &spans)?;

    let parsed: GmlLinearRing = de::from_reader(xml_document)?;
    let mut points: Vec<DirectPosition> = parsed.content.unwrap().try_into()?;
    dedup_adjacent_positions(&mut points, "LinearRing");
    if points.first().unwrap() == points.last().unwrap() {
        points.pop();
    }

    let linear_ring = LinearRing::from_abstract_ring(abstract_ring, points)?;
    Ok(linear_ring)
}

pub fn serialize_linear_ring(
    linear_ring: &LinearRing,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_ring(linear_ring.abstract_ring(), formatting)?;

    if let Some(raw) = serialize_inner(GmlLinearRing::from(linear_ring), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(GmlElement::LinearRing.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRing {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<GmlLinearRingContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlLinearRingContent {
    #[serde(rename(serialize = "gml:posList", deserialize = "posList"))]
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
        let points: Vec<DirectPosition> = value
            .content
            .ok_or(Error::ElementNotFound("No element found".to_string()))?
            .try_into()?;

        let linear_ring = LinearRing::new(points)?;
        Ok(linear_ring)
    }
}

impl From<&LinearRing> for GmlLinearRing {
    fn from(ring: &LinearRing) -> Self {
        // GML requires the ring to be closed: the closing vertex (= first point) must
        // be written explicitly, but LinearRing stores points in open form (no repeat).
        let mut points: Vec<DirectPosition> = ring.points().to_vec();
        if let Some(&first) = ring.points().first() {
            points.push(first);
        }

        Self {
            content: Some(GmlLinearRingContent::PosList(GmlDirectPositionList::from(
                points.as_slice(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlLinearRingContent;
    use crate::codec::geometry::primitives::{
        GmlLinearRing, deserialize_linear_ring, serialize_linear_ring,
    };
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::LinearRing;
    use quick_xml::de;

    fn make_triangle() -> LinearRing {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        LinearRing::new(points).unwrap()
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list_and_id() {
        let xml_document = b"<gml:LinearRing gml:id=\"4018115_LR.d2yyBHNssydL8g3B8MvW\">
<gml:posList srsDimension=\"3\">
678058.447 5403817.567 424.209
678058.275 5403817.484 424.209
678058.689 5403816.628 424.209
678058.871 5403816.718 424.209
678058.447 5403817.567 424.209
</gml:posList>
</gml:LinearRing>";

        let linear_ring: LinearRing =
            deserialize_linear_ring(xml_document).expect("should deserialize");
        use egml_core::model::base::AsAbstractGml;
        assert!(linear_ring.id().is_some());
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list() {
        let xml_document = b"<gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>";

        let linear_ring_result = deserialize_linear_ring(xml_document);

        assert!(linear_ring_result.is_ok());
        let linear_ring = linear_ring_result.unwrap();
        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_linear_ring_repairs_adjacent_duplicate_points() {
        let xml_document = b"<gml:LinearRing>
                      <gml:posList>0 0 0 1 0 0 1 0 0 1 1 0 0 0 0</gml:posList>
                    </gml:LinearRing>";

        let linear_ring = deserialize_linear_ring(xml_document).expect("should deserialize");

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_linear_ring_with_duplicate_points_returns_error() {
        let xml_document = b"<gml:LinearRing gml:id=\"DEBY_LOD2_4959457_LR.EEAbfUPItTlOGZGH7VDv\">
                      <gml:posList>691040.851 5336002.449 529.908 691040.741 5336002.172 529.908 691040.851 5336002.449 529.908 691040.851 5336002.449 529.908</gml:posList>
                    </gml:LinearRing>";

        let linear_ring_result = deserialize_linear_ring(xml_document);

        assert!(linear_ring_result.is_err());
    }

    #[test]
    fn serialize_linear_ring_writes_gml_tags() {
        let linear_ring = make_triangle();
        let xml_node = serialize_linear_ring(&linear_ring, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_linear_ring_appends_closing_vertex() {
        let ring = make_triangle(); // 3 open points

        // Verify via the intermediate GmlLinearRing that the closing vertex is present
        let gml = GmlLinearRing::from(&ring);
        let positions: Vec<DirectPosition> = match gml.content.unwrap() {
            GmlLinearRingContent::PosList(pos_list) => pos_list.try_into().unwrap(),
            GmlLinearRingContent::Pos(_) => panic!("expected PosList"),
        };
        assert_eq!(positions.len(), 4); // 3 open points + closing vertex
        assert_eq!(positions.first(), positions.last()); // closing vertex equals first point
    }

    #[test]
    fn serialize_linear_ring_with_id() {
        use egml_core::model::base::AsAbstractGmlMut;
        let points = vec![
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 0.0, 1.0).unwrap(),
        ];
        let mut linear_ring = LinearRing::new(points).unwrap();
        linear_ring.set_id(Id::from_hashed_string("test-ring"));

        let xml_node = serialize_linear_ring(&linear_ring, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");
        assert!(xml.contains("id="));
    }

    #[test]
    fn round_trip_linear_ring_preserves_points() {
        let linear_ring = make_triangle();
        let xml_node = serialize_linear_ring(&linear_ring, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        let recovered_linear_ring: LinearRing =
            deserialize_linear_ring(xml.as_ref()).expect("should deserialize");

        assert_eq!(
            recovered_linear_ring.points().len(),
            linear_ring.points().len()
        );
        for (a, b) in recovered_linear_ring
            .points()
            .iter()
            .zip(linear_ring.points().iter())
        {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_linear_ring_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let linear_ring = LinearRing::new(points.clone()).unwrap();

        let xml_node = serialize_linear_ring(&linear_ring, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        let recovered_linear_ring: LinearRing =
            deserialize_linear_ring(xml.as_ref()).expect("should deserialize");

        for (a, b) in recovered_linear_ring.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_linear_ring_from_xml() {
        let xml_document = "<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing>";

        let linear_ring =
            deserialize_linear_ring(xml_document.as_ref()).expect("should deserialize");

        let xml_node = serialize_linear_ring(&linear_ring, Formatting::Compact).unwrap();
        let output_xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert_eq!(xml_document, output_xml);
    }

    #[test]
    fn deserialize_linear_ring_with_pos_elements() {
        let xml_document = b"<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                      <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                    </gml:LinearRing>";

        let linear_ring_result = deserialize_linear_ring(xml_document);

        assert!(linear_ring_result.is_ok());
        let linear_ring = linear_ring_result.unwrap();
        assert_eq!(linear_ring.points().len(), 5);
    }
}
