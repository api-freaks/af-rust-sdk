pub use crate::prelude::*;

/// Query parameters for pdf_delete_file
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfDeleteFileQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PdfDeleteFileRequestFormat>,
    /// The unique ID of the file to be deleted.
    #[serde(default)]
    pub file_id: String,
}

impl PdfDeleteFileQueryRequest {
    pub fn builder() -> PdfDeleteFileQueryRequestBuilder {
        <PdfDeleteFileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfDeleteFileQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<PdfDeleteFileRequestFormat>,
    file_id: Option<String>,
}

impl PdfDeleteFileQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfDeleteFileRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfDeleteFileQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfDeleteFileQueryRequestBuilder::api_key)
    /// - [`file_id`](PdfDeleteFileQueryRequestBuilder::file_id)
    pub fn build(self) -> Result<PdfDeleteFileQueryRequest, BuildError> {
        Ok(PdfDeleteFileQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
