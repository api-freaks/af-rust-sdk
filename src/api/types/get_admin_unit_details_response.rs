pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminUnitDetailsResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub admin_code: String,
    #[serde(default)]
    pub admin_level: String,
    #[serde(rename = "iso_alpha_2")]
    #[serde(default)]
    pub iso_alpha2: String,
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
    admin_level: Option<String>,
    iso_alpha2: Option<String>,
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

    pub fn admin_level(mut self, value: impl Into<String>) -> Self {
        self.admin_level = Some(value.into());
        self
    }

    pub fn iso_alpha2(mut self, value: impl Into<String>) -> Self {
        self.iso_alpha2 = Some(value.into());
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
    /// - [`iso_alpha2`](GetAdminUnitDetailsResponseBuilder::iso_alpha2)
    /// - [`country_name`](GetAdminUnitDetailsResponseBuilder::country_name)
    pub fn build(self) -> Result<GetAdminUnitDetailsResponse, BuildError> {
        Ok(GetAdminUnitDetailsResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            admin_code: self
                .admin_code
                .ok_or_else(|| BuildError::missing_field("admin_code"))?,
            admin_level: self
                .admin_level
                .ok_or_else(|| BuildError::missing_field("admin_level"))?,
            iso_alpha2: self
                .iso_alpha2
                .ok_or_else(|| BuildError::missing_field("iso_alpha2"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
        })
    }
}
