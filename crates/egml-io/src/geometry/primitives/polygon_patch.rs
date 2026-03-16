use crate::Error;
use crate::primitives::abstract_ring_property::GmlRingProperty;
use egml_core::model::geometry::primitives::{PolygonPatch, RingPropertyKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPolygonPatch {
    #[serde(rename(serialize = "gml:exterior", deserialize = "exterior"))]
    pub exterior: Option<GmlRingProperty>,

    #[serde(
        rename(serialize = "gml:interior", deserialize = "interior"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
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

impl From<&PolygonPatch> for GmlPolygonPatch {
    fn from(patch: &PolygonPatch) -> Self {
        let exterior = patch.exterior().map(ring_property_kind_to_gml);
        let interior = patch
            .interior()
            .iter()
            .map(ring_property_kind_to_gml)
            .collect();
        Self { exterior, interior }
    }
}

fn ring_property_kind_to_gml(kind: &RingPropertyKind) -> GmlRingProperty {
    match kind {
        RingPropertyKind::LinearRing(lr) => GmlRingProperty::from(lr),
        RingPropertyKind::RingKind(_) => todo!("Ring serialization is not yet implemented"),
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlPolygonPatch;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRing, AbstractSurfacePatch, LinearRing, PolygonPatch, RingPropertyKind,
    };
    use quick_xml::{DeError, de, se};

    #[test]
    fn deserialize_polygon_patch() {
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
    fn deserialize_polygon_patch_with_interior_rings() {
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

    fn make_polygon_patch() -> PolygonPatch {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = LinearRing::new(AbstractRing::default(), points).unwrap();
        PolygonPatch::new(
            AbstractSurfacePatch::default(),
            Some(RingPropertyKind::LinearRing(ring)),
            vec![],
        )
    }

    #[test]
    fn serialize_polygon_patch_writes_gml_tags() {
        let patch = make_polygon_patch();
        let gml = GmlPolygonPatch::from(&patch);
        let xml = se::to_string_with_root("gml:PolygonPatch", &gml).unwrap();

        assert!(xml.contains("<gml:PolygonPatch"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_polygon_patch_from_xml() {
        let input_xml = "<gml:PolygonPatch>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:PolygonPatch>";

        let gml: GmlPolygonPatch = de::from_reader(input_xml.as_bytes()).unwrap();
        let patch: PolygonPatch = gml.try_into().unwrap();
        let output_xml =
            se::to_string_with_root("gml:PolygonPatch", &GmlPolygonPatch::from(&patch)).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
