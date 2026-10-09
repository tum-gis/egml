use chrono::{FixedOffset, Offset, Utc};

/// Tuning knobs for deserialization. Every `deserialize_*` function that
/// takes an [`XmlDocumentIndex`](crate::util::XmlDocumentIndex) also takes a
/// `&DeserializationConfig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeserializationConfig {
    /// `max_depth` passed to
    /// [`XmlDocumentIndex::from_scan`](crate::util::XmlDocumentIndex::from_scan)
    /// when a [`truncated`](crate::util::XmlDocumentIndex::is_truncated)
    /// index needs to be rescanned mid-deserialization.
    rescan_depth: Option<usize>,

    /// Timezone assigned to `xs:date` and `xs:dateTime` values that have none.
    default_offset: FixedOffset,
}

impl Default for DeserializationConfig {
    fn default() -> Self {
        Self {
            rescan_depth: Some(5),
            default_offset: Utc.fix(),
        }
    }
}

impl DeserializationConfig {
    /// Creates a config with the given rescan depth and a UTC default offset.
    pub fn new(rescan_depth: Option<usize>) -> Self {
        Self {
            rescan_depth,
            ..Self::default()
        }
    }

    /// Returns a copy that assigns `default_offset` to `xs:date` and
    /// `xs:dateTime` values without a timezone.
    pub fn with_default_offset(self, default_offset: FixedOffset) -> Self {
        Self {
            default_offset,
            ..self
        }
    }

    pub fn rescan_depth(&self) -> Option<usize> {
        self.rescan_depth
    }

    pub fn set_rescan_depth(&mut self, geometry_scan_depth: Option<usize>) {
        self.rescan_depth = geometry_scan_depth;
    }

    /// Returns the timezone assigned to `xs:date` and `xs:dateTime` values
    /// that have none (UTC by default).
    pub fn default_offset(&self) -> FixedOffset {
        self.default_offset
    }

    pub fn set_default_offset(&mut self, default_offset: FixedOffset) {
        self.default_offset = default_offset;
    }
}
