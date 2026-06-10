pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfUploadBinaryRequest {
    #[serde(skip_serializing)]
    #[serde(default)]
    pub body: Vec<u8>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    #[serde(skip_serializing)]
    pub format: Option<PdfUploadBinaryRequestFormat>,
    /// The desired name for the uploaded PDF file. This name will be used for storage on the server.
    ///
    ///
    /// **NOTE**: Please ensure file_name has extension `.pdf`.
    #[serde(skip_serializing)]
    #[serde(default)]
    pub file_name: String,
}

impl PdfUploadBinaryRequest {
    pub fn builder() -> PdfUploadBinaryRequestBuilder {
        <PdfUploadBinaryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfUploadBinaryRequestBuilder {
    body: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfUploadBinaryRequestFormat>,
    file_name: Option<String>,
}

impl PdfUploadBinaryRequestBuilder {
    pub fn body(mut self, value: Vec<u8>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfUploadBinaryRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfUploadBinaryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](PdfUploadBinaryRequestBuilder::body)
    /// - [`api_key`](PdfUploadBinaryRequestBuilder::api_key)
    /// - [`file_name`](PdfUploadBinaryRequestBuilder::file_name)
    pub fn build(self) -> Result<PdfUploadBinaryRequest, BuildError> {
        Ok(PdfUploadBinaryRequest {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
        })
    }
}
