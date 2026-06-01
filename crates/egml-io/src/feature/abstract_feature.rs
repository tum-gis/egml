use crate::feature::bounding_shape::GmlBoundingShape;
use crate::{Error, GmlAbstractGml, deserialize_abstract_gml};
use egml_core::model::feature::AbstractFeature;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_feature(xml_document: &[u8]) -> Result<AbstractFeature, Error> {
    let abstract_gml = deserialize_abstract_gml(xml_document)?;

    let parsed_feature: GmlAbstractFeature = de::from_reader(xml_document)?;

    let mut abstract_feature = AbstractFeature::new(abstract_gml);
    abstract_feature.bounded_by = parsed_feature
        .bounded_by
        .map(|x| x.try_into())
        .transpose()?;

    Ok(abstract_feature)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractFeature {
    #[serde(flatten, skip_deserializing)]
    pub abstract_gml: GmlAbstractGml,

    #[serde(
        rename(serialize = "gml:boundedBy", deserialize = "boundedBy"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bounded_by: Option<GmlBoundingShape>,
}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(item: &AbstractFeature) -> Self {
        Self {
            abstract_gml: (&item.abstract_gml).into(),
            bounded_by: item.bounded_by.as_ref().map(|x| x.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::abstract_feature::deserialize_abstract_feature;
    use egml_core::model::feature::AbstractFeature;
    use egml_core::model::geometry::Envelope;

    #[test]
    fn deserialize_simple_abstract_feature() {
        let xml_document = "<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
    <gml:boundedBy>
        <gml:Envelope srsDimension=\"3\" srsName=\"urn:ogc:def:crs:EPSG::25832\">
            <gml:lowerCorner>1.0 2.0 3</gml:lowerCorner>
            <gml:upperCorner>11.0 12.0 13.0</gml:upperCorner>
        </gml:Envelope>
    </gml:boundedBy>
</ExampleFeature>";

        let abstract_feature: AbstractFeature =
            deserialize_abstract_feature(xml_document.as_ref()).unwrap();

        let envelope: Envelope = abstract_feature.bounded_by.unwrap().envelope.unwrap();
        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }
}
