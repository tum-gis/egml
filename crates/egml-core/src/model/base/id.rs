use crate::Error;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Write;
use uuid::Uuid;

/// A stable, globally unique identifier for a GML object.
///
/// Corresponds to the `gml:id` XML attribute ([OGC 07-036 §7.2.4.5](https://docs.ogc.org/is/07-036/07-036.pdf)).
/// An `Id` is a non-empty string; it can be constructed from arbitrary
/// bytes or strings by hashing them with SHA-256, or generated as a
/// random UUID v4.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Id(String);

impl Id {
    /// Constructs an `Id` by hashing bytes using SHA-256.
    ///
    /// The resulting id is a 64-character uppercase hex string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::Id;
    ///
    /// let id = Id::from_hashed_bytes(b"hello");
    /// assert_eq!(id.as_str().len(), 64);
    /// ```
    pub fn from_hashed_bytes(val: impl AsRef<[u8]>) -> Self {
        Self(Self::hash_bytes_to_hex(val.as_ref()))
    }

    /// Constructs an `Id` by hashing a string using SHA-256.
    ///
    /// Two calls with equal strings always produce the same id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::Id;
    ///
    /// let id_a = Id::from_hashed_string("object-42");
    /// let id_b = Id::from_hashed_string("object-42");
    /// assert_eq!(id_a, id_b);
    /// ```
    pub fn from_hashed_string(val: &str) -> Self {
        Self::from_hashed_bytes(val.as_bytes())
    }

    /// Constructs an `Id` by hashing a `u64` using SHA-256 (little-endian bytes).
    pub fn from_hashed_u64(val: u64) -> Self {
        Self::from_hashed_bytes(val.to_le_bytes())
    }

    /// Generates a random UUID v4 as an `Id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::Id;
    ///
    /// let id = Id::generate_uuid_v4();
    /// assert!(!id.as_str().is_empty());
    /// ```
    pub fn generate_uuid_v4() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Generates a time-ordered UUID v7 as an `Id`.
    ///
    /// v7 encodes a millisecond-precision Unix timestamp in the high bits,
    /// making ids lexicographically sortable by creation time — preferable
    /// over v4 when ids are stored in a database index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::Id;
    ///
    /// let a = Id::generate_uuid_v7();
    /// let b = Id::generate_uuid_v7();
    /// assert!(a.as_str() <= b.as_str());
    /// ```
    pub fn generate_uuid_v7() -> Self {
        Self(Uuid::now_v7().to_string())
    }

    /// Generates a deterministic UUID v5 from a namespace and a name.
    ///
    /// Two calls with the same `namespace` and `name` always produce the same
    /// id. Use the predefined namespace constants on [`Uuid`] (e.g.
    /// [`Uuid::NAMESPACE_URL`]) or supply a custom one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::Id;
    /// use uuid::Uuid;
    ///
    /// let id_a = Id::generate_uuid_v5(&Uuid::NAMESPACE_URL, "https://example.com/object-1");
    /// let id_b = Id::generate_uuid_v5(&Uuid::NAMESPACE_URL, "https://example.com/object-1");
    /// assert_eq!(id_a, id_b);
    /// ```
    pub fn generate_uuid_v5(namespace: &Uuid, name: &str) -> Self {
        Self(Uuid::new_v5(namespace, name.as_bytes()).to_string())
    }

    /// Returns the id as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Id {
    fn hash_bytes_to_hex(val: &[u8]) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(val);
        let result = sha256.finalize();

        let mut hash = String::with_capacity(64);
        for byte in result {
            write!(&mut hash, "{:02X}", byte).unwrap();
        }
        hash
    }

    fn validate(s: &str) -> Result<(), Error> {
        if s.is_empty() {
            return Err(Error::EmptyId);
        }
        Ok(())
    }
}

impl From<Id> for String {
    fn from(item: Id) -> Self {
        item.0
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::EmptyId`] if the string slice is empty.
    fn try_from(item: &str) -> Result<Self, Self::Error> {
        Self::validate(item)?;
        Ok(Self(item.to_string()))
    }
}

impl TryFrom<String> for Id {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::EmptyId`] if the owned string is empty.
    fn try_from(item: String) -> Result<Self, Self::Error> {
        Self::validate(&item)?;
        Ok(Self(item))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_empty_string_returns_empty_id_error() {
        assert_eq!(Id::try_from(""), Err(Error::EmptyId));
        assert_eq!(Id::try_from("".to_string()), Err(Error::EmptyId));
    }

    #[test]
    fn from_hashed_string_is_deterministic() {
        let xml = "<gml:Point><gml:pos>1 2 3</gml:pos></gml:Point>";
        assert_eq!(Id::from_hashed_string(xml), Id::from_hashed_string(xml));
    }
}
