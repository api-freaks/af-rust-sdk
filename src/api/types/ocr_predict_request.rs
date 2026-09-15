pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrPredictRequest {
    /// URL of the image or PDF (required if `file` not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// OCR model to use. `mini-ocr-v1` for CAPTCHA OCR, `ocr-v1` for general OCR
    pub model: OcrPredictRequestModel,
    /// Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_range: Option<String>,
    /// Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// Set to 1 to split output text into individual lines (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_line: Option<i64>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
}

impl OcrPredictRequest {
    pub fn builder() -> OcrPredictRequestBuilder {
        <OcrPredictRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPredictRequestBuilder {
    url: Option<String>,
    model: Option<OcrPredictRequestModel>,
    page_range: Option<String>,
    zone: Option<String>,
    new_line: Option<i64>,
    api_key: Option<String>,
}

impl OcrPredictRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn model(mut self, value: OcrPredictRequestModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn page_range(mut self, value: impl Into<String>) -> Self {
        self.page_range = Some(value.into());
        self
    }

    pub fn zone(mut self, value: impl Into<String>) -> Self {
        self.zone = Some(value.into());
        self
    }

    pub fn new_line(mut self, value: i64) -> Self {
        self.new_line = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OcrPredictRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](OcrPredictRequestBuilder::model)
    /// - [`api_key`](OcrPredictRequestBuilder::api_key)
    pub fn build(self) -> Result<OcrPredictRequest, BuildError> {
        Ok(OcrPredictRequest {
            url: self.url,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            page_range: self.page_range,
            zone: self.zone,
            new_line: self.new_line,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
