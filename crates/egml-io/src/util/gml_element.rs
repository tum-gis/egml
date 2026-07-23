use crate::util::XmlElement;
use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum GmlElement {
    CompositeSurface,
    CurveMemberProperty,
    ExteriorProperty,
    GeometryMemberProperty,
    GeometryMembersProperty,
    InteriorProperty,
    LineString,
    LinearRing,
    MultiCurve,
    MultiGeometry,
    MultiPoint,
    MultiSurface,
    PatchesProperty,
    Point,
    PointMemberProperty,
    PointMembersProperty,
    Polygon,
    PolygonPatch,
    PosListProperty,
    Ring,
    Shell,
    Solid,
    Surface,
    SurfaceMemberProperty,
    Triangle,
    TrianglePatchesProperty,
    TriangulatedSurface,
}

impl XmlElement for GmlElement {
    fn from_local_name(local_name: &[u8]) -> Option<Self> {
        match local_name {
            b"CompositeSurface" => Some(Self::CompositeSurface),
            b"LineString" => Some(Self::LineString),
            b"LinearRing" => Some(Self::LinearRing),
            b"MultiCurve" => Some(Self::MultiCurve),
            b"MultiGeometry" => Some(Self::MultiGeometry),
            b"MultiPoint" => Some(Self::MultiPoint),
            b"MultiSurface" => Some(Self::MultiSurface),
            b"Point" => Some(Self::Point),
            b"Polygon" => Some(Self::Polygon),
            b"PolygonPatch" => Some(Self::PolygonPatch),
            b"Ring" => Some(Self::Ring),
            b"Shell" => Some(Self::Shell),
            b"Solid" => Some(Self::Solid),
            b"Surface" => Some(Self::Surface),
            b"Triangle" => Some(Self::Triangle),
            b"TriangulatedSurface" => Some(Self::TriangulatedSurface),
            b"curveMember" => Some(Self::CurveMemberProperty),
            b"exterior" => Some(Self::ExteriorProperty),
            b"geometryMember" => Some(Self::GeometryMemberProperty),
            b"geometryMembers" => Some(Self::GeometryMembersProperty),
            b"interior" => Some(Self::InteriorProperty),
            b"patches" => Some(Self::PatchesProperty),
            b"pointMember" => Some(Self::PointMemberProperty),
            b"pointMembers" => Some(Self::PointMembersProperty),
            b"posList" => Some(Self::PosListProperty),
            b"surfaceMember" => Some(Self::SurfaceMemberProperty),
            b"trianglePatches" => Some(Self::TrianglePatchesProperty),
            _ => {
                tracing::debug!(
                    "unknown XML element: {}",
                    String::from_utf8_lossy(local_name)
                );
                None
            }
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            GmlElement::CompositeSurface => "gml:CompositeSurface",
            GmlElement::CurveMemberProperty => "gml:curveMember",
            GmlElement::ExteriorProperty => "gml:exterior",
            GmlElement::GeometryMemberProperty => "gml:geometryMember",
            GmlElement::GeometryMembersProperty => "gml:geometryMembers",
            GmlElement::InteriorProperty => "gml:interior",
            GmlElement::LineString => "gml:LineString",
            GmlElement::LinearRing => "gml:LinearRing",
            GmlElement::MultiCurve => "gml:MultiCurve",
            GmlElement::MultiGeometry => "gml:MultiGeometry",
            GmlElement::MultiPoint => "gml:MultiPoint",
            GmlElement::MultiSurface => "gml:MultiSurface",
            GmlElement::PatchesProperty => "gml:patches",
            GmlElement::Point => "gml:Point",
            GmlElement::PointMemberProperty => "gml:pointMember",
            GmlElement::PointMembersProperty => "gml:pointMembers",
            GmlElement::Polygon => "gml:Polygon",
            GmlElement::PolygonPatch => "gml:PolygonPatch",
            GmlElement::PosListProperty => "gml:posList",
            GmlElement::Ring => "gml:Ring",
            GmlElement::Shell => "gml:Shell",
            GmlElement::Solid => "gml:Solid",
            GmlElement::Surface => "gml:Surface",
            GmlElement::SurfaceMemberProperty => "gml:surfaceMember",
            GmlElement::Triangle => "gml:Triangle",
            GmlElement::TrianglePatchesProperty => "gml:trianglePatches",
            GmlElement::TriangulatedSurface => "gml:TriangulatedSurface",
        }
    }
}

impl From<GmlElement> for &'static str {
    fn from(item: GmlElement) -> Self {
        item.as_str()
    }
}
