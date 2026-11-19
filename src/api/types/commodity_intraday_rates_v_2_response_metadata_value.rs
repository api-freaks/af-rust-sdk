pub use crate::prelude::*;

/// Unit and currency of the prices for one commodity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityIntradayRatesV2ResponseMetadataValue {
    /// Unit the commodity is priced in (for example `T.oz` or `Bbl`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Currency the prices are quoted in (for example `USD`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
}

impl CommodityIntradayRatesV2ResponseMetadataValue {
    pub fn builder() -> CommodityIntradayRatesV2ResponseMetadataValueBuilder {
        <CommodityIntradayRatesV2ResponseMetadataValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityIntradayRatesV2ResponseMetadataValueBuilder {
    unit: Option<String>,
    quote: Option<String>,
}

impl CommodityIntradayRatesV2ResponseMetadataValueBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityIntradayRatesV2ResponseMetadataValue`].
    pub fn build(self) -> Result<CommodityIntradayRatesV2ResponseMetadataValue, BuildError> {
        Ok(CommodityIntradayRatesV2ResponseMetadataValue {
            unit: self.unit,
            quote: self.quote,
        })
    }
}
