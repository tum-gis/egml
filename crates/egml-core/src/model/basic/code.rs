#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Code {
    pub code_space: Option<String>,
    pub value: String,
}
