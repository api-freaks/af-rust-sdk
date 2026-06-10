pub use crate::prelude::*;

/// Query parameters for zipcode_search_by_city
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ZipcodeSearchByCityQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ZipcodeSearchByCityRequestFormat>,
    /// Name of the city in which we want to find zipcodes in.
    #[serde(default)]
    pub city: String,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Name of the state or province associated with the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
    /// Page number to retrieve paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ZipcodeSearchByCityQueryRequest {
    pub fn builder() -> ZipcodeSearchByCityQueryRequestBuilder {
        <ZipcodeSearchByCityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByCityQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<ZipcodeSearchByCityRequestFormat>,
    city: Option<String>,
    country: Option<String>,
    state_name: Option<String>,
    page: Option<i64>,
}

impl ZipcodeSearchByCityQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeSearchByCityRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn state_name(mut self, value: impl Into<String>) -> Self {
        self.state_name = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByCityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ZipcodeSearchByCityQueryRequestBuilder::api_key)
    /// - [`city`](ZipcodeSearchByCityQueryRequestBuilder::city)
    /// - [`country`](ZipcodeSearchByCityQueryRequestBuilder::country)
    pub fn build(self) -> Result<ZipcodeSearchByCityQueryRequest, BuildError> {
        Ok(ZipcodeSearchByCityQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            state_name: self.state_name,
            page: self.page,
        })
    }
}
