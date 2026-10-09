use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_patch_kind_for, serialize_abstract_surface_patch_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::geometry::primitives::AbstractSurfacePatchKind;
use std::io::Write;

/// Deserializes a `gml:SurfacePatchArrayPropertyType` element (`gml:patches`)
/// into its patches, in document order.
///
/// The property type carries no attributes.
pub fn deserialize_abstract_surface_patch_array_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Vec<AbstractSurfacePatchKind>, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let mut all_children: Vec<(GmlElement, &XmlDocumentIndex<GmlElement>)> = index
        .children()
        .iter()
        .flat_map(|(elem, nodes)| nodes.iter().map(move |node| (*elem, node)))
        .collect();
    all_children.sort_by_key(|(_, node)| node.range().start);

    all_children
        .iter()
        .filter_map(|(elem, node)| {
            let slice = &xml_document[node.range()];
            deserialize_abstract_surface_patch_kind_for(*elem, slice, node, config).transpose()
        })
        .collect()
}

/// Serializes `patches` wrapped in a `gml:SurfacePatchArrayPropertyType` element.
pub fn serialize_abstract_surface_patch_array_property<N: XmlNamespace, E: XmlElement, W: Write>(
    patches: &[AbstractSurfacePatchKind],
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(target_xml_namespace, target_xml_element)?;
    for patch in patches {
        serialize_abstract_surface_patch_kind(patch, xml_fragment_writer)?;
    }
    xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{
        deserialize_abstract_surface_patch_array_property,
        serialize_abstract_surface_patch_array_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};

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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let patches = deserialize_abstract_surface_patch_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert_eq!(patches.len(), 2);
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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let patches = deserialize_abstract_surface_patch_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert_eq!(patches.len(), 3);
    }

    #[test]
    fn serialize_surface_patch_array_property_with_triangles() {
        let xml_document = b"<gml:patches>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_abstract_surface_patch_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_surface_patch_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PatchesProperty,
        )
        .expect("serialize should work");
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert!(output.contains("<gml:patches"));
        assert!(output.contains("<gml:Triangle"));
        assert_eq!(
            output.matches("<gml:Triangle").count(),
            2,
            "should have 2 triangles"
        );
    }

    #[test]
    fn deserialize_ignores_xlink_and_ownership_attributes() {
        // gml:SurfacePatchArrayPropertyType declares neither attribute group; tolerate them in input.
        let xml_document = b"<gml:patches xlink:href=\"#some-id\" gml:owns=\"true\">\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let patches = deserialize_abstract_surface_patch_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();
        assert_eq!(patches.len(), 1);

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_surface_patch_array_property(
            &patches,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PatchesProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();
        assert!(output.starts_with("<gml:patches><gml:Triangle"));
    }
}
