use crate::model::xal::country_name::CountryName;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Country {
    name_elements: Vec<CountryName>,
}

impl Country {
    pub fn new(name_elements: Vec<CountryName>) -> Self {
        Self { name_elements }
    }

    pub fn name_elements(&self) -> &[CountryName] {
        &self.name_elements
    }
}
