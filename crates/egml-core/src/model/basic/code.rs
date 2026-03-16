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
/// let code = Code {
///     code_space: Some("https://example.org/codes".to_string()),
///     value: "WallSurface".to_string(),
/// };
/// assert_eq!(code.value, "WallSurface");
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Code {
    /// Optional URI identifying the code list or dictionary.
    pub code_space: Option<String>,
    /// The code value string.
    pub value: String,
}
