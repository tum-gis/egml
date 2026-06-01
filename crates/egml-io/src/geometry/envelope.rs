use crate::{Error, GmlDirectPosition};
use egml_core::model::geometry::{DirectPosition, Envelope};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlEnvelope {
    #[serde(rename(serialize = "gml:lowerCorner", deserialize = "lowerCorner"))]
    lower_corner: GmlDirectPosition,
    #[serde(rename(serialize = "gml:upperCorner", deserialize = "upperCorner"))]
    upper_corner: GmlDirectPosition,

    #[serde(rename = "@srsName")]
    srs_name: Option<String>,
    #[serde(rename = "@srsDimension")]
    srs_dimension: Option<u8>,
}

impl TryFrom<GmlEnvelope> for Envelope {
    type Error = Error;

    fn try_from(item: GmlEnvelope) -> Result<Self, Self::Error> {
        if item.srs_dimension.unwrap_or(3) != 3 {
            return Err(Error::UnsupportedDimension {
                found: item.srs_dimension.unwrap_or(0) as u32,
            });
        }

        let lower_corner: DirectPosition = item.lower_corner.try_into()?;
        let upper_corner: DirectPosition = item.upper_corner.try_into()?;

        let envelope = Envelope::new(lower_corner, upper_corner)?;
        Ok(envelope)
    }
}

impl From<&Envelope> for GmlEnvelope {
    fn from(item: &Envelope) -> Self {
        Self {
            lower_corner: item.lower_corner().into(),
            upper_corner: item.upper_corner().into(),
            srs_name: None,
            srs_dimension: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::envelope::GmlEnvelope;
    use egml_core::model::geometry::Envelope;
    use quick_xml::{DeError, de};

    #[test]
    fn deserialize_envelope() {
        let xml_document = "<gml:Envelope srsDimension=\"3\">
<gml:lowerCorner>1.0 2.0 3.0</gml:lowerCorner>
<gml:upperCorner>11.0 12.0 13.0</gml:upperCorner>
</gml:Envelope>";

        let parsed_result: Result<GmlEnvelope, DeError> = de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let envelope: Envelope = parsed_gml.try_into().unwrap();

        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }
}
