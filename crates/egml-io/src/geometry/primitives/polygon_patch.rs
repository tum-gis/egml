use crate::Error;
use crate::primitives::abstract_ring_property::GmlRingProperty;
use egml_core::model::geometry::primitives::{PolygonPatch, RingPropertyKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPolygonPatch {
    #[serde(rename = "exterior")]
    pub exterior: Option<GmlRingProperty>,

    #[serde(rename = "interior", default)]
    pub interior: Vec<GmlRingProperty>,
}

impl TryFrom<GmlPolygonPatch> for PolygonPatch {
    type Error = Error;

    fn try_from(value: GmlPolygonPatch) -> Result<Self, Self::Error> {
        let exterior: Option<RingPropertyKind> =
            value.exterior.map(|x| x.try_into()).transpose()?;

        let interior: Vec<RingPropertyKind> = value
            .interior
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PolygonPatch::new(Default::default(), exterior, interior))
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{GmlPolygon, GmlPolygonPatch};
    use egml_core::model::geometry::primitives::{
        LinearRing, Polygon, PolygonPatch, RingKind, RingPropertyKind,
    };
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_polygon_patch() {
        let xml_document = b"<gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>";

        let parsed_result: Result<GmlPolygonPatch, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let polygon_patch: PolygonPatch = parsed_gml.try_into().unwrap();

        let exterior: &RingPropertyKind = polygon_patch.exterior().expect("should be set");
        match exterior {
            RingPropertyKind::LinearRing(x) => {
                assert_eq!(x.points().len(), 3);
            }
            _ => panic!("should be linear ring"),
        }
    }

    #[test]
    fn parsing_polygon_patch_with_interior() {
        let xml_document = b"<gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                    <gml:interior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:interior>
                    <gml:interior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:interior>
                </gml:PolygonPatch>";

        let parsed_result: Result<GmlPolygonPatch, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let polygon_patch: PolygonPatch = parsed_gml.try_into().unwrap();

        assert_eq!(polygon_patch.interior().len(), 2);

        let exterior: &RingPropertyKind = polygon_patch.exterior().expect("should be set");
        match exterior {
            RingPropertyKind::LinearRing(x) => {
                assert_eq!(x.points().len(), 3);
            }
            _ => panic!("should be linear ring"),
        }
    }
}
