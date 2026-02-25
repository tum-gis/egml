use crate::Error;
use crate::primitives::GmlSurfacePatchKind;
use egml_core::model::geometry::primitives::{SurfacePatchArrayProperty, SurfacePatchKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurfacePatchArrayProperty {
    #[serde(rename = "$value")]
    pub patches: Vec<GmlSurfacePatchKind>,
}

impl TryFrom<GmlSurfacePatchArrayProperty> for SurfacePatchArrayProperty {
    type Error = Error;

    fn try_from(item: GmlSurfacePatchArrayProperty) -> Result<Self, Self::Error> {
        let patches: Vec<SurfacePatchKind> = item
            .patches
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(patches))
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::surface_patch_array_property::GmlSurfacePatchArrayProperty;
    use egml_core::model::geometry::primitives::{
        LinearRing, PolygonPatch, SurfacePatchArrayProperty,
    };
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_surface_patch_array_property() {
        let xml_document = b"<gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>";

        let parsed_result: Result<GmlSurfacePatchArrayProperty, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let surface_patch_property_array: SurfacePatchArrayProperty =
            parsed_gml.try_into().unwrap();

        assert_eq!(surface_patch_property_array.patches().len(), 2);
    }

    #[test]
    fn parsing_surface_patch_array_property_with_triangles() {
        let xml_document = b"<gml:patches>
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 355.39898681640625 978.8480224609375 2.388849973678589 355.3919982910156 978.8480224609375 2.1084799766540527 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 355.3919982910156 978.8480224609375 2.1084799766540527 354.01800537109375 978.864990234375 2.1084799766540527 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 354.01800537109375 978.864990234375 2.1084799766540527 353.9599914550781 978.8660278320312 3.0346200466156006 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>
</gml:patches>";

        let parsed_result: Result<GmlSurfacePatchArrayProperty, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let surface_patch_property_array: SurfacePatchArrayProperty =
            parsed_gml.try_into().unwrap();

        assert_eq!(surface_patch_property_array.patches().len(), 3);
    }
}
