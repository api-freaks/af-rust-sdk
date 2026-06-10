pub use crate::prelude::*;

/// Query parameters for get_supported_flags
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSupportedFlagsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
}

impl GetSupportedFlagsQueryRequest {
    pub fn builder() -> GetSupportedFlagsQueryRequestBuilder {
        <GetSupportedFlagsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSupportedFlagsQueryRequestBuilder {
    api_key: Option<String>,
}

impl GetSupportedFlagsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSupportedFlagsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetSupportedFlagsQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GetSupportedFlagsQueryRequest, BuildError> {
        Ok(GetSupportedFlagsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
