/// The value of the `xlink:show` attribute on a simple XLink.
///
/// Controls how the referenced resource should be presented when the link is
/// traversed. Corresponds to the W3C XLink `showType` simple type defined in
/// [XLink 1.1 §5.4](https://www.w3.org/TR/xlink11/#show-att).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShowType {
    /// Load the resource in a new window, frame, or presentation context.
    New,
    /// Load the resource in the same presentation context, replacing the current content.
    Replace,
    /// Embed the resource inline at the location of the link.
    Embed,
    /// Show behaviour is defined by the application; not one of the standard values.
    Other,
    /// No show behaviour is defined.
    None,
}

impl ShowType {
    /// Returns the canonical XLink string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ShowType::New => "new",
            ShowType::Replace => "replace",
            ShowType::Embed => "embed",
            ShowType::Other => "other",
            ShowType::None => "none",
        }
    }
}

impl std::fmt::Display for ShowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ShowType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(ShowType::New),
            "replace" => Ok(ShowType::Replace),
            "embed" => Ok(ShowType::Embed),
            "other" => Ok(ShowType::Other),
            "none" => Ok(ShowType::None),
            other => Err(format!("unknown xlink:show value '{other}'")),
        }
    }
}
