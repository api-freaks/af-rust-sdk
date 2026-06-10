pub use crate::prelude::*;

/// Query parameters for get_cities
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCitiesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetCitiesRequestFormat>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Administrative unit code used to filter cities within a specific region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_unit: Option<String>,
}

impl GetCitiesQueryRequest {
    pub fn builder() -> GetCitiesQueryRequestBuilder {
        <GetCitiesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCitiesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetCitiesRequestFormat>,
    country: Option<String>,
    admin_unit: Option<String>,
}

impl GetCitiesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetCitiesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn admin_unit(mut self, value: impl Into<String>) -> Self {
        self.admin_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCitiesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetCitiesQueryRequestBuilder::api_key)
    /// - [`country`](GetCitiesQueryRequestBuilder::country)
    pub fn build(self) -> Result<GetCitiesQueryRequest, BuildError> {
        Ok(GetCitiesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            admin_unit: self.admin_unit,
        })
    }
}
