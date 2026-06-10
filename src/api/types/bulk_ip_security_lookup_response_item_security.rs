pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkIpSecurityLookupResponseItemSecurity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threat_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_provider_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub proxy_confidence_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_last_seen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_residential_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vpn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_provider_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vpn_confidence_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_last_seen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_provider_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_known_attacker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloud_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_name: Option<String>,
}

impl BulkIpSecurityLookupResponseItemSecurity {
    pub fn builder() -> BulkIpSecurityLookupResponseItemSecurityBuilder {
        <BulkIpSecurityLookupResponseItemSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkIpSecurityLookupResponseItemSecurityBuilder {
    threat_score: Option<f64>,
    is_tor: Option<bool>,
    is_proxy: Option<bool>,
    proxy_provider_names: Option<Vec<String>>,
    proxy_confidence_score: Option<f64>,
    proxy_last_seen: Option<String>,
    is_residential_proxy: Option<bool>,
    is_vpn: Option<bool>,
    vpn_provider_names: Option<Vec<String>>,
    vpn_confidence_score: Option<f64>,
    vpn_last_seen: Option<String>,
    is_relay: Option<bool>,
    relay_provider_name: Option<String>,
    is_anonymous: Option<bool>,
    is_known_attacker: Option<bool>,
    is_bot: Option<bool>,
    is_spam: Option<bool>,
    is_cloud_provider: Option<bool>,
    cloud_provider_name: Option<String>,
}

impl BulkIpSecurityLookupResponseItemSecurityBuilder {
    pub fn threat_score(mut self, value: f64) -> Self {
        self.threat_score = Some(value);
        self
    }

    pub fn is_tor(mut self, value: bool) -> Self {
        self.is_tor = Some(value);
        self
    }

    pub fn is_proxy(mut self, value: bool) -> Self {
        self.is_proxy = Some(value);
        self
    }

    pub fn proxy_provider_names(mut self, value: Vec<String>) -> Self {
        self.proxy_provider_names = Some(value);
        self
    }

    pub fn proxy_confidence_score(mut self, value: f64) -> Self {
        self.proxy_confidence_score = Some(value);
        self
    }

    pub fn proxy_last_seen(mut self, value: impl Into<String>) -> Self {
        self.proxy_last_seen = Some(value.into());
        self
    }

    pub fn is_residential_proxy(mut self, value: bool) -> Self {
        self.is_residential_proxy = Some(value);
        self
    }

    pub fn is_vpn(mut self, value: bool) -> Self {
        self.is_vpn = Some(value);
        self
    }

    pub fn vpn_provider_names(mut self, value: Vec<String>) -> Self {
        self.vpn_provider_names = Some(value);
        self
    }

    pub fn vpn_confidence_score(mut self, value: f64) -> Self {
        self.vpn_confidence_score = Some(value);
        self
    }

    pub fn vpn_last_seen(mut self, value: impl Into<String>) -> Self {
        self.vpn_last_seen = Some(value.into());
        self
    }

    pub fn is_relay(mut self, value: bool) -> Self {
        self.is_relay = Some(value);
        self
    }

    pub fn relay_provider_name(mut self, value: impl Into<String>) -> Self {
        self.relay_provider_name = Some(value.into());
        self
    }

    pub fn is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = Some(value);
        self
    }

    pub fn is_known_attacker(mut self, value: bool) -> Self {
        self.is_known_attacker = Some(value);
        self
    }

    pub fn is_bot(mut self, value: bool) -> Self {
        self.is_bot = Some(value);
        self
    }

    pub fn is_spam(mut self, value: bool) -> Self {
        self.is_spam = Some(value);
        self
    }

    pub fn is_cloud_provider(mut self, value: bool) -> Self {
        self.is_cloud_provider = Some(value);
        self
    }

    pub fn cloud_provider_name(mut self, value: impl Into<String>) -> Self {
        self.cloud_provider_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkIpSecurityLookupResponseItemSecurity`].
    pub fn build(self) -> Result<BulkIpSecurityLookupResponseItemSecurity, BuildError> {
        Ok(BulkIpSecurityLookupResponseItemSecurity {
            threat_score: self.threat_score,
            is_tor: self.is_tor,
            is_proxy: self.is_proxy,
            proxy_provider_names: self.proxy_provider_names,
            proxy_confidence_score: self.proxy_confidence_score,
            proxy_last_seen: self.proxy_last_seen,
            is_residential_proxy: self.is_residential_proxy,
            is_vpn: self.is_vpn,
            vpn_provider_names: self.vpn_provider_names,
            vpn_confidence_score: self.vpn_confidence_score,
            vpn_last_seen: self.vpn_last_seen,
            is_relay: self.is_relay,
            relay_provider_name: self.relay_provider_name,
            is_anonymous: self.is_anonymous,
            is_known_attacker: self.is_known_attacker,
            is_bot: self.is_bot,
            is_spam: self.is_spam,
            is_cloud_provider: self.is_cloud_provider,
            cloud_provider_name: self.cloud_provider_name,
        })
    }
}
