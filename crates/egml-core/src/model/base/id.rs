use crate::Error;
use sha2::{Digest, Sha256};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn from_hashed_string(val: &str) -> Self {
        let mut sha256 = Sha256::new();
        sha256.update(val);
        let hash: String = format!("{:X}", sha256.finalize());
        Self(hash)
    }

    pub fn from_hashed_u64(val: u64) -> Self {
        let mut sha256 = Sha256::new();
        sha256.update(val.to_le_bytes());
        let hash: String = format!("{:X}", sha256.finalize());
        Self(hash)
    }

    pub fn generate_uuid_v4() -> Self {
        let uuid: String = Uuid::new_v4().into();
        Self(uuid)
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
