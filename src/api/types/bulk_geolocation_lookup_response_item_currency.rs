pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemCurrency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl BulkGeolocationLookupResponseItemCurrency {
    pub fn builder() -> BulkGeolocationLookupResponseItemCurrencyBuilder {
        <BulkGeolocationLookupResponseItemCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemCurrencyBuilder {
    code: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
}

impl BulkGeolocationLookupResponseItemCurrencyBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemCurrency`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemCurrency, BuildError> {
        Ok(BulkGeolocationLookupResponseItemCurrency {
            code: self.code,
            name: self.name,
            symbol: self.symbol,
        })
    }
}
