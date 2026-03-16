use crate::Error;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Write;
use uuid::Uuid;

/// A stable, globally unique identifier for a GML object.
///
/// Corresponds to the `gml:id` XML attribute (ISO 19136 §7.2.2).
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
    /// assert_eq!(id.to_string().len(), 64);
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
    /// assert!(!id.to_string().is_empty());
    /// ```
    pub fn generate_uuid_v4() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Id {
    // Low-level helper for hashing bytes
    fn hash_bytes_to_hex(val: &[u8]) -> String {
        let mut sha256 = Sha256::new();
        sha256.update(val);
        let result = sha256.finalize();

        // Preallocate 64-char string
        let mut hash = String::with_capacity(64);
        for byte in result {
            write!(&mut hash, "{:02X}", byte).unwrap();
        }
        hash
    }
}

impl From<Id> for String {
    fn from(item: Id) -> Self {
        item.0
    }
}

impl TryFrom<&String> for Id {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if the string is empty.
    fn try_from(item: &String) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::EmptyCollection("id"))
        } else {
            Ok(Self(item.to_string()))
        }
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if the string slice is empty.
    fn try_from(item: &str) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::EmptyCollection("id"))
        } else {
            Ok(Self(item.to_string()))
        }
    }
}

impl TryFrom<String> for Id {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if the owned string is empty.
    fn try_from(item: String) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::EmptyCollection("id"))
        } else {
            Ok(Self(item))
        }
    }
}

/*impl From<String> for Id {
    fn from(item: String) -> Self {
        Self(item)
    }
}*/

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::EmptyCollection;

    #[test]
    fn id_from_empty_string() {
        let result = Id::try_from("".to_string());

        assert_eq!(result, Err(EmptyCollection("id")));
    }

    #[test]
    fn test() {
        let xml_document = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

        let id_a = Id::from_hashed_string(xml_document);
        let id_b = Id::from_hashed_string(xml_document);

        assert_eq!(id_a, id_b);
    }
}
