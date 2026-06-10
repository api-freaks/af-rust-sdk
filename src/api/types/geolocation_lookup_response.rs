pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeolocationLookupResponse {
    /// The IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The hostname (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeolocationLookupResponseLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<GeolocationLookupResponseCountryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<GeolocationLookupResponseNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<GeolocationLookupResponseCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<GeolocationLookupResponseSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<Vec<GeolocationLookupResponseAbuseItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<GeolocationLookupResponseTimeZone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<GeolocationLookupResponseUserAgent>,
}

impl GeolocationLookupResponse {
    pub fn builder() -> GeolocationLookupResponseBuilder {
        <GeolocationLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseBuilder {
    ip: Option<String>,
    hostname: Option<String>,
    location: Option<GeolocationLookupResponseLocation>,
    country_metadata: Option<GeolocationLookupResponseCountryMetadata>,
    network: Option<GeolocationLookupResponseNetwork>,
    currency: Option<GeolocationLookupResponseCurrency>,
    security: Option<GeolocationLookupResponseSecurity>,
    abuse: Option<Vec<GeolocationLookupResponseAbuseItem>>,
    time_zone: Option<GeolocationLookupResponseTimeZone>,
    user_agent: Option<GeolocationLookupResponseUserAgent>,
}

impl GeolocationLookupResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn location(mut self, value: GeolocationLookupResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn country_metadata(mut self, value: GeolocationLookupResponseCountryMetadata) -> Self {
        self.country_metadata = Some(value);
        self
    }

    pub fn network(mut self, value: GeolocationLookupResponseNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn currency(mut self, value: GeolocationLookupResponseCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn security(mut self, value: GeolocationLookupResponseSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn abuse(mut self, value: Vec<GeolocationLookupResponseAbuseItem>) -> Self {
        self.abuse = Some(value);
        self
    }

    pub fn time_zone(mut self, value: GeolocationLookupResponseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_agent(mut self, value: GeolocationLookupResponseUserAgent) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupResponse`].
    pub fn build(self) -> Result<GeolocationLookupResponse, BuildError> {
        Ok(GeolocationLookupResponse {
            ip: self.ip,
            hostname: self.hostname,
            location: self.location,
            country_metadata: self.country_metadata,
            network: self.network,
            currency: self.currency,
            security: self.security,
            abuse: self.abuse,
            time_zone: self.time_zone,
            user_agent: self.user_agent,
        })
    }
}
