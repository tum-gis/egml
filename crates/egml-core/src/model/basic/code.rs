/// A coded value optionally qualified by a code-space URI.
///
/// Corresponds to `gml:CodeType` in ISO 19136.  The optional `code_space`
/// attribute is a URI that identifies the code list or dictionary in which
/// `value` is defined.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::basic::Code;
///
///
/// let code = Code::with_code_space("https://example.org/codes", "WallSurface");
/// assert_eq!(code.value(), "WallSurface");
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Code {
    /// Optional URI identifying the code list or dictionary.
    code_space: Option<String>,
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
    pub fn with_code_space(code_space: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            code_space: Some(code_space.into()),
            value: value.into(),
        }
    }

    pub fn from_parts(code_space: Option<impl Into<String>>, value: impl Into<String>) -> Self {
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
    pub fn code_space(&self) -> Option<&str> {
        self.code_space.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn set_code_space<S: Into<String>>(&mut self, code_space: Option<S>) {
        self.code_space = code_space.map(Into::into);
    }
}
