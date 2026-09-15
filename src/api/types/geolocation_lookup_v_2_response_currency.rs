pub use crate::prelude::*;

/// Currency information for the IP's country.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseCurrency {
    /// Currency code (ISO 4217).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Currency name (ISO 4217).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Currency symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl GeolocationLookupV2ResponseCurrency {
    pub fn builder() -> GeolocationLookupV2ResponseCurrencyBuilder {
        <GeolocationLookupV2ResponseCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseCurrencyBuilder {
    code: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
}

impl GeolocationLookupV2ResponseCurrencyBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseCurrency`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseCurrency, BuildError> {
        Ok(GeolocationLookupV2ResponseCurrency {
            code: self.code,
            name: self.name,
            symbol: self.symbol,
        })
    }
}
