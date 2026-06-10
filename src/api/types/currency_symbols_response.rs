pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencySymbolsResponse {
    /// A map of all currency symbols keyed by currency code; value is the full currency name.
    #[serde(rename = "currencySymbols")]
    #[serde(default)]
    pub currency_symbols: HashMap<String, String>,
}

impl CurrencySymbolsResponse {
    pub fn builder() -> CurrencySymbolsResponseBuilder {
        <CurrencySymbolsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencySymbolsResponseBuilder {
    currency_symbols: Option<HashMap<String, String>>,
}

impl CurrencySymbolsResponseBuilder {
    pub fn currency_symbols(mut self, value: HashMap<String, String>) -> Self {
        self.currency_symbols = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencySymbolsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_symbols`](CurrencySymbolsResponseBuilder::currency_symbols)
    pub fn build(self) -> Result<CurrencySymbolsResponse, BuildError> {
        Ok(CurrencySymbolsResponse {
            currency_symbols: self
                .currency_symbols
                .ok_or_else(|| BuildError::missing_field("currency_symbols"))?,
        })
    }
}
