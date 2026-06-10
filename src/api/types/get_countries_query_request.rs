pub use crate::prelude::*;

/// Query parameters for get_countries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCountriesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetCountriesRequestFormat>,
    /// Optional filter to return countries within a specific region from the region endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Optional filter to return countries within a specific subregion from the subregion endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subregion: Option<String>,
}

impl GetCountriesQueryRequest {
    pub fn builder() -> GetCountriesQueryRequestBuilder {
        <GetCountriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCountriesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetCountriesRequestFormat>,
    region: Option<String>,
    subregion: Option<String>,
}

impl GetCountriesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetCountriesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn subregion(mut self, value: impl Into<String>) -> Self {
        self.subregion = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCountriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetCountriesQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GetCountriesQueryRequest, BuildError> {
        Ok(GetCountriesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            region: self.region,
            subregion: self.subregion,
        })
    }
}
