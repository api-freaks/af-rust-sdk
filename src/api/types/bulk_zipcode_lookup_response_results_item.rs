pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkZipcodeLookupResponseResultsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
}

impl BulkZipcodeLookupResponseResultsItem {
    pub fn builder() -> BulkZipcodeLookupResponseResultsItemBuilder {
        <BulkZipcodeLookupResponseResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkZipcodeLookupResponseResultsItemBuilder {
    code: Option<String>,
    country_code: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
    city: Option<String>,
    locality: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

impl BulkZipcodeLookupResponseResultsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
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

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkZipcodeLookupResponseResultsItem`].
    pub fn build(self) -> Result<BulkZipcodeLookupResponseResultsItem, BuildError> {
        Ok(BulkZipcodeLookupResponseResultsItem {
            code: self.code,
            country_code: self.country_code,
            region: self.region,
            region_code: self.region_code,
            city: self.city,
            locality: self.locality,
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
