use crate::{Envelope, Error, LinearRing};

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    id: String,
    exterior: LinearRing,
    interior: Vec<LinearRing>,
}

impl Polygon {
    pub fn new(id: String, exterior: LinearRing, interior: Vec<LinearRing>) -> Result<Self, Error> {
        Ok(Self {
            id,
            exterior,
            interior,
        })
    }

    pub fn exterior(&self) -> &LinearRing {
        &self.exterior
    }

    pub fn set_exterior(&mut self, val: LinearRing) {
        self.exterior = val;
    }

    pub fn interior(&self) -> &Vec<LinearRing> {
        &self.interior
    }

    pub fn set_interior(&mut self, val: Vec<LinearRing>) {
        self.interior = val;
    }

    pub fn get_envelope(&self) -> Envelope {
        self.exterior.get_envelope()
    }
}
