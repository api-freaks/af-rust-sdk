pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkVatRateByCountryRequest {
    #[serde(default)]
    pub countries: Vec<BulkVatRateByCountryRequestCountriesItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing)]
    pub format: Option<BulkVatRateByCountryRequestFormat>,
}

impl BulkVatRateByCountryRequest {
    pub fn builder() -> BulkVatRateByCountryRequestBuilder {
        <BulkVatRateByCountryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkVatRateByCountryRequestBuilder {
    countries: Option<Vec<BulkVatRateByCountryRequestCountriesItem>>,
    api_key: Option<String>,
    format: Option<BulkVatRateByCountryRequestFormat>,
}

impl BulkVatRateByCountryRequestBuilder {
    pub fn countries(mut self, value: Vec<BulkVatRateByCountryRequestCountriesItem>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkVatRateByCountryRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkVatRateByCountryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`countries`](BulkVatRateByCountryRequestBuilder::countries)
    /// - [`api_key`](BulkVatRateByCountryRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkVatRateByCountryRequest, BuildError> {
        Ok(BulkVatRateByCountryRequest {
            countries: self
                .countries
                .ok_or_else(|| BuildError::missing_field("countries"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
