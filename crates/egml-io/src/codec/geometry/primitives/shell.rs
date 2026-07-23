use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface, deserialize_abstract_surface_property,
    serialize_abstract_surface, serialize_abstract_surface_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_children_lenient,
    extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::{AsAbstractSurface, Shell};
use tracing::debug;

pub fn deserialize_shell(xml_document: &[u8]) -> Result<Shell, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface = deserialize_abstract_surface(xml_document, &spans)?;

    let (members, skipped) = collect_children_lenient(
        xml_document,
        &spans,
        GmlElement::SurfaceMemberProperty,
        deserialize_abstract_surface_property,
    );
    if !skipped.is_empty() {
        debug!(
            count = skipped.len(),
            "Shell: dropped invalid surfaceMember(s)"
        );
    }

    Ok(Shell::from_abstract_surface(abstract_surface, members)?)
}

pub fn serialize_shell(shell: &Shell, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_surface(shell.abstract_surface(), formatting)?;

    for member in shell.members() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_surface_property(
                member,
                formatting,
                GmlElement::SurfaceMemberProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::Shell.into(), parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::shell::{deserialize_shell, serialize_shell};
    use crate::util::Formatting;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon, Shell,
    };

    fn make_shell() -> Shell {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            [],
        )
        .unwrap();
        let member = AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        Shell::new([member]).unwrap()
    }

    #[test]
    fn deserialize_shell_test() {
        let xml_document = b"<gml:Shell>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:Shell>";

        let shell = deserialize_shell(xml_document).expect("should deserialize");

        assert_eq!(shell.members().len(), 2);
    }

    #[test]
    fn deserialize_shell_skips_invalid_polygon_member() {
        let xml_document = b"<gml:Shell>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>0 0 0 1 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:Shell>";

        let shell = deserialize_shell(xml_document).expect("should deserialize");

        assert_eq!(shell.members().len(), 1);
    }

    #[test]
    fn serialize_shell_writes_gml_tags() {
        let shell = make_shell();

        let xml_node = serialize_shell(&shell, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn round_trip_shell_preserves_member_count() {
        let shell = make_shell();

        let xml_node = serialize_shell(&shell, Formatting::Compact).expect("should serialize");
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_shell(xml.as_bytes()).expect("should deserialize");

        assert_eq!(recovered.members().len(), shell.members().len());
    }
}
