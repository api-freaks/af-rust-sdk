pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureRequestUrlsItem {
    #[serde(default)]
    pub url: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl BulkScreenshotCaptureRequestUrlsItem {
    pub fn builder() -> BulkScreenshotCaptureRequestUrlsItemBuilder {
        <BulkScreenshotCaptureRequestUrlsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureRequestUrlsItemBuilder {
    url: Option<String>,
}

impl BulkScreenshotCaptureRequestUrlsItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureRequestUrlsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BulkScreenshotCaptureRequestUrlsItemBuilder::url)
    pub fn build(self) -> Result<BulkScreenshotCaptureRequestUrlsItem, BuildError> {
        Ok(BulkScreenshotCaptureRequestUrlsItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            extra: Default::default(),
        })
    }
}
