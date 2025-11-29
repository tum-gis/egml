use crate::Error;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    /// Constructs an Id by hashing the provided bytes using SHA-256.
    /// The resulting Id is a 64-character uppercase hex string.
    pub fn from_hashed_bytes(val: impl AsRef<[u8]>) -> Self {
        Self(Self::hash_bytes_to_hex(val.as_ref()))
    }

    /// Constructs an Id by hashing the provided string using SHA-256.
    pub fn from_hashed_string(val: &str) -> Self {
        Self::from_hashed_bytes(val.as_bytes())
    }

    /// Constructs an Id by hashing the provided u64 using SHA-256.
    pub fn from_hashed_u64(val: u64) -> Self {
        Self::from_hashed_bytes(val.to_le_bytes())
    }

    /// Generate a random UUID v4
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

    fn try_from(item: &String) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::MustNotBeEmpty("id"))
        } else {
            Ok(Self(item.to_string()))
        }
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::MustNotBeEmpty("id"))
        } else {
            Ok(Self(item.to_string()))
        }
    }
}

impl TryFrom<String> for Id {
    type Error = Error;

    fn try_from(item: String) -> Result<Self, Self::Error> {
        if item.is_empty() {
            Err(Error::MustNotBeEmpty("id"))
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
