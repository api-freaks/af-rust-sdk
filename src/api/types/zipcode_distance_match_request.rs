pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceMatchRequest {
    /// Comma-separated list of postal/zip codes. Maximum 100 values allowed.
    #[serde(default)]
    pub codes: Vec<String>,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub country: String,
    /// Maximum allowed distance between postal code pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
    /// Supported distance units are m, km, mi, ft, yd, in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ZipcodeDistanceMatchRequestUnit>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<ZipcodeDistanceMatchRequestFormat>,
}

impl ZipcodeDistanceMatchRequest {
    pub fn builder() -> ZipcodeDistanceMatchRequestBuilder {
        <ZipcodeDistanceMatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceMatchRequestBuilder {
    codes: Option<Vec<String>>,
    country: Option<String>,
    distance: Option<f64>,
    unit: Option<ZipcodeDistanceMatchRequestUnit>,
    api_key: Option<String>,
    format: Option<ZipcodeDistanceMatchRequestFormat>,
}

impl ZipcodeDistanceMatchRequestBuilder {
    pub fn codes(mut self, value: Vec<String>) -> Self {
        self.codes = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    pub fn unit(mut self, value: ZipcodeDistanceMatchRequestUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeDistanceMatchRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceMatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`codes`](ZipcodeDistanceMatchRequestBuilder::codes)
    /// - [`country`](ZipcodeDistanceMatchRequestBuilder::country)
    /// - [`api_key`](ZipcodeDistanceMatchRequestBuilder::api_key)
    pub fn build(self) -> Result<ZipcodeDistanceMatchRequest, BuildError> {
        Ok(ZipcodeDistanceMatchRequest {
            codes: self
                .codes
                .ok_or_else(|| BuildError::missing_field("codes"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            distance: self.distance,
            unit: self.unit,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
