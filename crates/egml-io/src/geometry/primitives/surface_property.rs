use crate::Error;
use crate::primitives::GmlSurfaceKind;
use egml_core::model::geometry::primitives::SurfaceProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub object: Option<GmlSurfaceKind>,
}

impl TryFrom<GmlSurfaceProperty> for SurfaceProperty {
    type Error = Error;

    fn try_from(item: GmlSurfaceProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            object: item.object.map(|x| x.try_into()).transpose()?,
            href: item.href,
        })
    }
}

impl From<&SurfaceProperty> for GmlSurfaceProperty {
    fn from(item: &SurfaceProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlSurfaceProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        LinearRing, Polygon, RingProperty, SurfaceProperty,
    };
    use egml_core::model::geometry::primitives::{RingKind, SurfaceKind};
    use quick_xml::{de, se};

    fn make_surface_property() -> SurfaceProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring_kind = RingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(RingProperty::new(ring_kind)), vec![]).unwrap();
        SurfaceProperty::new(SurfaceKind::Polygon(polygon))
    }

    #[test]
    fn deserialize_surface_property_with_xlink() {
        let xml_document = b"\
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2_poly\"/>";

        let parsed_geometry: GmlSurfaceProperty = de::from_reader(xml_document.as_ref()).unwrap();

        assert_eq!(
            parsed_geometry.href,
            Some("#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2_poly".to_string())
        );
    }

    #[test]
    fn serialize_surface_property_writes_polygon_element() {
        let prop = make_surface_property();
        let gml = GmlSurfaceProperty::from(&prop);
        let xml = se::to_string_with_root("gml:surfaceMember", &gml).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn round_trip_surface_property_from_xml() {
        let input_xml = "<gml:surfaceMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:surfaceMember>";

        let gml: GmlSurfaceProperty = de::from_reader(input_xml.as_bytes()).unwrap();
        let output_xml = se::to_string_with_root("gml:surfaceMember", &gml).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
