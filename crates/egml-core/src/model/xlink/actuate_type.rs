/// The value of the `xlink:actuate` attribute on a simple XLink.
///
/// Controls when a link should be traversed — automatically on load or
/// explicitly on user request. Corresponds to the W3C XLink `actuateType`
/// simple type defined in [XLink 1.1 §5.4](https://www.w3.org/TR/xlink11/#actuate-att).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActuateType {
    /// Traverse the link automatically when the containing document is loaded.
    OnLoad,
    /// Traverse the link only when explicitly requested (e.g. a user click).
    OnRequest,
    /// Actuate behaviour is defined by the application; not one of the standard values.
    Other,
    /// No actuate behaviour is defined.
    None,
}

impl ActuateType {
    /// Returns the canonical XLink string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ActuateType::OnLoad => "onLoad",
            ActuateType::OnRequest => "onRequest",
            ActuateType::Other => "other",
            ActuateType::None => "none",
        }
    }
}

impl std::fmt::Display for ActuateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ActuateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "onLoad" => Ok(ActuateType::OnLoad),
            "onRequest" => Ok(ActuateType::OnRequest),
            "other" => Ok(ActuateType::Other),
            "none" => Ok(ActuateType::None),
            other => Err(format!("unknown xlink:actuate value '{other}'")),
        }
    }
}
