pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist {
    pub click_if_exist: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExistBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExistBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExistBuilder {
    click_if_exist: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExistBuilder {
    pub fn click_if_exist(mut self, value: impl Into<String>) -> Self {
        self.click_if_exist = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist {
            click_if_exist: self.click_if_exist,
        })
    }
}
