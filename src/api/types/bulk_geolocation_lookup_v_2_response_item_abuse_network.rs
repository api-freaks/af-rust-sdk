pub use crate::prelude::*;

/// Network information for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseNetwork {
    /// Network access type classification (e.g., DSL, Cable, Mobile, 5G) when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// Network prefix in CIDR notation that contains the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// true if the IP is anycast (same IP announced from multiple locations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anycast: Option<bool>,
    /// CDN provider name (e.g. Cloudflare) when is_cdn is true; empty otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_provider_name: Option<String>,
    /// true if the IP belongs to a content delivery network (CDN) or a cloud provider's network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cdn: Option<bool>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseNetwork {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseNetworkBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseNetworkBuilder {
    connection_type: Option<String>,
    route: Option<String>,
    is_anycast: Option<bool>,
    cdn_provider_name: Option<String>,
    is_cdn: Option<bool>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseNetworkBuilder {
    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn is_anycast(mut self, value: bool) -> Self {
        self.is_anycast = Some(value);
        self
    }

    pub fn cdn_provider_name(mut self, value: impl Into<String>) -> Self {
        self.cdn_provider_name = Some(value.into());
        self
    }

    pub fn is_cdn(mut self, value: bool) -> Self {
        self.is_cdn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseNetwork`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuseNetwork, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseNetwork {
            connection_type: self.connection_type,
            route: self.route,
            is_anycast: self.is_anycast,
            cdn_provider_name: self.cdn_provider_name,
            is_cdn: self.is_cdn,
        })
    }
}
