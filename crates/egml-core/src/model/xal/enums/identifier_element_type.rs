#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierElementType {
    Name,
    RangeFrom,
    Range,
    RangeTo,
    Prefix,
    Suffix,
    Number,
    Separator,
    Extension,
}
