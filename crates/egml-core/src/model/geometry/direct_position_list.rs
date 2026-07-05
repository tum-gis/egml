#[derive(Debug, PartialEq, Clone)]
pub struct DirectPositionList {
    srs_dimension: Option<u32>,

    values: Vec<f64>,
}

impl DirectPositionList {
    pub fn new(values: impl IntoIterator<Item = f64>) -> Self {
        Self {
            srs_dimension: None,
            values: values.into_iter().collect(),
        }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn srs_dimension(&self) -> Option<u32> {
        self.srs_dimension
    }

    pub fn set_srs_dimension(&mut self, val: Option<u32>) {
        self.srs_dimension = val;
    }

    pub fn push_value(&mut self, value: f64) {
        self.values.push(value);
    }

    pub fn extend_values(&mut self, values: impl IntoIterator<Item = f64>) {
        self.values.extend(values);
    }
}
