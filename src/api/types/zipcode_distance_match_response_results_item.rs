pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceMatchResponseResultsItem {
    /// First ZIP/postal code in the pair
    #[serde(rename = "code_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code1: Option<String>,
    /// Second ZIP/postal code in the pair
    #[serde(rename = "code_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code2: Option<String>,
    /// Distance between the ZIP/postal code pair
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
}

impl ZipcodeDistanceMatchResponseResultsItem {
    pub fn builder() -> ZipcodeDistanceMatchResponseResultsItemBuilder {
        <ZipcodeDistanceMatchResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceMatchResponseResultsItemBuilder {
    code1: Option<String>,
    code2: Option<String>,
    distance: Option<f64>,
}

impl ZipcodeDistanceMatchResponseResultsItemBuilder {
    pub fn code1(mut self, value: impl Into<String>) -> Self {
        self.code1 = Some(value.into());
        self
    }

    pub fn code2(mut self, value: impl Into<String>) -> Self {
        self.code2 = Some(value.into());
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceMatchResponseResultsItem`].
    pub fn build(self) -> Result<ZipcodeDistanceMatchResponseResultsItem, BuildError> {
        Ok(ZipcodeDistanceMatchResponseResultsItem {
            code1: self.code1,
            code2: self.code2,
            distance: self.distance,
        })
    }
}
