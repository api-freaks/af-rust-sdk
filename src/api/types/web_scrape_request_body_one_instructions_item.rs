pub use crate::prelude::*;

/// A single static-HTML scraping step. Exactly one of these action fields is set per instruction; every other field is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeRequestBodyOneInstructionsItem {
    /// Form selector and field values to submit via postForm or getForm.
    #[serde(rename = "postForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_form: Option<WebScrapeRequestBodyOneInstructionsItemPostForm>,
    /// Form selector and field values to submit via postForm or getForm.
    #[serde(rename = "getForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_form: Option<WebScrapeRequestBodyOneInstructionsItemGetForm>,
    /// Maps arbitrary output field names to CSS or XPath selectors. Each key becomes a field in the response's extractedData object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<HashMap<String, String>>,
    /// Get page action: retrieves the full page content. Optionally provide a URL or selector to target a specific page.
    #[serde(rename = "getPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_page: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItem {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemBuilder {
        <WebScrapeRequestBodyOneInstructionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemBuilder {
    post_form: Option<WebScrapeRequestBodyOneInstructionsItemPostForm>,
    get_form: Option<WebScrapeRequestBodyOneInstructionsItemGetForm>,
    extract: Option<HashMap<String, String>>,
    get_page: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemBuilder {
    pub fn post_form(mut self, value: WebScrapeRequestBodyOneInstructionsItemPostForm) -> Self {
        self.post_form = Some(value);
        self
    }

    pub fn get_form(mut self, value: WebScrapeRequestBodyOneInstructionsItemGetForm) -> Self {
        self.get_form = Some(value);
        self
    }

    pub fn extract(mut self, value: HashMap<String, String>) -> Self {
        self.extract = Some(value);
        self
    }

    pub fn get_page(mut self, value: impl Into<String>) -> Self {
        self.get_page = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItem`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItem, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItem {
            post_form: self.post_form,
            get_form: self.get_form,
            extract: self.extract,
            get_page: self.get_page,
        })
    }
}
