use crate::Error;
use crate::primitives::{GmlPointArrayProperty, GmlPointProperty};
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::aggregates::MultiPoint;
use egml_core::model::geometry::primitives::{PointArrayProperty, PointProperty};
use egml_core::model::geometry::{AsAbstractGeometry, AsAbstractGeometryMut};
use quick_xml::{DeError, de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiPoint {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(rename = "@srsDimension", skip_serializing_if = "Option::is_none")]
    srs_dimension: Option<u32>,

    #[serde(
        rename(serialize = "gml:pointMember", deserialize = "pointMember"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    point_member: Vec<GmlPointProperty>,

    #[serde(
        rename(serialize = "gml:pointMembers", deserialize = "pointMembers"),
        skip_serializing_if = "Option::is_none"
    )]
    point_members: Option<GmlPointArrayProperty>,
}

impl TryFrom<GmlMultiPoint> for MultiPoint {
    type Error = Error;

    fn try_from(item: GmlMultiPoint) -> Result<Self, Self::Error> {
        let id = item.id.map(|id| id.try_into()).transpose()?;

        let point_member: Vec<PointProperty> = item
            .point_member
            .iter()
            .map(|x| x.clone().try_into())
            .collect::<Result<Vec<PointProperty>, Error>>()?;

        let point_members: Option<PointArrayProperty> =
            item.point_members.map(|id| id.try_into()).transpose()?;

        let mut multi_point = MultiPoint::new(point_members)?;
        multi_point.set_id(id);
        multi_point.set_srs_dimension(item.srs_dimension);
        multi_point.set_point_member(point_member);
        Ok(multi_point)
    }
}

impl From<&MultiPoint> for GmlMultiPoint {
    fn from(multi_point: &MultiPoint) -> Self {
        Self {
            id: multi_point.id().map(|id| id.to_string()),
            srs_dimension: multi_point.srs_dimension(),
            point_member: multi_point
                .point_member()
                .iter()
                .map(|x| x.into())
                .collect(),
            point_members: multi_point.point_members().map(|x| x.into()),
        }
    }
}

pub fn deserialize_multi_point<R: BufRead>(reader: R) -> Result<MultiPoint, Error> {
    let parsed_geometry: Result<GmlMultiPoint, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

/// Serializes a [`MultiPoint`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_multi_point(multi_point: &MultiPoint) -> Result<String, Error> {
    let gml = GmlMultiPoint::from(multi_point);
    Ok(se::to_string_with_root("gml:MultiPoint", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlMultiPoint;
    use crate::aggregates::multi_point::{deserialize_multi_point, serialize_multi_point};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiPoint;
    use egml_core::model::geometry::primitives::{Point, PointArrayProperty, PointProperty};
    use quick_xml::de;

    fn make_point(x: f64, y: f64, z: f64) -> Point {
        Point::new(DirectPosition::new(x, y, z).unwrap())
    }

    fn make_point_property(x: f64, y: f64, z: f64) -> PointProperty {
        PointProperty::new(make_point(x, y, z))
    }

    fn make_multi_point() -> MultiPoint {
        let p1 = make_point(678267.6213956032, 5403783.626290152, 366.96639999999996);
        let p2 = make_point(678289.06567932, 5403807.373180328, 366.99789425533834);
        let point_members = PointArrayProperty::new([p1, p2]);

        let mut multi_point = MultiPoint::new(Some(point_members)).unwrap();
        multi_point.set_point_member(vec![
            make_point_property(678267.6213956032, 5403783.626290152, 366.96639999999996),
            make_point_property(678289.06567932, 5403807.373180328, 366.99789425533834),
        ]);
        multi_point
    }

    #[test]
    fn deserialize_point_member_only() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMember>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMember>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMember>
        </gml:MultiPoint>";

        let multi_point = deserialize_multi_point(xml.as_ref()).unwrap();

        assert_eq!(multi_point.point_member().len(), 2);
        assert!(multi_point.point_members().is_none());
    }

    #[test]
    fn deserialize_point_members_only() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMembers>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMembers>
        </gml:MultiPoint>";

        let multi_point = deserialize_multi_point(xml.as_ref()).unwrap();

        assert!(multi_point.point_member().is_empty());
        assert_eq!(multi_point.point_members().unwrap().objects.len(), 2);
    }

    #[test]
    fn deserialize_both_point_member_and_point_members() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMember>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMember>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMembers>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMembers>
        </gml:MultiPoint>";

        let multi_point = deserialize_multi_point(xml.as_ref()).unwrap();

        assert_eq!(multi_point.point_member().len(), 2);
        assert_eq!(multi_point.point_members().unwrap().objects.len(), 2);
    }

    #[test]
    fn serialize_multi_point_writes_gml_tags() {
        let multi_point = make_multi_point();
        let xml = serialize_multi_point(&multi_point).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:pointMembers"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_preserves_member_counts() {
        let original = make_multi_point();
        let xml = serialize_multi_point(&original).unwrap();
        let recovered = deserialize_multi_point(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.point_member().len(),
            original.point_member().len()
        );
        assert_eq!(
            recovered.point_members().map(|m| m.objects.len()),
            original.point_members().map(|m| m.objects.len()),
        );
    }

    #[test]
    fn round_trip_from_xml() {
        let input_xml = "<gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            <gml:pointMembers><gml:Point><gml:pos srsDimension=\"3\">4 5 6</gml:pos></gml:Point></gml:pointMembers>\
            </gml:MultiPoint>";

        let gml: GmlMultiPoint = de::from_reader(input_xml.as_bytes()).unwrap();
        let multi_point: MultiPoint = gml.try_into().unwrap();
        let output_xml = serialize_multi_point(&multi_point).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
