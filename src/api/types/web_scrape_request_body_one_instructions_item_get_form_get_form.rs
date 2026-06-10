pub use crate::prelude::*;

/// Submits a form using GET method. Provide the form's XPath/CSS selector and input values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormGetForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormGetForm {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemGetFormGetFormBuilder {
        <WebScrapeRequestBodyOneInstructionsItemGetFormGetFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormGetFormBuilder {
    selector: Option<String>,
    data: Option<WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormGetFormBuilder {
    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn data(
        mut self,
        value: WebScrapeRequestBodyOneInstructionsItemGetFormGetFormData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemGetFormGetForm`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyOneInstructionsItemGetFormGetForm, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemGetFormGetForm {
            selector: self.selector,
            data: self.data,
        })
    }
}
