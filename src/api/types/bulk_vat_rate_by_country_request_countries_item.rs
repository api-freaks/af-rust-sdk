pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkVatRateByCountryRequestCountriesItem {
    /// Country identifier in Alpha-2 (US), Alpha-3 (USA), or full name (United_States). Pair with "state" for regional lookup; values are case-insensitive and may use underscores.
    #[serde(default)]
    pub country: String,
    /// Optional state identifier in Alpha-2 (NY) or full name (New_York). Use with the country field for state-level VAT; values are case-insensitive and may use underscores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl BulkVatRateByCountryRequestCountriesItem {
    pub fn builder() -> BulkVatRateByCountryRequestCountriesItemBuilder {
        <BulkVatRateByCountryRequestCountriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkVatRateByCountryRequestCountriesItemBuilder {
    country: Option<String>,
    state: Option<String>,
}

impl BulkVatRateByCountryRequestCountriesItemBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkVatRateByCountryRequestCountriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](BulkVatRateByCountryRequestCountriesItemBuilder::country)
    pub fn build(self) -> Result<BulkVatRateByCountryRequestCountriesItem, BuildError> {
        Ok(BulkVatRateByCountryRequestCountriesItem {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            state: self.state,
        })
    }
}
