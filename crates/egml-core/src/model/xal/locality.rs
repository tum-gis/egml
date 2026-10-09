use crate::model::xal::LocalityName;
use crate::model::xal::enums::LocalityType;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Locality {
    name_elements: Vec<LocalityName>,
    locality_type: Option<LocalityType>,
}

impl Locality {
    pub fn new(name_elements: Vec<LocalityName>) -> Self {
        Self {
            name_elements,
            locality_type: None,
        }
    }

    /// Returns the locality's `xAL:NameElement` values.
    pub fn name_elements(&self) -> &[LocalityName] {
        &self.name_elements
    }

    pub fn locality_type(&self) -> Option<LocalityType> {
        self.locality_type
    }

    pub fn set_locality_type(&mut self, locality_type: LocalityType) {
        self.locality_type = Some(locality_type);
    }
}
