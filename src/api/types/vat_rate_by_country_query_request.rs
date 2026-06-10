pub use crate::prelude::*;

/// Query parameters for vat_rate_by_country
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatRateByCountryQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VatRateByCountryRequestFormat>,
    /// Country identifier in Alpha-2 (PK), Alpha-3 (PAK), or full name (Pakistan). Combine with the optional "state" query for sub-national VAT; values are case-insensitive and may use underscores instead of spaces.
    #[serde(default)]
    pub country: String,
    /// Optional state or region in Alpha-2 (NY) or full name (New_York). Use with "country" for state-level VAT; values are case-insensitive and may use underscores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl VatRateByCountryQueryRequest {
    pub fn builder() -> VatRateByCountryQueryRequestBuilder {
        <VatRateByCountryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatRateByCountryQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<VatRateByCountryRequestFormat>,
    country: Option<String>,
    state: Option<String>,
}

impl VatRateByCountryQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: VatRateByCountryRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatRateByCountryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](VatRateByCountryQueryRequestBuilder::api_key)
    /// - [`country`](VatRateByCountryQueryRequestBuilder::country)
    pub fn build(self) -> Result<VatRateByCountryQueryRequest, BuildError> {
        Ok(VatRateByCountryQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            state: self.state,
        })
    }
}
