use crate::model::xal::enums::IdentifierElementType;

/// A single fragment of a compound identifier — e.g. a thoroughfare
/// `Number`, a post code segment. Reused across several xAL elements
/// wherever `ct:IdentifierType` appears (`Thoroughfare::Number`,
/// `PostCode`, `RuralDelivery`, `PostalDeliveryPoint`, `PostOffice`, ...).
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    content: String,
    identifier_type: Option<IdentifierElementType>,
}

impl Identifier {
    pub fn new(content: String) -> Self {
        Self {
            content,
            identifier_type: None,
        }
    }

    pub fn with_identifier_type(content: String, identifier_type: IdentifierElementType) -> Self {
        Self {
            content,
            identifier_type: Some(identifier_type),
        }
    }

    /// Equivalent to [`with_identifier_type`](Self::with_identifier_type) when
    /// `identifier_type` is `Some`, and to [`new`](Self::new) when it's `None`.
    pub fn with_optional_identifier_type(
        content: String,
        identifier_type: Option<IdentifierElementType>,
    ) -> Self {
        match identifier_type {
            Some(identifier_type) => Self::with_identifier_type(content, identifier_type),
            None => Self::new(content),
        }
    }

    /// The identifier text itself, e.g. `"39"`.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// What this fragment represents, e.g. `Number` vs. `RangeFrom`.
    pub fn identifier_type(&self) -> Option<IdentifierElementType> {
        self.identifier_type
    }
}
