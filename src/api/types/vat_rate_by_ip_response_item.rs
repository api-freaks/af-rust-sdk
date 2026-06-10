pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VatRateByIpResponseItem {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub standard_rate: f64,
    #[serde(default)]
    pub reduced_rate: Vec<f64>,
    /// Optional super-reduced VAT rates applicable in specific categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_reduced_rate: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub parking_rate: Option<f64>,
    #[serde(default)]
    pub categories: HashMap<String, f64>,
}

impl VatRateByIpResponseItem {
    pub fn builder() -> VatRateByIpResponseItemBuilder {
        <VatRateByIpResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatRateByIpResponseItemBuilder {
    country: Option<String>,
    r#type: Option<String>,
    currency: Option<String>,
    standard_rate: Option<f64>,
    reduced_rate: Option<Vec<f64>>,
    super_reduced_rate: Option<Vec<f64>>,
    parking_rate: Option<f64>,
    categories: Option<HashMap<String, f64>>,
}

impl VatRateByIpResponseItemBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    /// Consumes the builder and constructs a [`VatRateByIpResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](VatRateByIpResponseItemBuilder::country)
    /// - [`r#type`](VatRateByIpResponseItemBuilder::r#type)
    /// - [`currency`](VatRateByIpResponseItemBuilder::currency)
    /// - [`standard_rate`](VatRateByIpResponseItemBuilder::standard_rate)
    /// - [`reduced_rate`](VatRateByIpResponseItemBuilder::reduced_rate)
    /// - [`categories`](VatRateByIpResponseItemBuilder::categories)
    pub fn build(self) -> Result<VatRateByIpResponseItem, BuildError> {
        Ok(VatRateByIpResponseItem {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            standard_rate: self
                .standard_rate
                .ok_or_else(|| BuildError::missing_field("standard_rate"))?,
            reduced_rate: self
                .reduced_rate
                .ok_or_else(|| BuildError::missing_field("reduced_rate"))?,
            super_reduced_rate: self.super_reduced_rate,
            parking_rate: self.parking_rate,
            categories: self
                .categories
                .ok_or_else(|| BuildError::missing_field("categories"))?,
        })
    }
}
