pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CurrencySupportedResponseSupportedCurrenciesMapValue {
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    pub currency_code: String,
    /// Currency name; null for some entries (e.g. crypto tokens).
    #[serde(rename = "currencyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    /// Country code; null for currencies not tied to a country.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Country name; null for currencies not tied to a country.
    #[serde(rename = "countryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    pub status: CurrencySupportedResponseSupportedCurrenciesMapValueStatus,
    #[serde(rename = "availableFrom")]
    #[serde(default)]
    pub available_from: String,
    #[serde(rename = "availableUntil")]
    #[serde(default)]
    pub available_until: String,
    #[serde(default)]
    pub icon: String,
}

impl CurrencySupportedResponseSupportedCurrenciesMapValue {
    pub fn builder() -> CurrencySupportedResponseSupportedCurrenciesMapValueBuilder {
        <CurrencySupportedResponseSupportedCurrenciesMapValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencySupportedResponseSupportedCurrenciesMapValueBuilder {
    currency_code: Option<String>,
    currency_name: Option<String>,
    country_code: Option<String>,
    country_name: Option<String>,
    status: Option<CurrencySupportedResponseSupportedCurrenciesMapValueStatus>,
    available_from: Option<String>,
    available_until: Option<String>,
    icon: Option<String>,
}

impl CurrencySupportedResponseSupportedCurrenciesMapValueBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn currency_name(mut self, value: impl Into<String>) -> Self {
        self.currency_name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: CurrencySupportedResponseSupportedCurrenciesMapValueStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn available_from(mut self, value: impl Into<String>) -> Self {
        self.available_from = Some(value.into());
        self
    }

    pub fn available_until(mut self, value: impl Into<String>) -> Self {
        self.available_until = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CurrencySupportedResponseSupportedCurrenciesMapValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](CurrencySupportedResponseSupportedCurrenciesMapValueBuilder::currency_code)
    /// - [`status`](CurrencySupportedResponseSupportedCurrenciesMapValueBuilder::status)
    /// - [`available_from`](CurrencySupportedResponseSupportedCurrenciesMapValueBuilder::available_from)
    /// - [`available_until`](CurrencySupportedResponseSupportedCurrenciesMapValueBuilder::available_until)
    /// - [`icon`](CurrencySupportedResponseSupportedCurrenciesMapValueBuilder::icon)
    pub fn build(self) -> Result<CurrencySupportedResponseSupportedCurrenciesMapValue, BuildError> {
        Ok(CurrencySupportedResponseSupportedCurrenciesMapValue {
            currency_code: self
                .currency_code
                .ok_or_else(|| BuildError::missing_field("currency_code"))?,
            currency_name: self.currency_name,
            country_code: self.country_code,
            country_name: self.country_name,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            available_from: self
                .available_from
                .ok_or_else(|| BuildError::missing_field("available_from"))?,
            available_until: self
                .available_until
                .ok_or_else(|| BuildError::missing_field("available_until"))?,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
        })
    }
}
