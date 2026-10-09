use crate::model::xal::{Identifier, ThoroughfareName};

/// One item of a [`Thoroughfare`]'s content. The schema interleaves
/// `NameElement` and `Number` in document order
/// (`<xs:choice maxOccurs="unbounded">`), so this preserves that ordering
/// instead of splitting them into two separate lists.
#[derive(Debug, Clone, PartialEq)]
pub enum ThoroughfareNameOrNumber {
    Name(ThoroughfareName),
    Number(Identifier),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thoroughfare {
    elements: Vec<ThoroughfareNameOrNumber>,
    /// Free text, e.g. `"Street"`, `"Lane"` — `ThoroughfareTypeList` has no
    /// enumeration in the schema, so this isn't a closed set.
    thoroughfare_type: Option<String>,
    type_code: Option<String>,
}

impl Thoroughfare {
    pub fn new(elements: Vec<ThoroughfareNameOrNumber>) -> Self {
        Self {
            elements,
            thoroughfare_type: None,
            type_code: None,
        }
    }

    /// Returns the thoroughfare's `xAL:NameElement`/`xAL:Number` content,
    /// in document order.
    pub fn elements(&self) -> &[ThoroughfareNameOrNumber] {
        &self.elements
    }

    pub fn thoroughfare_type(&self) -> Option<&str> {
        self.thoroughfare_type.as_deref()
    }

    pub fn set_thoroughfare_type(&mut self, thoroughfare_type: impl Into<String>) {
        self.thoroughfare_type = Some(thoroughfare_type.into());
    }

    pub fn type_code(&self) -> Option<&str> {
        self.type_code.as_deref()
    }

    pub fn set_type_code(&mut self, type_code: impl Into<String>) {
        self.type_code = Some(type_code.into());
    }
}
