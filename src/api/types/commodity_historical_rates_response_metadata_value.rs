pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityHistoricalRatesResponseMetadataValue {
    /// Unit of the respective commodity.
    #[serde(default)]
    pub unit: String,
    /// Quote currency of the respective commodity.
    #[serde(default)]
    pub quote: String,
}

impl CommodityHistoricalRatesResponseMetadataValue {
    pub fn builder() -> CommodityHistoricalRatesResponseMetadataValueBuilder {
        <CommodityHistoricalRatesResponseMetadataValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesResponseMetadataValueBuilder {
    unit: Option<String>,
    quote: Option<String>,
}

impl CommodityHistoricalRatesResponseMetadataValueBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesResponseMetadataValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](CommodityHistoricalRatesResponseMetadataValueBuilder::unit)
    /// - [`quote`](CommodityHistoricalRatesResponseMetadataValueBuilder::quote)
    pub fn build(self) -> Result<CommodityHistoricalRatesResponseMetadataValue, BuildError> {
        Ok(CommodityHistoricalRatesResponseMetadataValue {
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quote: self
                .quote
                .ok_or_else(|| BuildError::missing_field("quote"))?,
        })
    }
}
