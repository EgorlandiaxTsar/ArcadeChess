use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub enum Rescode {
    Ok,
    FailedToDeserialize,
}

impl Rescode {
    fn from(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::Ok),
            1 => Some(Self::FailedToDeserialize),
            _ => None,
        }
    }

    fn code(&self) -> u16 {
        match self {
            Rescode::Ok => 0,
            Rescode::FailedToDeserialize => 1,
        }
    }
}

impl PartialEq for Rescode {
    fn eq(&self, other: &Self) -> bool {
        self.code().eq(&other.code())
    }
}

impl PartialOrd for Rescode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.code().cmp(&other.code()))
    }
}

impl Ord for Rescode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code().cmp(&other.code())
    }
}

impl Serialize for Rescode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.code())
    }
}

impl<'de> Deserialize<'de> for Rescode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer).unwrap_or(Rescode::FailedToDeserialize.code());
        Ok(Rescode::from(code).unwrap_or(Rescode::FailedToDeserialize))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    pub success: bool,
    pub message: String,
    pub rescode: Rescode,
    pub timestamp: u64,
}

impl Metadata {
    fn new(success: bool, message: String, rescode: Rescode) -> Self {
        Metadata {
            success,
            message,
            rescode,
            timestamp: chrono::offset::Utc::now().timestamp_millis() as u64,
        }
    }
}

impl Ord for Metadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Metadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}
