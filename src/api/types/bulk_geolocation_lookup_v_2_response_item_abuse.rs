pub use crate::prelude::*;

/// Geolocation and threat intelligence result for one successfully resolved IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupV2ResponseItemAbuse {
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
    pub location: Option<BulkGeolocationLookupV2ResponseItemAbuseLocation>,
    /// Country-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata>,
    /// Network information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<BulkGeolocationLookupV2ResponseItemAbuseNetwork>,
    /// Autonomous System details for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<BulkGeolocationLookupV2ResponseItemAbuseAsn>,
    /// Company or ISP information mapped to the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<BulkGeolocationLookupV2ResponseItemAbuseCompany>,
    /// Currency information for the IP's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<BulkGeolocationLookupV2ResponseItemAbuseCurrency>,
    /// Threat intelligence and security information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkGeolocationLookupV2ResponseItemAbuseSecurity>,
    /// Abuse contact information for the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<BulkGeolocationLookupV2ResponseItemAbuseAbuse>,
    /// Time zone information for the IP's location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<BulkGeolocationLookupV2ResponseItemAbuseTimeZone>,
    /// Parsed User-Agent details from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgent>,
}

impl BulkGeolocationLookupV2ResponseItemAbuse {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseBuilder {
    ip: Option<String>,
    domain: Option<String>,
    hostname: Option<String>,
    location: Option<BulkGeolocationLookupV2ResponseItemAbuseLocation>,
    country_metadata: Option<BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata>,
    network: Option<BulkGeolocationLookupV2ResponseItemAbuseNetwork>,
    asn: Option<BulkGeolocationLookupV2ResponseItemAbuseAsn>,
    company: Option<BulkGeolocationLookupV2ResponseItemAbuseCompany>,
    currency: Option<BulkGeolocationLookupV2ResponseItemAbuseCurrency>,
    security: Option<BulkGeolocationLookupV2ResponseItemAbuseSecurity>,
    abuse: Option<BulkGeolocationLookupV2ResponseItemAbuseAbuse>,
    time_zone: Option<BulkGeolocationLookupV2ResponseItemAbuseTimeZone>,
    user_agent: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgent>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseBuilder {
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

    pub fn location(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn country_metadata(
        mut self,
        value: BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata,
    ) -> Self {
        self.country_metadata = Some(value);
        self
    }

    pub fn network(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn asn(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn company(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn currency(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn security(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn abuse(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseAbuse) -> Self {
        self.abuse = Some(value);
        self
    }

    pub fn time_zone(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_agent(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseUserAgent) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ip`](BulkGeolocationLookupV2ResponseItemAbuseBuilder::ip)
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuse, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuse {
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
