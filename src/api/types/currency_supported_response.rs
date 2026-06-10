pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencySupportedResponse {
    /// A map of all supported currencies, keyed by currency code.
    #[serde(rename = "supportedCurrenciesMap")]
    #[serde(default)]
    pub supported_currencies_map:
        HashMap<String, CurrencySupportedResponseSupportedCurrenciesMapValue>,
}

impl CurrencySupportedResponse {
    pub fn builder() -> CurrencySupportedResponseBuilder {
        <CurrencySupportedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencySupportedResponseBuilder {
    supported_currencies_map:
        Option<HashMap<String, CurrencySupportedResponseSupportedCurrenciesMapValue>>,
}

impl CurrencySupportedResponseBuilder {
    pub fn supported_currencies_map(
        mut self,
        value: HashMap<String, CurrencySupportedResponseSupportedCurrenciesMapValue>,
    ) -> Self {
        self.supported_currencies_map = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencySupportedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supported_currencies_map`](CurrencySupportedResponseBuilder::supported_currencies_map)
    pub fn build(self) -> Result<CurrencySupportedResponse, BuildError> {
        Ok(CurrencySupportedResponse {
            supported_currencies_map: self
                .supported_currencies_map
                .ok_or_else(|| BuildError::missing_field("supported_currencies_map"))?,
        })
    }
}
