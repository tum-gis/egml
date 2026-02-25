#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregationType {
    Set,
    Bag,
    Sequence,
    Array,
    Record,
    Table,
}
