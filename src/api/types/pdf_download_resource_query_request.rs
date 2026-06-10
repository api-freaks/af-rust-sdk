pub use crate::prelude::*;

/// Query parameters for pdf_download_resource
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfDownloadResourceQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PdfDownloadResourceRequestFormat>,
    /// The unique identifier of the file or ZIP archive to download.
    #[serde(default)]
    pub resource_id: String,
}

impl PdfDownloadResourceQueryRequest {
    pub fn builder() -> PdfDownloadResourceQueryRequestBuilder {
        <PdfDownloadResourceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfDownloadResourceQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<PdfDownloadResourceRequestFormat>,
    resource_id: Option<String>,
}

impl PdfDownloadResourceQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfDownloadResourceRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfDownloadResourceQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfDownloadResourceQueryRequestBuilder::api_key)
    /// - [`resource_id`](PdfDownloadResourceQueryRequestBuilder::resource_id)
    pub fn build(self) -> Result<PdfDownloadResourceQueryRequest, BuildError> {
        Ok(PdfDownloadResourceQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
        })
    }
}
