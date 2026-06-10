pub use crate::prelude::*;

/// Query parameters for get_admin_units
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminUnitsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetAdminUnitsRequestFormat>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Comma-separated list to filter results by one or more administrative levels.
    #[serde(rename = "adminLevels")]
    #[serde(default)]
    pub admin_levels: Vec<Option<String>>,
}

impl GetAdminUnitsQueryRequest {
    pub fn builder() -> GetAdminUnitsQueryRequestBuilder {
        <GetAdminUnitsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminUnitsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetAdminUnitsRequestFormat>,
    country: Option<String>,
    admin_levels: Option<Vec<Option<String>>>,
}

impl GetAdminUnitsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetAdminUnitsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn admin_levels(mut self, value: Vec<Option<String>>) -> Self {
        self.admin_levels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAdminUnitsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetAdminUnitsQueryRequestBuilder::api_key)
    /// - [`country`](GetAdminUnitsQueryRequestBuilder::country)
    /// - [`admin_levels`](GetAdminUnitsQueryRequestBuilder::admin_levels)
    pub fn build(self) -> Result<GetAdminUnitsQueryRequest, BuildError> {
        Ok(GetAdminUnitsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            admin_levels: self
                .admin_levels
                .ok_or_else(|| BuildError::missing_field("admin_levels"))?,
        })
    }
}
