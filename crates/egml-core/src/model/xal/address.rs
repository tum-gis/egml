use crate::model::xal::country::Country;
use crate::model::xal::locality::Locality;
use crate::model::xal::thoroughfare::Thoroughfare;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Address {
    country: Option<Country>,
    locality: Option<Locality>,
    thoroughfare: Option<Thoroughfare>,
}

impl Address {
    pub fn new() -> Self {
        Self {
            country: None,
            locality: None,
            thoroughfare: None,
        }
    }

    pub fn country(&self) -> Option<&Country> {
        self.country.as_ref()
    }

    pub fn set_country(&mut self, country: Country) {
        self.country = Some(country);
    }

    pub fn set_country_opt(&mut self, country: Option<Country>) {
        self.country = country;
    }

    pub fn locality(&self) -> Option<&Locality> {
        self.locality.as_ref()
    }

    pub fn set_locality(&mut self, locality: Locality) {
        self.locality = Some(locality);
    }

    pub fn set_locality_opt(&mut self, locality: Option<Locality>) {
        self.locality = locality;
    }

    pub fn thoroughfare(&self) -> Option<&Thoroughfare> {
        self.thoroughfare.as_ref()
    }

    pub fn set_thoroughfare(&mut self, thoroughfare: Thoroughfare) {
        self.thoroughfare = Some(thoroughfare);
    }

    pub fn set_thoroughfare_opt(&mut self, thoroughfare: Option<Thoroughfare>) {
        self.thoroughfare = thoroughfare;
    }
}
