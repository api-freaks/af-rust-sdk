pub use crate::prelude::*;

/// Response format: json (default).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BulkScreenshotCaptureRequestUrlsItemFormat {
    #[serde(rename = "json")]
    Json,
}
impl fmt::Display for BulkScreenshotCaptureRequestUrlsItemFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "json",
        };
        write!(f, "{}", s)
    }
}
