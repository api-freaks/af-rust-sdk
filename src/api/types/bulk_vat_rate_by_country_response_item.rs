pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkVatRateByCountryResponseItem {
    /// Country identifier in Alpha-2 (US), Alpha-3 (USA), or full name (United_States). Case-insensitive and may use underscores.
    #[serde(default)]
    pub country: String,
    /// Optional state identifier in Alpha-2 (NY) or full name (New_York). Required only for state-level taxation. Case-insensitive and may use underscores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Tax type applied for the country or state. Possible values include vat or none.
    #[serde(default)]
    pub r#type: String,
    /// ISO 4217 currency code.
    #[serde(default)]
    pub currency: String,
    /// Standard VAT or sales tax rate. Zero indicates no tax.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub standard_rate: f64,
    /// Optional reduced VAT rates applicable in the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduced_rate: Option<Vec<f64>>,
    /// Optional super-reduced VAT rates applicable in specific categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_reduced_rate: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub parking_rate: Option<f64>,
    /// Optional category-wise VAT rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<HashMap<String, f64>>,
}

impl BulkVatRateByCountryResponseItem {
    pub fn builder() -> BulkVatRateByCountryResponseItemBuilder {
        <BulkVatRateByCountryResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkVatRateByCountryResponseItemBuilder {
    country: Option<String>,
    state: Option<String>,
    r#type: Option<String>,
    currency: Option<String>,
    standard_rate: Option<f64>,
    reduced_rate: Option<Vec<f64>>,
    super_reduced_rate: Option<Vec<f64>>,
    parking_rate: Option<f64>,
    categories: Option<HashMap<String, f64>>,
}

impl BulkVatRateByCountryResponseItemBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn standard_rate(mut self, value: f64) -> Self {
        self.standard_rate = Some(value);
        self
    }

    pub fn reduced_rate(mut self, value: Vec<f64>) -> Self {
        self.reduced_rate = Some(value);
        self
    }

    pub fn super_reduced_rate(mut self, value: Vec<f64>) -> Self {
        self.super_reduced_rate = Some(value);
        self
    }

    pub fn parking_rate(mut self, value: f64) -> Self {
        self.parking_rate = Some(value);
        self
    }

    pub fn categories(mut self, value: HashMap<String, f64>) -> Self {
        self.categories = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkVatRateByCountryResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](BulkVatRateByCountryResponseItemBuilder::country)
    /// - [`r#type`](BulkVatRateByCountryResponseItemBuilder::r#type)
    /// - [`currency`](BulkVatRateByCountryResponseItemBuilder::currency)
    /// - [`standard_rate`](BulkVatRateByCountryResponseItemBuilder::standard_rate)
    pub fn build(self) -> Result<BulkVatRateByCountryResponseItem, BuildError> {
        Ok(BulkVatRateByCountryResponseItem {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            state: self.state,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            standard_rate: self
                .standard_rate
                .ok_or_else(|| BuildError::missing_field("standard_rate"))?,
            reduced_rate: self.reduced_rate,
            super_reduced_rate: self.super_reduced_rate,
            parking_rate: self.parking_rate,
            categories: self.categories,
        })
    }
}
