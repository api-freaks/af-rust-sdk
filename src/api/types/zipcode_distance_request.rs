pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceRequest {
    /// Comma separated list of postal / zip codes with which base point is compared w.r.t. Max 100 zip codes can be provided.
    #[serde(default)]
    pub compare: Vec<String>,
    /// Postal/Zip code to be used as the base point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Latitude coordinate for the base location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude coordinate for the base location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Supported distance units are m, km, mi, ft, yd, in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ZipcodeDistanceRequestUnit>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<ZipcodeDistanceRequestFormat>,
}

impl ZipcodeDistanceRequest {
    pub fn builder() -> ZipcodeDistanceRequestBuilder {
        <ZipcodeDistanceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceRequestBuilder {
    compare: Option<Vec<String>>,
    code: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    country: Option<String>,
    unit: Option<ZipcodeDistanceRequestUnit>,
    api_key: Option<String>,
    format: Option<ZipcodeDistanceRequestFormat>,
}

impl ZipcodeDistanceRequestBuilder {
    pub fn compare(mut self, value: Vec<String>) -> Self {
        self.compare = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn long(mut self, value: f64) -> Self {
        self.long = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn unit(mut self, value: ZipcodeDistanceRequestUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeDistanceRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`compare`](ZipcodeDistanceRequestBuilder::compare)
    /// - [`country`](ZipcodeDistanceRequestBuilder::country)
    /// - [`api_key`](ZipcodeDistanceRequestBuilder::api_key)
    pub fn build(self) -> Result<ZipcodeDistanceRequest, BuildError> {
        Ok(ZipcodeDistanceRequest {
            compare: self
                .compare
                .ok_or_else(|| BuildError::missing_field("compare"))?,
            code: self.code,
            lat: self.lat,
            long: self.long,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            unit: self.unit,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
