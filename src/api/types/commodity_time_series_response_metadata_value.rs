pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityTimeSeriesResponseMetadataValue {
    /// Unit of the respective commodity.
    #[serde(default)]
    pub unit: String,
    /// Quote currency of the respective commodity.
    #[serde(default)]
    pub quote: String,
}

impl CommodityTimeSeriesResponseMetadataValue {
    pub fn builder() -> CommodityTimeSeriesResponseMetadataValueBuilder {
        <CommodityTimeSeriesResponseMetadataValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesResponseMetadataValueBuilder {
    unit: Option<String>,
    quote: Option<String>,
}

impl CommodityTimeSeriesResponseMetadataValueBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesResponseMetadataValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](CommodityTimeSeriesResponseMetadataValueBuilder::unit)
    /// - [`quote`](CommodityTimeSeriesResponseMetadataValueBuilder::quote)
    pub fn build(self) -> Result<CommodityTimeSeriesResponseMetadataValue, BuildError> {
        Ok(CommodityTimeSeriesResponseMetadataValue {
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quote: self
                .quote
                .ok_or_else(|| BuildError::missing_field("quote"))?,
        })
    }
}
