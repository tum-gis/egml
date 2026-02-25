use crate::Error;
use crate::primitives::abstract_ring_property::GmlRingProperty;
use egml_core::model::geometry::primitives::{LinearRing, RingPropertyKind, Triangle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangle {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "exterior")]
    pub exterior: GmlRingProperty,
}

impl TryFrom<GmlTriangle> for Triangle {
    type Error = Error;

    fn try_from(item: GmlTriangle) -> Result<Triangle, Self::Error> {
        let exterior: RingPropertyKind = item.exterior.try_into()?;
        let exterior: LinearRing = match exterior {
            RingPropertyKind::LinearRing(x) => x,
            _ => todo!("needs to be implemented"),
        };

        if exterior.points().len() != 3 {
            return Err(Error::MissingElements(
                "triangle must have exactly 3 points".to_string(),
            ));
        }

        let a = *exterior.points().first().unwrap();
        let b = *exterior.points().get(1).unwrap();
        let c = *exterior.points().get(2).unwrap();

        Ok(Triangle::new(a, b, c)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::surface_patch_array_property::GmlSurfacePatchArrayProperty;
    use crate::primitives::triangle::GmlTriangle;
    use egml_core::model::geometry::primitives::{SurfacePatchArrayProperty, Triangle};
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_surface_patch_array_property_with_triangles() {
        let xml_document = b"
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 355.39898681640625 978.8480224609375 2.388849973678589 355.3919982910156 978.8480224609375 2.1084799766540527 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>";

        let parsed_result: Result<GmlTriangle, DeError> = de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let triangle: Triangle = parsed_gml.try_into().unwrap();

        assert_eq!(triangle.a.x(), 354.0249938964844);
        assert_eq!(triangle.a.y(), 978.864990234375);
        assert_eq!(triangle.a.z(), 2.388849973678589);
    }
}
