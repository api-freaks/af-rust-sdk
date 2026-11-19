pub use crate::prelude::*;

/// Detailed information for a single administrative unit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminUnitDetailsResponse {
    /// Name of the administrative unit.
    #[serde(default)]
    pub name: String,
    /// Administrative code of the unit.
    #[serde(default)]
    pub admin_code: String,
    /// The unit's full ISO 3166-2 code, formed as {country}-{admin_code} (e.g. US-CA, DE-BY).
    #[serde(rename = "admin_iso3166_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_iso31662: Option<String>,
    /// Level or type of the administrative unit (e.g. land, province, state).
    #[serde(default)]
    pub admin_level: String,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(rename = "country_iso3166_2")]
    #[serde(default)]
    pub country_iso31662: String,
    /// Name of the country.
    #[serde(default)]
    pub country_name: String,
}

impl GetAdminUnitDetailsResponse {
    pub fn builder() -> GetAdminUnitDetailsResponseBuilder {
        <GetAdminUnitDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminUnitDetailsResponseBuilder {
    name: Option<String>,
    admin_code: Option<String>,
    admin_iso31662: Option<String>,
    admin_level: Option<String>,
    country_iso31662: Option<String>,
    country_name: Option<String>,
}

impl GetAdminUnitDetailsResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn admin_code(mut self, value: impl Into<String>) -> Self {
        self.admin_code = Some(value.into());
        self
    }

    pub fn admin_iso31662(mut self, value: impl Into<String>) -> Self {
        self.admin_iso31662 = Some(value.into());
        self
    }

    pub fn admin_level(mut self, value: impl Into<String>) -> Self {
        self.admin_level = Some(value.into());
        self
    }

    pub fn country_iso31662(mut self, value: impl Into<String>) -> Self {
        self.country_iso31662 = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAdminUnitDetailsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetAdminUnitDetailsResponseBuilder::name)
    /// - [`admin_code`](GetAdminUnitDetailsResponseBuilder::admin_code)
    /// - [`admin_level`](GetAdminUnitDetailsResponseBuilder::admin_level)
    /// - [`country_iso31662`](GetAdminUnitDetailsResponseBuilder::country_iso31662)
    /// - [`country_name`](GetAdminUnitDetailsResponseBuilder::country_name)
    pub fn build(self) -> Result<GetAdminUnitDetailsResponse, BuildError> {
        Ok(GetAdminUnitDetailsResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            admin_code: self
                .admin_code
                .ok_or_else(|| BuildError::missing_field("admin_code"))?,
            admin_iso31662: self.admin_iso31662,
            admin_level: self
                .admin_level
                .ok_or_else(|| BuildError::missing_field("admin_level"))?,
            country_iso31662: self
                .country_iso31662
                .ok_or_else(|| BuildError::missing_field("country_iso31662"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
        })
    }
}
