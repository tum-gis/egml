use crate::model::base::AbstractGml;
use crate::model::geometry::Envelope;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbstractFeature {
    pub abstract_gml: AbstractGml,

    pub bounded_by: Option<Envelope>,
}

impl AbstractFeature {
    pub fn new(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            bounded_by: None,
        }
    }
}
