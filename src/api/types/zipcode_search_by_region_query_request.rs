pub use crate::prelude::*;

/// Query parameters for zipcode_search_by_region
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ZipcodeSearchByRegionQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ZipcodeSearchByRegionRequestFormat>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Name of the region, state or province associated with the country.
    #[serde(default)]
    pub region: String,
    /// Page no. to retrieve paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ZipcodeSearchByRegionQueryRequest {
    pub fn builder() -> ZipcodeSearchByRegionQueryRequestBuilder {
        <ZipcodeSearchByRegionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByRegionQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<ZipcodeSearchByRegionRequestFormat>,
    country: Option<String>,
    region: Option<String>,
    page: Option<i64>,
}

impl ZipcodeSearchByRegionQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeSearchByRegionRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByRegionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ZipcodeSearchByRegionQueryRequestBuilder::api_key)
    /// - [`country`](ZipcodeSearchByRegionQueryRequestBuilder::country)
    /// - [`region`](ZipcodeSearchByRegionQueryRequestBuilder::region)
    pub fn build(self) -> Result<ZipcodeSearchByRegionQueryRequest, BuildError> {
        Ok(ZipcodeSearchByRegionQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
            page: self.page,
        })
    }
}
