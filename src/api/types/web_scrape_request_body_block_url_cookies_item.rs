pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlCookiesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlCookiesItem {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlCookiesItemBuilder {
        <WebScrapeRequestBodyBlockUrlCookiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlCookiesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlCookiesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlCookiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WebScrapeRequestBodyBlockUrlCookiesItemBuilder::name)
    /// - [`value`](WebScrapeRequestBodyBlockUrlCookiesItemBuilder::value)
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlCookiesItem, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlCookiesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
