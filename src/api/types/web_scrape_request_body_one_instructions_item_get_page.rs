pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyOneInstructionsItemGetPage {
    /// Retrieves page content.
    pub get_page: Option<WebScrapeRequestBodyOneInstructionsItemGetPageGetPage>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetPage {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemGetPageBuilder {
        <WebScrapeRequestBodyOneInstructionsItemGetPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemGetPageBuilder {
    get_page: Option<WebScrapeRequestBodyOneInstructionsItemGetPageGetPage>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetPageBuilder {
    pub fn get_page(
        mut self,
        value: WebScrapeRequestBodyOneInstructionsItemGetPageGetPage,
    ) -> Self {
        self.get_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemGetPage`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItemGetPage, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemGetPage {
            get_page: self.get_page,
        })
    }
}
