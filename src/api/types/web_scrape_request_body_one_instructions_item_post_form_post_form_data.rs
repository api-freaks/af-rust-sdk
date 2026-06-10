pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemPostFormPostFormDataBuilder {
        <WebScrapeRequestBodyOneInstructionsItemPostFormPostFormDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemPostFormPostFormDataBuilder {
    name: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostFormPostFormDataBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData { name: self.name })
    }
}
