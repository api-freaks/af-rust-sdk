pub use crate::prelude::*;

/// The API returns an empty object `{}` when no extract instruction matched anything.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeResponse {
    /// Extracted data based on the extract instructions. Absent when nothing was extracted.
    #[serde(rename = "extractedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_data: Option<HashMap<String, serde_json::Value>>,
}

impl WebScrapeResponse {
    pub fn builder() -> WebScrapeResponseBuilder {
        <WebScrapeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeResponseBuilder {
    extracted_data: Option<HashMap<String, serde_json::Value>>,
}

impl WebScrapeResponseBuilder {
    pub fn extracted_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extracted_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeResponse`].
    pub fn build(self) -> Result<WebScrapeResponse, BuildError> {
        Ok(WebScrapeResponse {
            extracted_data: self.extracted_data,
        })
    }
}
