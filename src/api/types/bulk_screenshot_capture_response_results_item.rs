pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureResponseResultsItem {
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    pub url: BulkScreenshotCaptureResponseResultsItemUrl,
}

impl BulkScreenshotCaptureResponseResultsItem {
    pub fn builder() -> BulkScreenshotCaptureResponseResultsItemBuilder {
        <BulkScreenshotCaptureResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureResponseResultsItemBuilder {
    status: Option<String>,
    error_message: Option<String>,
    url: Option<BulkScreenshotCaptureResponseResultsItemUrl>,
}

impl BulkScreenshotCaptureResponseResultsItemBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn url(mut self, value: BulkScreenshotCaptureResponseResultsItemUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureResponseResultsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BulkScreenshotCaptureResponseResultsItemBuilder::status)
    /// - [`url`](BulkScreenshotCaptureResponseResultsItemBuilder::url)
    pub fn build(self) -> Result<BulkScreenshotCaptureResponseResultsItem, BuildError> {
        Ok(BulkScreenshotCaptureResponseResultsItem {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            error_message: self.error_message,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
