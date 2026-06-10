pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureRequest {
    /// List of website URLs to capture screenshots of
    #[serde(default)]
    pub urls: Vec<BulkScreenshotCaptureRequestUrlsItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<BulkScreenshotCaptureRequestFormat>,
}

impl BulkScreenshotCaptureRequest {
    pub fn builder() -> BulkScreenshotCaptureRequestBuilder {
        <BulkScreenshotCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureRequestBuilder {
    urls: Option<Vec<BulkScreenshotCaptureRequestUrlsItem>>,
    api_key: Option<String>,
    format: Option<BulkScreenshotCaptureRequestFormat>,
}

impl BulkScreenshotCaptureRequestBuilder {
    pub fn urls(mut self, value: Vec<BulkScreenshotCaptureRequestUrlsItem>) -> Self {
        self.urls = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkScreenshotCaptureRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`urls`](BulkScreenshotCaptureRequestBuilder::urls)
    /// - [`api_key`](BulkScreenshotCaptureRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkScreenshotCaptureRequest, BuildError> {
        Ok(BulkScreenshotCaptureRequest {
            urls: self.urls.ok_or_else(|| BuildError::missing_field("urls"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
