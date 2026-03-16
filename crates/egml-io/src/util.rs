use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_space_separated_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}

pub fn serialize_space_separated_f64<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    serializer.serialize_str(&s)
}
