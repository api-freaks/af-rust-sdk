pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeolocationLookupV2Response {
    /// The IP address used for the lookup (IPv4 or IPv6).
    #[serde(default)]
    pub ip: String,
    /// The input domain, returned only for domain-based lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Reverse DNS hostname (PTR) for the input IP; returns the input IP if not resolvable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Geographic location information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeolocationLookupV2ResponseLocation>,
    /// Country-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<GeolocationLookupV2ResponseCountryMetadata>,
    /// Network information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<GeolocationLookupV2ResponseNetwork>,
    /// Autonomous System details for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<GeolocationLookupV2ResponseAsn>,
    /// Company or ISP information mapped to the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<GeolocationLookupV2ResponseCompany>,
    /// Currency information for the IP's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<GeolocationLookupV2ResponseCurrency>,
    /// Threat intelligence and security information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<GeolocationLookupV2ResponseSecurity>,
    /// Abuse contact information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<GeolocationLookupV2ResponseAbuse>,
    /// Time zone information for the IP's location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<GeolocationLookupV2ResponseTimeZone>,
    /// Parsed User-Agent details from the request.
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
    domain: Option<String>,
    hostname: Option<String>,
    location: Option<GeolocationLookupV2ResponseLocation>,
    country_metadata: Option<GeolocationLookupV2ResponseCountryMetadata>,
    network: Option<GeolocationLookupV2ResponseNetwork>,
    asn: Option<GeolocationLookupV2ResponseAsn>,
    company: Option<GeolocationLookupV2ResponseCompany>,
    currency: Option<GeolocationLookupV2ResponseCurrency>,
    security: Option<GeolocationLookupV2ResponseSecurity>,
    abuse: Option<GeolocationLookupV2ResponseAbuse>,
    time_zone: Option<GeolocationLookupV2ResponseTimeZone>,
    user_agent: Option<GeolocationLookupV2ResponseUserAgent>,
}

impl GeolocationLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
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

    pub fn asn(mut self, value: GeolocationLookupV2ResponseAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn company(mut self, value: GeolocationLookupV2ResponseCompany) -> Self {
        self.company = Some(value);
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

    pub fn abuse(mut self, value: GeolocationLookupV2ResponseAbuse) -> Self {
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
    /// This method will fail if any of the following fields are not set:
    /// - [`ip`](GeolocationLookupV2ResponseBuilder::ip)
    pub fn build(self) -> Result<GeolocationLookupV2Response, BuildError> {
        Ok(GeolocationLookupV2Response {
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            domain: self.domain,
            hostname: self.hostname,
            location: self.location,
            country_metadata: self.country_metadata,
            network: self.network,
            asn: self.asn,
            company: self.company,
            currency: self.currency,
            security: self.security,
            abuse: self.abuse,
            time_zone: self.time_zone,
            user_agent: self.user_agent,
        })
    }
}
