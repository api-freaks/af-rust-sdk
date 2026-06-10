pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage {
    /// Saves image using selector or ID.
    pub saveimage: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSaveimageBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSaveimageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSaveimageBuilder {
    saveimage: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSaveimageBuilder {
    pub fn saveimage(mut self, value: impl Into<String>) -> Self {
        self.saveimage = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage {
            saveimage: self.saveimage,
        })
    }
}
