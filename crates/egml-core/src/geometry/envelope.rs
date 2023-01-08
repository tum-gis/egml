use crate::DirectPosition;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Envelope {
    lower_corner: DirectPosition,
    upper_corner: DirectPosition,
}

impl Envelope {
    pub fn new(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Self {
        assert!(
            lower_corner <= upper_corner,
            "Lower corner must be below upper corner"
        );

        Self {
            lower_corner,
            upper_corner,
        }
    }

    pub fn contains(&self, point: &DirectPosition) -> bool {
        &self.lower_corner <= point && point <= &self.upper_corner
    }
}
