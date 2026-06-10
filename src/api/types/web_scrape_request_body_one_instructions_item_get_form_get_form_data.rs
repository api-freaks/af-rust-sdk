pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemGetFormGetFormDataBuilder {
        <WebScrapeRequestBodyOneInstructionsItemGetFormGetFormDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormGetFormDataBuilder {
    name: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormGetFormDataBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData { name: self.name })
    }
}
