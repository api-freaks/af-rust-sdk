pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceResponseResultsItem {
    /// ZIP/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Distance from the base ZIP/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
}

impl ZipcodeDistanceResponseResultsItem {
    pub fn builder() -> ZipcodeDistanceResponseResultsItemBuilder {
        <ZipcodeDistanceResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceResponseResultsItemBuilder {
    code: Option<String>,
    distance: Option<f64>,
}

impl ZipcodeDistanceResponseResultsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceResponseResultsItem`].
    pub fn build(self) -> Result<ZipcodeDistanceResponseResultsItem, BuildError> {
        Ok(ZipcodeDistanceResponseResultsItem {
            code: self.code,
            distance: self.distance,
        })
    }
}
