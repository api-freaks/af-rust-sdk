pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeResponse {
    /// Extracted data based on the extract instructions
    #[serde(rename = "extractedData")]
    #[serde(default)]
    pub extracted_data: HashMap<String, serde_json::Value>,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`extracted_data`](WebScrapeResponseBuilder::extracted_data)
    pub fn build(self) -> Result<WebScrapeResponse, BuildError> {
        Ok(WebScrapeResponse {
            extracted_data: self
                .extracted_data
                .ok_or_else(|| BuildError::missing_field("extracted_data"))?,
        })
    }
}
