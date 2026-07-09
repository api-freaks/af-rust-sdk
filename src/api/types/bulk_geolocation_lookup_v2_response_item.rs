pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupV2ResponseItem {
    /// The IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The hostname (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name (present only when the looked-up value is a domain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BulkGeolocationLookupV2ResponseItemLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<BulkGeolocationLookupV2ResponseItemCountryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<BulkGeolocationLookupV2ResponseItemNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<BulkGeolocationLookupV2ResponseItemCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkGeolocationLookupV2ResponseItemSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<BulkGeolocationLookupV2ResponseItemAbuseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<BulkGeolocationLookupV2ResponseItemTimeZone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<BulkGeolocationLookupV2ResponseItemUserAgent>,
}

impl BulkGeolocationLookupV2ResponseItem {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemBuilder {
        <BulkGeolocationLookupV2ResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemBuilder {
    ip: Option<String>,
    hostname: Option<String>,
    domain: Option<String>,
    location: Option<BulkGeolocationLookupV2ResponseItemLocation>,
    country_metadata: Option<BulkGeolocationLookupV2ResponseItemCountryMetadata>,
    network: Option<BulkGeolocationLookupV2ResponseItemNetwork>,
    currency: Option<BulkGeolocationLookupV2ResponseItemCurrency>,
    security: Option<BulkGeolocationLookupV2ResponseItemSecurity>,
    abuse: Option<BulkGeolocationLookupV2ResponseItemAbuseItem>,
    time_zone: Option<BulkGeolocationLookupV2ResponseItemTimeZone>,
    user_agent: Option<BulkGeolocationLookupV2ResponseItemUserAgent>,
}

impl BulkGeolocationLookupV2ResponseItemBuilder {
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

    pub fn location(mut self, value: BulkGeolocationLookupV2ResponseItemLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn country_metadata(
        mut self,
        value: BulkGeolocationLookupV2ResponseItemCountryMetadata,
    ) -> Self {
        self.country_metadata = Some(value);
        self
    }

    pub fn network(mut self, value: BulkGeolocationLookupV2ResponseItemNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn currency(mut self, value: BulkGeolocationLookupV2ResponseItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn security(mut self, value: BulkGeolocationLookupV2ResponseItemSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn abuse(mut self, value: BulkGeolocationLookupV2ResponseItemAbuseItem) -> Self {
        self.abuse = Some(value);
        self
    }

    pub fn time_zone(mut self, value: BulkGeolocationLookupV2ResponseItemTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_agent(mut self, value: BulkGeolocationLookupV2ResponseItemUserAgent) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItem`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItem, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItem {
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
