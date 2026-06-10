pub use crate::prelude::*;

/// Query parameters for get_country_details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCountryDetailsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetCountryDetailsRequestFormat>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
}

impl GetCountryDetailsQueryRequest {
    pub fn builder() -> GetCountryDetailsQueryRequestBuilder {
        <GetCountryDetailsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCountryDetailsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetCountryDetailsRequestFormat>,
    country: Option<String>,
}

impl GetCountryDetailsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetCountryDetailsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCountryDetailsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetCountryDetailsQueryRequestBuilder::api_key)
    /// - [`country`](GetCountryDetailsQueryRequestBuilder::country)
    pub fn build(self) -> Result<GetCountryDetailsQueryRequest, BuildError> {
        Ok(GetCountryDetailsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
        })
    }
}
