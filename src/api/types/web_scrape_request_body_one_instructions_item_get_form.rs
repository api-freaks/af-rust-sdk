pub use crate::prelude::*;

/// Form selector and field values to submit via postForm or getForm.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeRequestBodyOneInstructionsItemGetForm {
    /// CSS or XPath selector identifying the form element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Form field name/value pairs to submit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetForm {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
        <WebScrapeRequestBodyOneInstructionsItemGetFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
    selector: Option<String>,
    data: Option<HashMap<String, String>>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn data(mut self, value: HashMap<String, String>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemGetForm`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItemGetForm, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemGetForm {
            selector: self.selector,
            data: self.data,
        })
    }
}
