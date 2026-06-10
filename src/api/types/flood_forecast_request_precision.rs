pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FloodForecastRequestPrecision {
    #[serde(rename = "daily")]
    Daily,
}
impl fmt::Display for FloodForecastRequestPrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Daily => "daily",
        };
        write!(f, "{}", s)
    }
}
