pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFill {
    pub fill: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFillFill>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFill {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemFillBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemFillBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillBuilder {
    fill: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFillFill>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillBuilder {
    pub fn fill(mut self, value: WebScrapeRequestBodyBlockUrlInstructionsItemFillFill) -> Self {
        self.fill = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemFill`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemFill, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemFill { fill: self.fill })
    }
}
