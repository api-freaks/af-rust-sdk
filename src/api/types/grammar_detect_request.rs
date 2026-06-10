pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrammarDetectRequest {
    /// Text to analyze for grammar errors
    #[serde(default)]
    pub text: String,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
}

impl GrammarDetectRequest {
    pub fn builder() -> GrammarDetectRequestBuilder {
        <GrammarDetectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrammarDetectRequestBuilder {
    text: Option<String>,
    api_key: Option<String>,
}

impl GrammarDetectRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GrammarDetectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](GrammarDetectRequestBuilder::text)
    /// - [`api_key`](GrammarDetectRequestBuilder::api_key)
    pub fn build(self) -> Result<GrammarDetectRequest, BuildError> {
        Ok(GrammarDetectRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
