pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadabilityScoreRequest {
    /// Text to analyze for readability
    #[serde(default)]
    pub text: String,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Target audience used to tune sentence difficulty levels
    #[serde(skip_serializing)]
    pub target: Option<ReadabilityScoreRequestTarget>,
    /// Comma-separated response sections to omit. Possible values are readability_scores, sentence_readability, readability_grade
    #[serde(skip_serializing)]
    pub exclude: Option<String>,
}

impl ReadabilityScoreRequest {
    pub fn builder() -> ReadabilityScoreRequestBuilder {
        <ReadabilityScoreRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreRequestBuilder {
    text: Option<String>,
    api_key: Option<String>,
    target: Option<ReadabilityScoreRequestTarget>,
    exclude: Option<String>,
}

impl ReadabilityScoreRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn target(mut self, value: ReadabilityScoreRequestTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn exclude(mut self, value: impl Into<String>) -> Self {
        self.exclude = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ReadabilityScoreRequestBuilder::text)
    /// - [`api_key`](ReadabilityScoreRequestBuilder::api_key)
    pub fn build(self) -> Result<ReadabilityScoreRequest, BuildError> {
        Ok(ReadabilityScoreRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            target: self.target,
            exclude: self.exclude,
        })
    }
}
