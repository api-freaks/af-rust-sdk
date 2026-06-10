pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrPredictRequest {
    /// URL of the image or PDF (required if `file` not provided)
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_predict_request_url: Option<String>,
    /// OCR model to use. `mini-ocr-v1` for CAPTCHA OCR, `ocr-v1` for general OCR
    #[serde(rename = "model")]
    pub ocr_predict_request_model: OcrPredictRequestModel,
    /// Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    #[serde(rename = "page_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_predict_request_page_range: Option<String>,
    /// Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_predict_request_zone: Option<String>,
    /// Set to 1 to split output text into individual lines (default: 0)
    #[serde(rename = "new_line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_predict_request_new_line: Option<i64>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// URL of the image or PDF (required if `file` not provided)
    #[serde(skip_serializing)]
    pub url: Option<String>,
    /// OCR model to use.
    #[serde(skip_serializing)]
    pub model: OcrPredictRequestModel,
    /// Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    #[serde(skip_serializing)]
    pub page_range: Option<String>,
    /// Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    #[serde(skip_serializing)]
    pub zone: Option<String>,
    /// Set to 1 to split output text into individual lines (default: 0)
    #[serde(skip_serializing)]
    pub new_line: Option<i64>,
}

impl OcrPredictRequest {
    pub fn builder() -> OcrPredictRequestBuilder {
        <OcrPredictRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPredictRequestBuilder {
    ocr_predict_request_url: Option<String>,
    ocr_predict_request_model: Option<OcrPredictRequestModel>,
    ocr_predict_request_page_range: Option<String>,
    ocr_predict_request_zone: Option<String>,
    ocr_predict_request_new_line: Option<i64>,
    api_key: Option<String>,
    url: Option<String>,
    model: Option<OcrPredictRequestModel>,
    page_range: Option<String>,
    zone: Option<String>,
    new_line: Option<i64>,
}

impl OcrPredictRequestBuilder {
    pub fn ocr_predict_request_url(mut self, value: impl Into<String>) -> Self {
        self.ocr_predict_request_url = Some(value.into());
        self
    }

    pub fn ocr_predict_request_model(mut self, value: OcrPredictRequestModel) -> Self {
        self.ocr_predict_request_model = Some(value);
        self
    }

    pub fn ocr_predict_request_page_range(mut self, value: impl Into<String>) -> Self {
        self.ocr_predict_request_page_range = Some(value.into());
        self
    }

    pub fn ocr_predict_request_zone(mut self, value: impl Into<String>) -> Self {
        self.ocr_predict_request_zone = Some(value.into());
        self
    }

    pub fn ocr_predict_request_new_line(mut self, value: i64) -> Self {
        self.ocr_predict_request_new_line = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`OcrPredictRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ocr_predict_request_model`](OcrPredictRequestBuilder::ocr_predict_request_model)
    /// - [`api_key`](OcrPredictRequestBuilder::api_key)
    /// - [`model`](OcrPredictRequestBuilder::model)
    pub fn build(self) -> Result<OcrPredictRequest, BuildError> {
        Ok(OcrPredictRequest {
            ocr_predict_request_url: self.ocr_predict_request_url,
            ocr_predict_request_model: self
                .ocr_predict_request_model
                .ok_or_else(|| BuildError::missing_field("ocr_predict_request_model"))?,
            ocr_predict_request_page_range: self.ocr_predict_request_page_range,
            ocr_predict_request_zone: self.ocr_predict_request_zone,
            ocr_predict_request_new_line: self.ocr_predict_request_new_line,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            url: self.url,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            page_range: self.page_range,
            zone: self.zone,
            new_line: self.new_line,
        })
    }
}
