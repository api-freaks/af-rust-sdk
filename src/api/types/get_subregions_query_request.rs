pub use crate::prelude::*;

/// Query parameters for get_subregions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSubregionsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetSubregionsRequestFormat>,
    /// Name of the region.
    #[serde(default)]
    pub region: String,
}

impl GetSubregionsQueryRequest {
    pub fn builder() -> GetSubregionsQueryRequestBuilder {
        <GetSubregionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubregionsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetSubregionsRequestFormat>,
    region: Option<String>,
}

impl GetSubregionsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetSubregionsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSubregionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetSubregionsQueryRequestBuilder::api_key)
    /// - [`region`](GetSubregionsQueryRequestBuilder::region)
    pub fn build(self) -> Result<GetSubregionsQueryRequest, BuildError> {
        Ok(GetSubregionsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
        })
    }
}
