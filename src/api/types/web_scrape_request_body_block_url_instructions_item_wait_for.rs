pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor {
    pub wait_for: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemWaitForBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemWaitForBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemWaitForBuilder {
    wait_for: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemWaitForBuilder {
    pub fn wait_for(mut self, value: impl Into<String>) -> Self {
        self.wait_for = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor {
            wait_for: self.wait_for,
        })
    }
}
