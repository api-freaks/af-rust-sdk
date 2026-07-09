pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeolocationLookupV2Response {
    /// The IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The hostname (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name (present only when the `ip` parameter is a domain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeolocationLookupV2ResponseLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<GeolocationLookupV2ResponseCountryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<GeolocationLookupV2ResponseNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<GeolocationLookupV2ResponseCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<GeolocationLookupV2ResponseSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<GeolocationLookupV2ResponseAbuseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<GeolocationLookupV2ResponseTimeZone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<GeolocationLookupV2ResponseUserAgent>,
}

impl GeolocationLookupV2Response {
    pub fn builder() -> GeolocationLookupV2ResponseBuilder {
        <GeolocationLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseBuilder {
    ip: Option<String>,
    hostname: Option<String>,
    domain: Option<String>,
    location: Option<GeolocationLookupV2ResponseLocation>,
    country_metadata: Option<GeolocationLookupV2ResponseCountryMetadata>,
    network: Option<GeolocationLookupV2ResponseNetwork>,
    currency: Option<GeolocationLookupV2ResponseCurrency>,
    security: Option<GeolocationLookupV2ResponseSecurity>,
    abuse: Option<GeolocationLookupV2ResponseAbuseItem>,
    time_zone: Option<GeolocationLookupV2ResponseTimeZone>,
    user_agent: Option<GeolocationLookupV2ResponseUserAgent>,
}

impl GeolocationLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn location(mut self, value: GeolocationLookupV2ResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn country_metadata(mut self, value: GeolocationLookupV2ResponseCountryMetadata) -> Self {
        self.country_metadata = Some(value);
        self
    }

    pub fn network(mut self, value: GeolocationLookupV2ResponseNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn currency(mut self, value: GeolocationLookupV2ResponseCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn security(mut self, value: GeolocationLookupV2ResponseSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn abuse(mut self, value: GeolocationLookupV2ResponseAbuseItem) -> Self {
        self.abuse = Some(value);
        self
    }

    pub fn time_zone(mut self, value: GeolocationLookupV2ResponseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_agent(mut self, value: GeolocationLookupV2ResponseUserAgent) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2Response`].
    pub fn build(self) -> Result<GeolocationLookupV2Response, BuildError> {
        Ok(GeolocationLookupV2Response {
            ip: self.ip,
            hostname: self.hostname,
            domain: self.domain,
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
