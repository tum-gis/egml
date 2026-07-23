use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_patch_kind, serialize_abstract_surface_patch_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractSurfacePatchArrayProperty;
use egml_core::model::geometry::primitives::AbstractSurfacePatchKind;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_surface_patch_array_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractSurfacePatchArrayProperty>, Error> {
    let parsed: GmlAbstractSurfacePatchArrayProperty = de::from_reader(xml_document)?;

    let mut all_spans: Vec<(GmlElement, std::ops::Range<usize>)> = spans
        .spans()
        .iter()
        .flat_map(|(elem, ranges)| ranges.iter().map(|r| (*elem, r.clone())))
        .collect();
    all_spans.sort_by_key(|(_, r)| r.start);

    let patches: Vec<AbstractSurfacePatchKind> = all_spans
        .iter()
        .filter_map(|(elem, span)| {
            let slice = &xml_document[span.start..span.end];
            let parent_spans = XmlElementSpans::single(*elem, slice.len());
            deserialize_abstract_surface_patch_kind(slice, &parent_spans).transpose()
        })
        .collect::<Result<_, _>>()?;

    if patches.is_empty() {
        return Ok(None);
    }
    Ok(Some(AbstractSurfacePatchArrayProperty::new(
        patches,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    )))
}

pub fn serialize_abstract_surface_patch_array_property(
    abstract_surface_patch_array_property: &AbstractSurfacePatchArrayProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_surface_patch_array_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_surface_patch_array_property.ownership(),
    ));

    for patch in abstract_surface_patch_array_property.objects() {
        parts.content.push(XmlNodeContent::Child(
            serialize_abstract_surface_patch_kind(patch, formatting)?,
        ));
    }

    Ok(XmlNode::new(target_xml_element, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSurfacePatchArrayProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{
        deserialize_abstract_surface_patch_array_property,
        serialize_abstract_surface_patch_array_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::primitives::AbstractSurfacePatchArrayProperty;
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn deserialize_surface_patch_array_property_with_polygon_patches() {
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

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let surface_patch_array_property: AbstractSurfacePatchArrayProperty =
            deserialize_abstract_surface_patch_array_property(xml_document, &spans)
                .expect("should deserialize")
                .expect("should be some");

        assert_eq!(surface_patch_array_property.objects().len(), 2);
    }

    #[test]
    fn deserialize_surface_patch_array_property_with_triangles() {
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

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let surface_patch_array_property: AbstractSurfacePatchArrayProperty =
            deserialize_abstract_surface_patch_array_property(xml_document, &spans)
                .expect("should deserialize")
                .expect("should be some");

        assert_eq!(surface_patch_array_property.objects().len(), 3);
    }

    #[test]
    fn serialize_surface_patch_array_property_with_triangles() {
        let xml_document = b"<gml:patches>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_abstract_surface_patch_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_abstract_surface_patch_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PatchesProperty.into(),
        )
        .expect("serialize should work");
        let output = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(output.contains("<gml:patches"));
        assert!(output.contains("<gml:Triangle"));
        assert_eq!(
            output.matches("<gml:Triangle").count(),
            2,
            "should have 2 triangles"
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:patches xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_surface_patch_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert_eq!(property.title().as_deref(), Some("Some Title"));
        assert_eq!(property.role().as_deref(), Some("http://example.com/role"));
        assert_eq!(
            property.arcrole().as_deref(),
            Some("http://example.com/arcrole")
        );
        assert_eq!(property.show(), Some(&ShowType::New));
        assert_eq!(property.actuate(), Some(&ActuateType::OnLoad));
        assert!(property.owns());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:patches xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_surface_patch_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_abstract_surface_patch_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PatchesProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered =
            deserialize_abstract_surface_patch_array_property(output.as_bytes(), &spans2)
                .unwrap()
                .unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
