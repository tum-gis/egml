use crate::Error;
use crate::primitives::GmlSurfaceKind;
use egml_core::model::geometry::primitives::{SurfaceKind, SurfaceProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfaceProperty {
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<GmlSurfaceKind>,
}

impl TryFrom<GmlSurfaceProperty> for SurfaceProperty {
    type Error = Error;

    fn try_from(item: GmlSurfaceProperty) -> Result<Self, Self::Error> {
        if item.href.is_some() && item.content.is_none() {
            return Err(Error::UnsupportedXLink());
        }

        let surface: SurfaceKind = item
            .content
            .ok_or(Error::MissingSurfaceKind(
                "parsing the GmlSurfaceProperty".to_string(),
            ))?
            .try_into()?;

        Ok(SurfaceProperty::new(surface))
    }
}

impl From<&SurfaceProperty> for GmlSurfaceProperty {
    fn from(prop: &SurfaceProperty) -> Self {
        Self {
            href: None,
            content: Some(GmlSurfaceKind::from(&prop.content)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlSurfaceProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRing, AbstractSurface, LinearRing, Polygon, RingPropertyKind, SurfaceKind,
        SurfaceProperty,
    };
    use quick_xml::{de, se};

    fn make_surface_property() -> SurfaceProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = LinearRing::new(AbstractRing::default(), points).unwrap();
        let polygon = Polygon::new(
            AbstractSurface::default(),
            Some(RingPropertyKind::LinearRing(ring)),
            vec![],
        )
        .unwrap();
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
