pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkScreenshotCaptureResponseMetaData {
    #[serde(default)]
    pub total_urls: i64,
    #[serde(default)]
    pub successful_urls: i64,
    #[serde(default)]
    pub failed_urls: i64,
}

impl BulkScreenshotCaptureResponseMetaData {
    pub fn builder() -> BulkScreenshotCaptureResponseMetaDataBuilder {
        <BulkScreenshotCaptureResponseMetaDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureResponseMetaDataBuilder {
    total_urls: Option<i64>,
    successful_urls: Option<i64>,
    failed_urls: Option<i64>,
}

impl BulkScreenshotCaptureResponseMetaDataBuilder {
    pub fn total_urls(mut self, value: i64) -> Self {
        self.total_urls = Some(value);
        self
    }

    pub fn successful_urls(mut self, value: i64) -> Self {
        self.successful_urls = Some(value);
        self
    }

    pub fn failed_urls(mut self, value: i64) -> Self {
        self.failed_urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureResponseMetaData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_urls`](BulkScreenshotCaptureResponseMetaDataBuilder::total_urls)
    /// - [`successful_urls`](BulkScreenshotCaptureResponseMetaDataBuilder::successful_urls)
    /// - [`failed_urls`](BulkScreenshotCaptureResponseMetaDataBuilder::failed_urls)
    pub fn build(self) -> Result<BulkScreenshotCaptureResponseMetaData, BuildError> {
        Ok(BulkScreenshotCaptureResponseMetaData {
            total_urls: self
                .total_urls
                .ok_or_else(|| BuildError::missing_field("total_urls"))?,
            successful_urls: self
                .successful_urls
                .ok_or_else(|| BuildError::missing_field("successful_urls"))?,
            failed_urls: self
                .failed_urls
                .ok_or_else(|| BuildError::missing_field("failed_urls"))?,
        })
    }
}
