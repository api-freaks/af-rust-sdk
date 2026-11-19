pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScreenshotCaptureRequestFormat {
    #[serde(rename = "json")]
    Json,
}
impl fmt::Display for ScreenshotCaptureRequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "json",
        };
        write!(f, "{}", s)
    }
}
