pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WeakWordsDetectRequest {
    /// Text to analyze for weak words
    #[serde(default)]
    pub text: String,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
}

impl WeakWordsDetectRequest {
    pub fn builder() -> WeakWordsDetectRequestBuilder {
        <WeakWordsDetectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeakWordsDetectRequestBuilder {
    text: Option<String>,
    api_key: Option<String>,
}

impl WeakWordsDetectRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WeakWordsDetectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](WeakWordsDetectRequestBuilder::text)
    /// - [`api_key`](WeakWordsDetectRequestBuilder::api_key)
    pub fn build(self) -> Result<WeakWordsDetectRequest, BuildError> {
        Ok(WeakWordsDetectRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
