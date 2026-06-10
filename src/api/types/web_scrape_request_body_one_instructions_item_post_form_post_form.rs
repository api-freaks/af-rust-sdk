pub use crate::prelude::*;

/// Submits a form using POST method. Provide the form's XPath/CSS selector and input values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyOneInstructionsItemPostFormPostForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostFormPostForm {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemPostFormPostFormBuilder {
        <WebScrapeRequestBodyOneInstructionsItemPostFormPostFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemPostFormPostFormBuilder {
    selector: Option<String>,
    data: Option<WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostFormPostFormBuilder {
    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn data(
        mut self,
        value: WebScrapeRequestBodyOneInstructionsItemPostFormPostFormData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemPostFormPostForm`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyOneInstructionsItemPostFormPostForm, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemPostFormPostForm {
            selector: self.selector,
            data: self.data,
        })
    }
}
