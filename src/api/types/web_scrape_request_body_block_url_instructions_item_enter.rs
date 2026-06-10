pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemEnter {
    pub enter: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemEnter {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemEnterBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemEnterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemEnterBuilder {
    enter: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemEnterBuilder {
    pub fn enter(mut self, value: impl Into<String>) -> Self {
        self.enter = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemEnter`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemEnter, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemEnter { enter: self.enter })
    }
}
