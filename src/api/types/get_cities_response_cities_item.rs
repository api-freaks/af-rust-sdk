pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCitiesResponseCitiesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    #[serde(default)]
    pub admin_unit: GetCitiesResponseCitiesItemAdminUnit,
    #[serde(rename = "iso_alpha_2")]
    #[serde(default)]
    pub iso_alpha2: String,
}

impl GetCitiesResponseCitiesItem {
    pub fn builder() -> GetCitiesResponseCitiesItemBuilder {
        <GetCitiesResponseCitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCitiesResponseCitiesItemBuilder {
    name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    admin_unit: Option<GetCitiesResponseCitiesItemAdminUnit>,
    iso_alpha2: Option<String>,
}

impl GetCitiesResponseCitiesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn admin_unit(mut self, value: GetCitiesResponseCitiesItemAdminUnit) -> Self {
        self.admin_unit = Some(value);
        self
    }

    pub fn iso_alpha2(mut self, value: impl Into<String>) -> Self {
        self.iso_alpha2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCitiesResponseCitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetCitiesResponseCitiesItemBuilder::name)
    /// - [`latitude`](GetCitiesResponseCitiesItemBuilder::latitude)
    /// - [`longitude`](GetCitiesResponseCitiesItemBuilder::longitude)
    /// - [`admin_unit`](GetCitiesResponseCitiesItemBuilder::admin_unit)
    /// - [`iso_alpha2`](GetCitiesResponseCitiesItemBuilder::iso_alpha2)
    pub fn build(self) -> Result<GetCitiesResponseCitiesItem, BuildError> {
        Ok(GetCitiesResponseCitiesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            admin_unit: self
                .admin_unit
                .ok_or_else(|| BuildError::missing_field("admin_unit"))?,
            iso_alpha2: self
                .iso_alpha2
                .ok_or_else(|| BuildError::missing_field("iso_alpha2"))?,
        })
    }
}
