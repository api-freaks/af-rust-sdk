pub use crate::prelude::*;

/// Indicates that this response contains historical data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainWhoisHistoryResponseWhois {
    #[serde(rename = "historical")]
    Historical,
}
impl fmt::Display for DomainWhoisHistoryResponseWhois {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Historical => "historical",
        };
        write!(f, "{}", s)
    }
}
