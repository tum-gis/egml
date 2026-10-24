use crate::model::xs::AnyUri;

/// A coded value optionally qualified by a code-space URI.
///
/// Corresponds to `gml:CodeType` in ISO 19136.  The optional `code_space`
/// attribute is an [`AnyUri`] that identifies the code list or dictionary in
/// which `value` is defined.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::basic_types::Code;
///
///
/// let code = Code::with_code_space("https://example.org/codes", "WallSurface");
/// assert_eq!(code.value(), "WallSurface");
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct Code {
    /// Optional URI identifying the code list or dictionary.
    code_space: Option<AnyUri>,
    /// The code value string.
    value: String,
}

impl Code {
    /// Creates a new `Code` without a code space.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            code_space: None,
            value: value.into(),
        }
    }

    /// Creates a new `Code` with a code space.
    pub fn with_code_space(code_space: impl Into<AnyUri>, value: impl Into<String>) -> Self {
        Self {
            code_space: Some(code_space.into()),
            value: value.into(),
        }
    }

    /// Creates a new `Code` with an optional code space.
    pub fn from_parts(code_space: Option<impl Into<AnyUri>>, value: impl Into<String>) -> Self {
        Self {
            code_space: code_space.map(Into::into),
            value: value.into(),
        }
    }

    /// Returns the code value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the optional code-space URI.
    pub fn code_space(&self) -> Option<&AnyUri> {
        self.code_space.as_ref()
    }

    /// Sets the code value.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    /// Sets the code-space URI.
    pub fn set_code_space(&mut self, code_space: impl Into<AnyUri>) {
        self.code_space = Some(code_space.into());
    }

    /// Sets or clears the code-space URI.
    pub fn set_code_space_opt(&mut self, code_space: Option<AnyUri>) {
        self.code_space = code_space;
    }

    /// Clears the code-space URI.
    pub fn clear_code_space(&mut self) {
        self.code_space = None;
    }
}

impl From<String> for Code {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Code {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE_SPACE: &str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml";

    #[test]
    fn new_has_no_code_space() {
        let code = Code::new("1000");

        assert_eq!(code.value(), "1000");
        assert_eq!(code.code_space(), None);
    }

    #[test]
    fn code_space_accepts_strings_and_any_uri() {
        assert_eq!(
            Code::with_code_space(CODE_SPACE, "1000")
                .code_space()
                .map(AnyUri::as_str),
            Some(CODE_SPACE)
        );
        assert_eq!(
            Code::with_code_space(AnyUri::new(CODE_SPACE), "1000")
                .code_space()
                .map(AnyUri::as_str),
            Some(CODE_SPACE)
        );
        assert_eq!(
            Code::from_parts(Some(CODE_SPACE.to_owned()), "1000")
                .code_space()
                .map(AnyUri::as_str),
            Some(CODE_SPACE)
        );
        assert_eq!(Code::from_parts(None::<AnyUri>, "1000").code_space(), None);
    }

    #[test]
    fn code_space_setters() {
        let mut code = Code::new("1000");

        code.set_code_space(CODE_SPACE);
        assert_eq!(code.code_space().map(AnyUri::as_str), Some(CODE_SPACE));

        code.clear_code_space();
        assert_eq!(code.code_space(), None);

        code.set_code_space_opt(Some(AnyUri::new(CODE_SPACE)));
        assert_eq!(code.code_space().map(AnyUri::as_str), Some(CODE_SPACE));

        code.set_code_space_opt(None);
        assert_eq!(code.code_space(), None);
    }
}
