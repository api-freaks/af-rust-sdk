pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupResponseItem {
    /// The IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The hostname (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BulkGeolocationLookupResponseItemLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_metadata: Option<BulkGeolocationLookupResponseItemCountryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<BulkGeolocationLookupResponseItemNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<BulkGeolocationLookupResponseItemCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkGeolocationLookupResponseItemSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse: Option<Vec<BulkGeolocationLookupResponseItemAbuseItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<BulkGeolocationLookupResponseItemTimeZone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<BulkGeolocationLookupResponseItemUserAgent>,
}

impl BulkGeolocationLookupResponseItem {
    pub fn builder() -> BulkGeolocationLookupResponseItemBuilder {
        <BulkGeolocationLookupResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemBuilder {
    ip: Option<String>,
    hostname: Option<String>,
    location: Option<BulkGeolocationLookupResponseItemLocation>,
    country_metadata: Option<BulkGeolocationLookupResponseItemCountryMetadata>,
    network: Option<BulkGeolocationLookupResponseItemNetwork>,
    currency: Option<BulkGeolocationLookupResponseItemCurrency>,
    security: Option<BulkGeolocationLookupResponseItemSecurity>,
    abuse: Option<Vec<BulkGeolocationLookupResponseItemAbuseItem>>,
    time_zone: Option<BulkGeolocationLookupResponseItemTimeZone>,
    user_agent: Option<BulkGeolocationLookupResponseItemUserAgent>,
}

impl BulkGeolocationLookupResponseItemBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn location(mut self, value: BulkGeolocationLookupResponseItemLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn country_metadata(
        mut self,
        value: BulkGeolocationLookupResponseItemCountryMetadata,
    ) -> Self {
        self.country_metadata = Some(value);
        self
    }

    pub fn network(mut self, value: BulkGeolocationLookupResponseItemNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn currency(mut self, value: BulkGeolocationLookupResponseItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn security(mut self, value: BulkGeolocationLookupResponseItemSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn abuse(mut self, value: Vec<BulkGeolocationLookupResponseItemAbuseItem>) -> Self {
        self.abuse = Some(value);
        self
    }

    pub fn time_zone(mut self, value: BulkGeolocationLookupResponseItemTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_agent(mut self, value: BulkGeolocationLookupResponseItemUserAgent) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItem`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItem, BuildError> {
        Ok(BulkGeolocationLookupResponseItem {
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
