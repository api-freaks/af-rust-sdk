pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyOneInstructionsItemGetForm {
    /// Submits a form using GET method. Provide the form's XPath/CSS selector and input values.
    pub get_form: Option<WebScrapeRequestBodyOneInstructionsItemGetFormGetForm>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetForm {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
        <WebScrapeRequestBodyOneInstructionsItemGetFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
    get_form: Option<WebScrapeRequestBodyOneInstructionsItemGetFormGetForm>,
}

impl WebScrapeRequestBodyOneInstructionsItemGetFormBuilder {
    pub fn get_form(
        mut self,
        value: WebScrapeRequestBodyOneInstructionsItemGetFormGetForm,
    ) -> Self {
        self.get_form = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemGetForm`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItemGetForm, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemGetForm {
            get_form: self.get_form,
        })
    }
}
