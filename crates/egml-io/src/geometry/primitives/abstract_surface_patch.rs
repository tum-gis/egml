use crate::Error;
use crate::primitives::GmlPolygonPatch;
use crate::primitives::triangle::GmlTriangle;
use egml_core::model::geometry::primitives::SurfacePatchKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlSurfacePatchKind {
    // LinearRingProperty(GmlLinearRingProperty),
    PolygonPatch(GmlPolygonPatch),
    Triangle(GmlTriangle),
}

impl TryFrom<GmlSurfacePatchKind> for SurfacePatchKind {
    type Error = Error;

    fn try_from(item: GmlSurfacePatchKind) -> Result<Self, Self::Error> {
        let surface_patch_kind = match item {
            GmlSurfacePatchKind::PolygonPatch(x) => SurfacePatchKind::PolygonPatch(x.try_into()?),
            GmlSurfacePatchKind::Triangle(x) => SurfacePatchKind::Triangle(x.try_into()?),
        };
        Ok(surface_patch_kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregates::parse_multi_surface;
    use crate::primitives::GmlSurfacePatchKind;
    use crate::primitives::{GmlPolygonPatch, GmlSurfacePatchArrayProperty};
    use egml_core::model::geometry::primitives::{SurfacePatchArrayProperty, SurfacePatchKind};
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_multi_surface_with_patches() {
        let xml_document = b"
            <gml:PolygonPatch>
                <gml:exterior>
                    <gml:LinearRing>
                        <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                    </gml:LinearRing>
                </gml:exterior>
            </gml:PolygonPatch>";

        let parsed_result: Result<GmlSurfacePatchKind, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let surface_patch_kind: SurfacePatchKind = parsed_gml.try_into().unwrap();

        if let SurfacePatchKind::PolygonPatch(x) = surface_patch_kind {
            assert!(x.exterior().is_some());
        } else {
            panic!("should be polygon patch");
        }
    }
}
