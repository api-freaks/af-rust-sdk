pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub meta_data: BulkScreenshotCaptureResponseMetaData,
    #[serde(default)]
    pub results: Vec<BulkScreenshotCaptureResponseResultsItem>,
}

impl BulkScreenshotCaptureResponse {
    pub fn builder() -> BulkScreenshotCaptureResponseBuilder {
        <BulkScreenshotCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureResponseBuilder {
    status: Option<String>,
    meta_data: Option<BulkScreenshotCaptureResponseMetaData>,
    results: Option<Vec<BulkScreenshotCaptureResponseResultsItem>>,
}

impl BulkScreenshotCaptureResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn meta_data(mut self, value: BulkScreenshotCaptureResponseMetaData) -> Self {
        self.meta_data = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<BulkScreenshotCaptureResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BulkScreenshotCaptureResponseBuilder::status)
    /// - [`meta_data`](BulkScreenshotCaptureResponseBuilder::meta_data)
    /// - [`results`](BulkScreenshotCaptureResponseBuilder::results)
    pub fn build(self) -> Result<BulkScreenshotCaptureResponse, BuildError> {
        Ok(BulkScreenshotCaptureResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            meta_data: self
                .meta_data
                .ok_or_else(|| BuildError::missing_field("meta_data"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
