pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClick {
    pub click: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClick {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemClickBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemClickBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickBuilder {
    click: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickBuilder {
    pub fn click(mut self, value: impl Into<String>) -> Self {
        self.click = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemClick`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemClick, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemClick { click: self.click })
    }
}
