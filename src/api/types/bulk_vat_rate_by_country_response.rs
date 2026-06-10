pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkVatRateByCountryResponse {
    #[serde(default)]
    pub countries: Vec<BulkVatRateByCountryResponseCountriesItem>,
}

impl BulkVatRateByCountryResponse {
    pub fn builder() -> BulkVatRateByCountryResponseBuilder {
        <BulkVatRateByCountryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkVatRateByCountryResponseBuilder {
    countries: Option<Vec<BulkVatRateByCountryResponseCountriesItem>>,
}

impl BulkVatRateByCountryResponseBuilder {
    pub fn countries(mut self, value: Vec<BulkVatRateByCountryResponseCountriesItem>) -> Self {
        self.countries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkVatRateByCountryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`countries`](BulkVatRateByCountryResponseBuilder::countries)
    pub fn build(self) -> Result<BulkVatRateByCountryResponse, BuildError> {
        Ok(BulkVatRateByCountryResponse {
            countries: self
                .countries
                .ok_or_else(|| BuildError::missing_field("countries"))?,
        })
    }
}
