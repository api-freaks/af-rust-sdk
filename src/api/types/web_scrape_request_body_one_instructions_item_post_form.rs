pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyOneInstructionsItemPostForm {
    /// Submits a form using POST method. Provide the form's XPath/CSS selector and input values.
    pub post_form: Option<WebScrapeRequestBodyOneInstructionsItemPostFormPostForm>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostForm {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemPostFormBuilder {
        <WebScrapeRequestBodyOneInstructionsItemPostFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemPostFormBuilder {
    post_form: Option<WebScrapeRequestBodyOneInstructionsItemPostFormPostForm>,
}

impl WebScrapeRequestBodyOneInstructionsItemPostFormBuilder {
    pub fn post_form(
        mut self,
        value: WebScrapeRequestBodyOneInstructionsItemPostFormPostForm,
    ) -> Self {
        self.post_form = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemPostForm`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItemPostForm, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemPostForm {
            post_form: self.post_form,
        })
    }
}
