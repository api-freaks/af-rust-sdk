pub use crate::prelude::*;

/// Query parameters for get_regions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRegionsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetRegionsRequestFormat>,
}

impl GetRegionsQueryRequest {
    pub fn builder() -> GetRegionsQueryRequestBuilder {
        <GetRegionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRegionsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetRegionsRequestFormat>,
}

impl GetRegionsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetRegionsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRegionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetRegionsQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GetRegionsQueryRequest, BuildError> {
        Ok(GetRegionsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
