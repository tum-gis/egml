use crate::Error;
use crate::primitives::GmlPoint;
use egml_core::model::geometry::primitives::PointArrayProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointArrayProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(serialize = "gml:Point", deserialize = "Point"),
        skip_serializing_if = "Vec::is_empty"
    )]
    pub objects: Vec<GmlPoint>,
}

impl TryFrom<GmlPointArrayProperty> for PointArrayProperty {
    type Error = Error;

    fn try_from(item: GmlPointArrayProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            objects: item
                .objects
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            href: item.href,
        })
    }
}

impl From<&PointArrayProperty> for GmlPointArrayProperty {
    fn from(item: &PointArrayProperty) -> Self {
        Self {
            href: item.href.clone(),
            objects: item.objects.iter().map(|x| x.into()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::point_array_property::GmlPointArrayProperty;
    use egml_core::model::geometry::primitives::{PointArrayProperty, SurfacePatchArrayProperty};
    use quick_xml::{DeError, de};

    #[test]
    fn deserialize_basic_point_array_property() {
        let xml_document = b"<gml:pointMembers>
    <gml:Point>
        <gml:pos>1.0 2.0 3.0</gml:pos>
    </gml:Point>
    <gml:Point>
        <gml:pos>11.0 12.0 13.0</gml:pos>
    </gml:Point>
</gml:pointMembers>";

        let parsed_result: Result<GmlPointArrayProperty, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let point_array_property: PointArrayProperty = parsed_gml.try_into().unwrap();

        assert_eq!(point_array_property.objects.len(), 2);
    }
}
