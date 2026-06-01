/// Qualifies how elements within a geometry aggregate relate to one another.
///
/// Corresponds to `gml:AggregationType` in [OGC 07-036 §7.2.5.2](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregationType {
    /// Unordered collection of unique elements.
    Set,
    /// Unordered collection allowing duplicates.
    Bag,
    /// Ordered list of elements.
    Sequence,
    /// Fixed-size indexed collection.
    Array,
    /// Named-field record.
    Record,
    /// Tabular collection with rows and columns.
    Table,
}
