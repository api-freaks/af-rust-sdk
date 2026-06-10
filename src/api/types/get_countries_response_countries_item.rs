pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCountriesResponseCountriesItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "iso_alpha_2")]
    #[serde(default)]
    pub iso_alpha2: String,
    #[serde(rename = "iso_alpha_3")]
    #[serde(default)]
    pub iso_alpha3: String,
    #[serde(default)]
    pub iso_numeric: i64,
    #[serde(default)]
    pub capital: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub subregion: String,
}

impl GetCountriesResponseCountriesItem {
    pub fn builder() -> GetCountriesResponseCountriesItemBuilder {
        <GetCountriesResponseCountriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCountriesResponseCountriesItemBuilder {
    name: Option<String>,
    iso_alpha2: Option<String>,
    iso_alpha3: Option<String>,
    iso_numeric: Option<i64>,
    capital: Option<String>,
    region: Option<String>,
    subregion: Option<String>,
}

impl GetCountriesResponseCountriesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn iso_alpha2(mut self, value: impl Into<String>) -> Self {
        self.iso_alpha2 = Some(value.into());
        self
    }

    pub fn iso_alpha3(mut self, value: impl Into<String>) -> Self {
        self.iso_alpha3 = Some(value.into());
        self
    }

    pub fn iso_numeric(mut self, value: i64) -> Self {
        self.iso_numeric = Some(value);
        self
    }

    pub fn capital(mut self, value: impl Into<String>) -> Self {
        self.capital = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn subregion(mut self, value: impl Into<String>) -> Self {
        self.subregion = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCountriesResponseCountriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetCountriesResponseCountriesItemBuilder::name)
    /// - [`iso_alpha2`](GetCountriesResponseCountriesItemBuilder::iso_alpha2)
    /// - [`iso_alpha3`](GetCountriesResponseCountriesItemBuilder::iso_alpha3)
    /// - [`iso_numeric`](GetCountriesResponseCountriesItemBuilder::iso_numeric)
    /// - [`capital`](GetCountriesResponseCountriesItemBuilder::capital)
    /// - [`region`](GetCountriesResponseCountriesItemBuilder::region)
    /// - [`subregion`](GetCountriesResponseCountriesItemBuilder::subregion)
    pub fn build(self) -> Result<GetCountriesResponseCountriesItem, BuildError> {
        Ok(GetCountriesResponseCountriesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            iso_alpha2: self
                .iso_alpha2
                .ok_or_else(|| BuildError::missing_field("iso_alpha2"))?,
            iso_alpha3: self
                .iso_alpha3
                .ok_or_else(|| BuildError::missing_field("iso_alpha3"))?,
            iso_numeric: self
                .iso_numeric
                .ok_or_else(|| BuildError::missing_field("iso_numeric"))?,
            capital: self
                .capital
                .ok_or_else(|| BuildError::missing_field("capital"))?,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
            subregion: self
                .subregion
                .ok_or_else(|| BuildError::missing_field("subregion"))?,
        })
    }
}
