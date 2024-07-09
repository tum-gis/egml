use crate::error::Error;
use crate::{enlarge_envelopes, Envelope, LinearRing};

#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    members: Vec<LinearRing>,
}

impl Solid {
    pub fn new(members: Vec<LinearRing>) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty(""));
        }

        Ok(Self { members })
    }

    pub fn members(&self) -> &Vec<LinearRing> {
        self.members.as_ref()
    }

    pub fn set_members(&mut self, val: Vec<LinearRing>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty(""));
        }
        self.members = val;
        Ok(())
    }

    pub fn get_envelope(&self) -> Result<Envelope, Error> {
        let envelopes: Vec<Envelope> = self.members.iter().map(|m| m.get_envelope()).collect();

        let enlarged_envelope = enlarge_envelopes(&envelopes)?;
        Ok(enlarged_envelope)
    }
}
