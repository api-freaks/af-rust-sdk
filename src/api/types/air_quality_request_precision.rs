pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AirQualityRequestPrecision {
    #[serde(rename = "hourly")]
    Hourly,
}
impl fmt::Display for AirQualityRequestPrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Hourly => "hourly",
        };
        write!(f, "{}", s)
    }
}
