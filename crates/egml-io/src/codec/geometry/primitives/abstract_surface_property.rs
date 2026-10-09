use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_kind, serialize_abstract_surface_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractSurfaceProperty;
use std::io::Write;

pub fn deserialize_abstract_surface_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractSurfaceProperty, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;
    let object = deserialize_abstract_surface_kind(xml_document, index, config)?;

    Ok(AbstractSurfaceProperty::new(object, association, ownership))
}

pub fn serialize_abstract_surface_property<N: XmlNamespace, E: XmlElement, W: Write>(
    abstract_surface_property: &AbstractSurfaceProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        abstract_surface_property.association(),
        abstract_surface_property.ownership(),
    );

    match abstract_surface_property.object() {
        Some(abstract_surface_kind) => {
            xml_fragment_writer.write_start_event_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
            serialize_abstract_surface_kind(abstract_surface_kind, xml_fragment_writer)?;
            xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
        }
        None => {
            xml_fragment_writer.write_empty_element_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::abstract_surface_property::{
        deserialize_abstract_surface_property, serialize_abstract_surface_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::HasAssociationAttributes;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfaceKind, AbstractSurfaceProperty, LinearRing, Polygon,
    };
    use egml_core::model::xlink::HRef;

    fn make_surface_property() -> AbstractSurfaceProperty {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(Some(AbstractRingKind::LinearRing(ring)), vec![]).unwrap();
        AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon))
    }

    #[test]
    fn deserialize_abstract_surface_property_with_polygon() {
        let xml_document = b"<gml:surfaceMember>
    <gml:Polygon>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Polygon>
</gml:surfaceMember>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert!(matches!(
            property.object(),
            Some(AbstractSurfaceKind::Polygon(_))
        ));
    }

    #[test]
    fn deserialize_abstract_surface_property_with_xlink() {
        let xml_document = b"<gml:surfaceMember xlink:href=\"#some-surface-id\"/>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-surface-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_abstract_surface_property_writes_gml_tags() {
        let property = make_surface_property();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_surface_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::SurfaceMemberProperty,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_abstract_surface_property_preserves_polygon() {
        let xml_document = b"<gml:surfaceMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:surfaceMember>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_surface_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::SurfaceMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_surface_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert!(matches!(
            recovered.object(),
            Some(AbstractSurfaceKind::Polygon(_))
        ));
    }

    #[test]
    fn serialize_href_only_property_writes_self_closed_tag() {
        let property = AbstractSurfaceProperty::from_href(HRef::from_local("some-id"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_surface_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::SurfaceMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(xml, r##"<gml:surfaceMember xlink:href="#some-id"/>"##);
    }
}
