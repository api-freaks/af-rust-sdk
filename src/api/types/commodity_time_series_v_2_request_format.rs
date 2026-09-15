pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CommodityTimeSeriesV2RequestFormat {
    #[serde(rename = "json")]
    Json,
}
impl fmt::Display for CommodityTimeSeriesV2RequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "json",
        };
        write!(f, "{}", s)
    }
}
