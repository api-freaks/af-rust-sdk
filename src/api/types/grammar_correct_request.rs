pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrammarCorrectRequest {
    /// Text to correct
    #[serde(default)]
    pub text: String,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
}

impl GrammarCorrectRequest {
    pub fn builder() -> GrammarCorrectRequestBuilder {
        <GrammarCorrectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrammarCorrectRequestBuilder {
    text: Option<String>,
    api_key: Option<String>,
}

impl GrammarCorrectRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GrammarCorrectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](GrammarCorrectRequestBuilder::text)
    /// - [`api_key`](GrammarCorrectRequestBuilder::api_key)
    pub fn build(self) -> Result<GrammarCorrectRequest, BuildError> {
        Ok(GrammarCorrectRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
