pub use crate::prelude::*;

/// Query parameters for get_admin_unit_details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminUnitDetailsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetAdminUnitDetailsRequestFormat>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Optional admin code to fetch details for a specific administrative unit.
    #[serde(default)]
    pub admin_unit: String,
}

impl GetAdminUnitDetailsQueryRequest {
    pub fn builder() -> GetAdminUnitDetailsQueryRequestBuilder {
        <GetAdminUnitDetailsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminUnitDetailsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GetAdminUnitDetailsRequestFormat>,
    country: Option<String>,
    admin_unit: Option<String>,
}

impl GetAdminUnitDetailsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GetAdminUnitDetailsRequestFormat) -> Self {
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

    /// Consumes the builder and constructs a [`GetAdminUnitDetailsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetAdminUnitDetailsQueryRequestBuilder::api_key)
    /// - [`country`](GetAdminUnitDetailsQueryRequestBuilder::country)
    /// - [`admin_unit`](GetAdminUnitDetailsQueryRequestBuilder::admin_unit)
    pub fn build(self) -> Result<GetAdminUnitDetailsQueryRequest, BuildError> {
        Ok(GetAdminUnitDetailsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            admin_unit: self
                .admin_unit
                .ok_or_else(|| BuildError::missing_field("admin_unit"))?,
        })
    }
}
