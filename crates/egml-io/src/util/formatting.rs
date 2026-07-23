/// Controls whitespace formatting of serialized XML output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Formatting {
    /// All elements on a single line with no whitespace between them.
    Compact,
    /// Each element on its own line with no indentation.
    #[default]
    NewLine,
    /// Each element on its own line, indented by `size` repetitions of `char`.
    Indent { char: char, size: usize },
}
