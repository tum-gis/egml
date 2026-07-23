use crate::Error;
use crate::codec::base::{deserialize_abstract_gml, serialize_abstract_gml};
use crate::codec::feature::bounding_shape::GmlBoundingShape;
use crate::util::{
    Formatting, GmlElement, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner,
};
use egml_core::model::base::AsAbstractGml;
use egml_core::model::feature::{AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_feature(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractFeature, Error> {
    let abstract_gml = deserialize_abstract_gml(xml_document, spans)?;
    let mut abstract_feature = AbstractFeature::from_abstract_gml(abstract_gml);

    let parsed: GmlAbstractFeature = de::from_reader(xml_document)?;
    abstract_feature.set_bounded_by(parsed.bounded_by.map(|x| x.try_into()).transpose()?);

    Ok(abstract_feature)
}

pub fn serialize_abstract_feature(
    abstract_feature: &AbstractFeature,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_gml(abstract_feature.abstract_gml(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAbstractFeature::from(abstract_feature), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractFeature {
    #[serde(
        rename(serialize = "gml:boundedBy", deserialize = "boundedBy"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bounded_by: Option<GmlBoundingShape>,
}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(item: &AbstractFeature) -> Self {
        Self {
            bounded_by: item.bounded_by().map(|x| x.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::feature::abstract_feature::deserialize_abstract_feature;
    use crate::util::{GmlElement, XmlElementSpans, extract_xml_element_spans};
    use egml_core::model::feature::{AbstractFeature, AsAbstractFeature};
    use egml_core::model::geometry::Envelope;

    #[test]
    fn deserialize_simple_abstract_feature() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
    <gml:boundedBy>
        <gml:Envelope srsDimension=\"3\" srsName=\"urn:ogc:def:crs:EPSG::25832\">
            <gml:lowerCorner>1.0 2.0 3</gml:lowerCorner>
            <gml:upperCorner>11.0 12.0 13.0</gml:upperCorner>
        </gml:Envelope>
    </gml:boundedBy>
</ExampleFeature>";

        let spans: XmlElementSpans<GmlElement> =
            extract_xml_element_spans(xml_document).expect("should work");
        let abstract_feature: AbstractFeature =
            deserialize_abstract_feature(xml_document.as_ref(), &spans).unwrap();

        let envelope: Envelope = abstract_feature
            .bounded_by()
            .unwrap()
            .envelope()
            .unwrap()
            .clone();
        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }
}
