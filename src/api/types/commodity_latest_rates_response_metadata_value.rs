pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityLatestRatesResponseMetadataValue {
    /// Unit of the respective commodity.
    #[serde(default)]
    pub unit: String,
    /// Quote currency of the respective commodity.
    #[serde(default)]
    pub quote: String,
}

impl CommodityLatestRatesResponseMetadataValue {
    pub fn builder() -> CommodityLatestRatesResponseMetadataValueBuilder {
        <CommodityLatestRatesResponseMetadataValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesResponseMetadataValueBuilder {
    unit: Option<String>,
    quote: Option<String>,
}

impl CommodityLatestRatesResponseMetadataValueBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesResponseMetadataValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](CommodityLatestRatesResponseMetadataValueBuilder::unit)
    /// - [`quote`](CommodityLatestRatesResponseMetadataValueBuilder::quote)
    pub fn build(self) -> Result<CommodityLatestRatesResponseMetadataValue, BuildError> {
        Ok(CommodityLatestRatesResponseMetadataValue {
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quote: self
                .quote
                .ok_or_else(|| BuildError::missing_field("quote"))?,
        })
    }
}
