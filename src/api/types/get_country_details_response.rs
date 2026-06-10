pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCountryDetailsResponse {
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
    pub phone_code: i64,
    #[serde(default)]
    pub capital: String,
    #[serde(default)]
    pub top_level_domain: String,
    #[serde(default)]
    pub native_name: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub subregion: String,
    #[serde(default)]
    pub nationality: String,
    #[serde(default)]
    pub flag_emoji: String,
    #[serde(default)]
    pub currency_code: String,
    #[serde(default)]
    pub currency_name: String,
    #[serde(default)]
    pub currency_symbol: String,
}

impl GetCountryDetailsResponse {
    pub fn builder() -> GetCountryDetailsResponseBuilder {
        <GetCountryDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCountryDetailsResponseBuilder {
    name: Option<String>,
    iso_alpha2: Option<String>,
    iso_alpha3: Option<String>,
    iso_numeric: Option<i64>,
    phone_code: Option<i64>,
    capital: Option<String>,
    top_level_domain: Option<String>,
    native_name: Option<String>,
    region: Option<String>,
    subregion: Option<String>,
    nationality: Option<String>,
    flag_emoji: Option<String>,
    currency_code: Option<String>,
    currency_name: Option<String>,
    currency_symbol: Option<String>,
}

impl GetCountryDetailsResponseBuilder {
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

    pub fn phone_code(mut self, value: i64) -> Self {
        self.phone_code = Some(value);
        self
    }

    pub fn capital(mut self, value: impl Into<String>) -> Self {
        self.capital = Some(value.into());
        self
    }

    pub fn top_level_domain(mut self, value: impl Into<String>) -> Self {
        self.top_level_domain = Some(value.into());
        self
    }

    pub fn native_name(mut self, value: impl Into<String>) -> Self {
        self.native_name = Some(value.into());
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

    pub fn nationality(mut self, value: impl Into<String>) -> Self {
        self.nationality = Some(value.into());
        self
    }

    pub fn flag_emoji(mut self, value: impl Into<String>) -> Self {
        self.flag_emoji = Some(value.into());
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn currency_name(mut self, value: impl Into<String>) -> Self {
        self.currency_name = Some(value.into());
        self
    }

    pub fn currency_symbol(mut self, value: impl Into<String>) -> Self {
        self.currency_symbol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCountryDetailsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetCountryDetailsResponseBuilder::name)
    /// - [`iso_alpha2`](GetCountryDetailsResponseBuilder::iso_alpha2)
    /// - [`iso_alpha3`](GetCountryDetailsResponseBuilder::iso_alpha3)
    /// - [`iso_numeric`](GetCountryDetailsResponseBuilder::iso_numeric)
    /// - [`phone_code`](GetCountryDetailsResponseBuilder::phone_code)
    /// - [`capital`](GetCountryDetailsResponseBuilder::capital)
    /// - [`top_level_domain`](GetCountryDetailsResponseBuilder::top_level_domain)
    /// - [`native_name`](GetCountryDetailsResponseBuilder::native_name)
    /// - [`region`](GetCountryDetailsResponseBuilder::region)
    /// - [`subregion`](GetCountryDetailsResponseBuilder::subregion)
    /// - [`nationality`](GetCountryDetailsResponseBuilder::nationality)
    /// - [`flag_emoji`](GetCountryDetailsResponseBuilder::flag_emoji)
    /// - [`currency_code`](GetCountryDetailsResponseBuilder::currency_code)
    /// - [`currency_name`](GetCountryDetailsResponseBuilder::currency_name)
    /// - [`currency_symbol`](GetCountryDetailsResponseBuilder::currency_symbol)
    pub fn build(self) -> Result<GetCountryDetailsResponse, BuildError> {
        Ok(GetCountryDetailsResponse {
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
            phone_code: self
                .phone_code
                .ok_or_else(|| BuildError::missing_field("phone_code"))?,
            capital: self
                .capital
                .ok_or_else(|| BuildError::missing_field("capital"))?,
            top_level_domain: self
                .top_level_domain
                .ok_or_else(|| BuildError::missing_field("top_level_domain"))?,
            native_name: self
                .native_name
                .ok_or_else(|| BuildError::missing_field("native_name"))?,
            region: self
                .region
                .ok_or_else(|| BuildError::missing_field("region"))?,
            subregion: self
                .subregion
                .ok_or_else(|| BuildError::missing_field("subregion"))?,
            nationality: self
                .nationality
                .ok_or_else(|| BuildError::missing_field("nationality"))?,
            flag_emoji: self
                .flag_emoji
                .ok_or_else(|| BuildError::missing_field("flag_emoji"))?,
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            currency_name: self
                .currency_name
                .ok_or_else(|| BuildError::missing_field("currency_name"))?,
            currency_symbol: self
                .currency_symbol
                .ok_or_else(|| BuildError::missing_field("currency_symbol"))?,
        })
    }
}
