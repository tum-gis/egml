use crate::{Error, GmlEnvelope};
use egml_core::model::feature::BoundingShape;
use egml_core::model::geometry::Envelope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBoundingShape {
    #[serde(
        rename(serialize = "gml:Envelope", deserialize = "Envelope"),
        skip_serializing_if = "Option::is_none"
    )]
    pub envelope: Option<GmlEnvelope>,
}

impl TryFrom<GmlBoundingShape> for BoundingShape {
    type Error = Error;

    fn try_from(item: GmlBoundingShape) -> Result<Self, Self::Error> {
        let envelope: Option<Envelope> = item.envelope.map(|x| x.try_into()).transpose()?;

        let bounding_shape = BoundingShape::new_unchecked(envelope, None);
        Ok(bounding_shape)
    }
}

impl From<&BoundingShape> for GmlBoundingShape {
    fn from(item: &BoundingShape) -> Self {
        Self {
            envelope: item.envelope.as_ref().map(|x| x.into()),
        }
    }
}
