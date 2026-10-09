use crate::model::xal::enums::ThoroughfareNameType;

/// One `xAL:NameElement` of a [`Thoroughfare`](crate::model::xal::Thoroughfare)
/// — a name fragment plus what it represents (e.g. the street name vs. a
/// pre/post direction).
#[derive(Debug, Clone, PartialEq)]
pub struct ThoroughfareName {
    content: String,
    name_type: Option<ThoroughfareNameType>,
}

impl ThoroughfareName {
    pub fn new(content: String) -> Self {
        Self {
            content,
            name_type: None,
        }
    }

    pub fn with_name_type(content: String, name_type: ThoroughfareNameType) -> Self {
        Self {
            content,
            name_type: Some(name_type),
        }
    }

    /// Equivalent to [`with_name_type`](Self::with_name_type) when `name_type`
    /// is `Some`, and to [`new`](Self::new) when it's `None`.
    pub fn with_optional_name_type(
        content: String,
        name_type: Option<ThoroughfareNameType>,
    ) -> Self {
        match name_type {
            Some(name_type) => Self::with_name_type(content, name_type),
            None => Self::new(content),
        }
    }

    /// The name text itself, e.g. `"Baker"`.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// What this name represents, e.g. `NameOnly` vs. `PostDirection`.
    pub fn name_type(&self) -> Option<ThoroughfareNameType> {
        self.name_type
    }
}
