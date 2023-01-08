use crate::error::Error;
use crate::LinearRing;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    members: Vec<LinearRing>,
}

impl MultiSurface {
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
}
