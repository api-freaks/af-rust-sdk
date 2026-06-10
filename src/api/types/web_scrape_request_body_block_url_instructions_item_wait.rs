pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemWait {
    /// Wait time in milliseconds.
    pub wait: Option<i64>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemWait {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemWaitBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemWaitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemWaitBuilder {
    wait: Option<i64>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemWaitBuilder {
    pub fn wait(mut self, value: i64) -> Self {
        self.wait = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemWait`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemWait, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemWait { wait: self.wait })
    }
}
