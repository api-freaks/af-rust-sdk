pub use crate::prelude::*;

/// Query parameters for get_credits_usage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCreditsUsageQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response. Possible values: json, xml.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetCreditsUsageRequestFormat>,
}

impl GetCreditsUsageQueryRequest {
    pub fn builder() -> GetCreditsUsageQueryRequestBuilder {
        <GetCreditsUsageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCreditsUsageQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetCreditsUsageRequestFormat>,
}

impl GetCreditsUsageQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetCreditsUsageRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCreditsUsageQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetCreditsUsageQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GetCreditsUsageQueryRequest, BuildError> {
        Ok(GetCreditsUsageQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
