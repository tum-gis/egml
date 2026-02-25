use crate::Error;
use crate::primitives::GmlSurfaceKind;
use egml_core::model::geometry::primitives::{SurfaceKind, SurfaceProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfaceProperty {
    #[serde(rename = "@href")]
    pub href: Option<String>,

    #[serde(rename = "$value")]
    pub content: Option<GmlSurfaceKind>,
}

impl TryFrom<GmlSurfaceProperty> for SurfaceProperty {
    type Error = Error;

    fn try_from(item: GmlSurfaceProperty) -> Result<Self, Self::Error> {
        if item.href.is_some() && item.content.is_none() {
            return Err(Error::XLinksNotSupported());
        }

        let surface: SurfaceKind = item
            .content
            .ok_or(Error::MissingSurfaceKind(
                "parsing the GmlSurfaceProperty".to_string(),
            ))?
            .try_into()?;

        let surface_property = SurfaceProperty::new(surface)?;
        Ok(surface_property)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlSurfaceProperty;
    use quick_xml::de;

    #[test]
    fn parsing_surface_property_with_xlink() {
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
}
