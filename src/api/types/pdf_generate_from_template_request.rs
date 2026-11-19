pub use crate::prelude::*;

/// Request for pdf_generate_from_template (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PdfGenerateFromTemplateRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    /// The template to render, from the APIFreaks PDF Template Builder. Copy it from your dashboard when you save the template.
    #[serde(skip)]
    #[serde(default)]
    pub template_id: String,
    /// Template version to render, e.g. v2. Must start with v. Omit this parameter entirely to use the latest version; the literal value 'latest' is not accepted by the API.
    #[serde(skip)]
    pub version: Option<String>,
    /// A URL the API fetches your JSON data from, instead of the request body. Provide this when you are not sending a request body.
    #[serde(skip)]
    pub data_url: Option<String>,
    /// Free-form JSON object whose keys map to the fields bound in the template.
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
}

impl PdfGenerateFromTemplateRequest {
    pub fn builder() -> PdfGenerateFromTemplateRequestBuilder {
        <PdfGenerateFromTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfGenerateFromTemplateRequestBuilder {
    api_key: Option<String>,
    template_id: Option<String>,
    version: Option<String>,
    data_url: Option<String>,
    body: Option<HashMap<String, serde_json::Value>>,
}

impl PdfGenerateFromTemplateRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn data_url(mut self, value: impl Into<String>) -> Self {
        self.data_url = Some(value.into());
        self
    }

    pub fn body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfGenerateFromTemplateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfGenerateFromTemplateRequestBuilder::api_key)
    /// - [`template_id`](PdfGenerateFromTemplateRequestBuilder::template_id)
    /// - [`body`](PdfGenerateFromTemplateRequestBuilder::body)
    pub fn build(self) -> Result<PdfGenerateFromTemplateRequest, BuildError> {
        Ok(PdfGenerateFromTemplateRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            version: self.version,
            data_url: self.data_url,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
