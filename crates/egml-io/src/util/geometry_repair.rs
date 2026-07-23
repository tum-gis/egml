use egml_core::model::geometry::DirectPosition;
use tracing::debug;

/// Collapses exact adjacent duplicate positions in a parsed coordinate sequence.
///
/// Some real-world GML exports repeat a position between two elements (e.g. a stray
/// duplicated vertex in a `posList`). The `egml-core` constructors reject this via
/// `Error::AdjacentDuplicatePositions`, so deserialization repairs it here instead of
/// failing on otherwise well-formed geometry.
pub(crate) fn dedup_adjacent_positions(points: &mut Vec<DirectPosition>, geometry: &'static str) {
    let before = points.len();
    points.dedup();
    let removed = before - points.len();
    if removed > 0 {
        debug!(removed, geometry, "repaired adjacent duplicate positions");
    }
}

#[cfg(test)]
mod tests {
    use super::dedup_adjacent_positions;
    use egml_core::model::geometry::DirectPosition;

    #[test]
    fn removes_adjacent_duplicates_only() {
        let mut points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
        ];

        dedup_adjacent_positions(&mut points, "test");

        assert_eq!(
            points,
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            ]
        );
    }

    #[test]
    fn leaves_non_adjacent_duplicates_untouched() {
        let mut points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
        ];
        let original = points.clone();

        dedup_adjacent_positions(&mut points, "test");

        assert_eq!(points, original);
    }
}
