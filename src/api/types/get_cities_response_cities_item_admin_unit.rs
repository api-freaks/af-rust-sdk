pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCitiesResponseCitiesItemAdminUnit {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub admin_code: String,
    #[serde(default)]
    pub admin_level: String,
}

impl GetCitiesResponseCitiesItemAdminUnit {
    pub fn builder() -> GetCitiesResponseCitiesItemAdminUnitBuilder {
        <GetCitiesResponseCitiesItemAdminUnitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCitiesResponseCitiesItemAdminUnitBuilder {
    name: Option<String>,
    admin_code: Option<String>,
    admin_level: Option<String>,
}

impl GetCitiesResponseCitiesItemAdminUnitBuilder {
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

    /// Consumes the builder and constructs a [`GetCitiesResponseCitiesItemAdminUnit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetCitiesResponseCitiesItemAdminUnitBuilder::name)
    /// - [`admin_code`](GetCitiesResponseCitiesItemAdminUnitBuilder::admin_code)
    /// - [`admin_level`](GetCitiesResponseCitiesItemAdminUnitBuilder::admin_level)
    pub fn build(self) -> Result<GetCitiesResponseCitiesItemAdminUnit, BuildError> {
        Ok(GetCitiesResponseCitiesItemAdminUnit {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            admin_code: self
                .admin_code
                .ok_or_else(|| BuildError::missing_field("admin_code"))?,
            admin_level: self
                .admin_level
                .ok_or_else(|| BuildError::missing_field("admin_level"))?,
        })
    }
}
