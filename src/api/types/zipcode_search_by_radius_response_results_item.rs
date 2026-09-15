pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeSearchByRadiusResponseResultsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
}

impl ZipcodeSearchByRadiusResponseResultsItem {
    pub fn builder() -> ZipcodeSearchByRadiusResponseResultsItemBuilder {
        <ZipcodeSearchByRadiusResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByRadiusResponseResultsItemBuilder {
    code: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
    city: Option<String>,
    district: Option<String>,
    distance: Option<f64>,
}

impl ZipcodeSearchByRadiusResponseResultsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn region_code(mut self, value: impl Into<String>) -> Self {
        self.region_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByRadiusResponseResultsItem`].
    pub fn build(self) -> Result<ZipcodeSearchByRadiusResponseResultsItem, BuildError> {
        Ok(ZipcodeSearchByRadiusResponseResultsItem {
            code: self.code,
            region: self.region,
            region_code: self.region_code,
            city: self.city,
            district: self.district,
            distance: self.distance,
        })
    }
}
