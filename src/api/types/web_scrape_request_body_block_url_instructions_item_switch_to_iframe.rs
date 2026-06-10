pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe {
    /// Switches to an iframe by name or ID.
    pub switch_to_iframe: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframeBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframeBuilder {
    switch_to_iframe: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframeBuilder {
    pub fn switch_to_iframe(mut self, value: impl Into<String>) -> Self {
        self.switch_to_iframe = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe {
            switch_to_iframe: self.switch_to_iframe,
        })
    }
}
