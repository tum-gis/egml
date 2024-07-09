use crate::error::Error;
use crate::geometry::primitives::polygon::Polygon;
use crate::{enlarge_envelopes, Envelope};

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    members: Vec<Polygon>,
}

impl MultiSurface {
    pub fn new(members: Vec<Polygon>) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty(""));
        }

        Ok(Self { members })
    }

    pub fn members(&self) -> &Vec<Polygon> {
        self.members.as_ref()
    }

    pub fn set_members(&mut self, val: Vec<Polygon>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty(""));
        }
        self.members = val;
        Ok(())
    }

    pub fn get_envelope(&self) -> Result<Envelope, Error> {
        let envelopes: Vec<Envelope> = self
            .members
            .iter()
            .map(|m| m.get_envelope())
            .collect::<Vec<_>>();

        let enlarged_envelope = enlarge_envelopes(&envelopes)?;
        Ok(enlarged_envelope)
    }
}
