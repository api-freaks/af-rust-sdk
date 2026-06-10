pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame {
    /// Returns from iframe to parent context.
    pub switch_to_parent_frame: Option<bool>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrameBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrameBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrameBuilder {
    switch_to_parent_frame: Option<bool>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrameBuilder {
    pub fn switch_to_parent_frame(mut self, value: bool) -> Self {
        self.switch_to_parent_frame = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame {
                switch_to_parent_frame: self.switch_to_parent_frame,
            },
        )
    }
}
