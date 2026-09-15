pub use crate::prelude::*;

/// Threat intelligence and security information for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseSecurity {
    /// Aggregate risk score from 0 to 100.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub threat_score: f64,
    /// true if the IP matches a Tor exit node.
    #[serde(default)]
    pub is_tor: bool,
    /// true if the IP is associated with a proxy service.
    #[serde(default)]
    pub is_proxy: bool,
    /// Detected proxy provider names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_provider_names: Option<Vec<String>>,
    /// Proxy detection confidence from 0 to 100.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub proxy_confidence_score: f64,
    /// Last observed proxy activity date in YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_last_seen: Option<String>,
    /// true if the IP is linked to a residential proxy network.
    #[serde(default)]
    pub is_residential_proxy: bool,
    /// true if the IP is associated with a VPN service.
    #[serde(default)]
    pub is_vpn: bool,
    /// Detected VPN provider names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_provider_names: Option<Vec<String>>,
    /// VPN detection confidence from 0 to 100.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vpn_confidence_score: f64,
    /// Last observed VPN activity date in YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_last_seen: Option<String>,
    /// true if the IP is associated with a relay network.
    #[serde(default)]
    pub is_relay: bool,
    /// Relay provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_provider_name: Option<String>,
    /// true if any anonymity signal is present.
    #[serde(default)]
    pub is_anonymous: bool,
    /// true if the IP is flagged for known attacker behavior.
    #[serde(default)]
    pub is_known_attacker: bool,
    /// true if the IP is associated with bot activity.
    #[serde(default)]
    pub is_bot: bool,
    /// true if the IP is associated with spam activity.
    #[serde(default)]
    pub is_spam: bool,
    /// true if the IP belongs to a cloud provider range.
    #[serde(default)]
    pub is_cloud_provider: bool,
    /// Cloud provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_name: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseSecurity {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder {
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

impl BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseSecurity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`threat_score`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::threat_score)
    /// - [`is_tor`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_tor)
    /// - [`is_proxy`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_proxy)
    /// - [`proxy_confidence_score`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::proxy_confidence_score)
    /// - [`is_residential_proxy`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_residential_proxy)
    /// - [`is_vpn`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_vpn)
    /// - [`vpn_confidence_score`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::vpn_confidence_score)
    /// - [`is_relay`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_relay)
    /// - [`is_anonymous`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_anonymous)
    /// - [`is_known_attacker`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_known_attacker)
    /// - [`is_bot`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_bot)
    /// - [`is_spam`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_spam)
    /// - [`is_cloud_provider`](BulkGeolocationLookupV2ResponseItemAbuseSecurityBuilder::is_cloud_provider)
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuseSecurity, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseSecurity {
            threat_score: self
                .threat_score
                .ok_or_else(|| BuildError::missing_field("threat_score"))?,
            is_tor: self
                .is_tor
                .ok_or_else(|| BuildError::missing_field("is_tor"))?,
            is_proxy: self
                .is_proxy
                .ok_or_else(|| BuildError::missing_field("is_proxy"))?,
            proxy_provider_names: self.proxy_provider_names,
            proxy_confidence_score: self
                .proxy_confidence_score
                .ok_or_else(|| BuildError::missing_field("proxy_confidence_score"))?,
            proxy_last_seen: self.proxy_last_seen,
            is_residential_proxy: self
                .is_residential_proxy
                .ok_or_else(|| BuildError::missing_field("is_residential_proxy"))?,
            is_vpn: self
                .is_vpn
                .ok_or_else(|| BuildError::missing_field("is_vpn"))?,
            vpn_provider_names: self.vpn_provider_names,
            vpn_confidence_score: self
                .vpn_confidence_score
                .ok_or_else(|| BuildError::missing_field("vpn_confidence_score"))?,
            vpn_last_seen: self.vpn_last_seen,
            is_relay: self
                .is_relay
                .ok_or_else(|| BuildError::missing_field("is_relay"))?,
            relay_provider_name: self.relay_provider_name,
            is_anonymous: self
                .is_anonymous
                .ok_or_else(|| BuildError::missing_field("is_anonymous"))?,
            is_known_attacker: self
                .is_known_attacker
                .ok_or_else(|| BuildError::missing_field("is_known_attacker"))?,
            is_bot: self
                .is_bot
                .ok_or_else(|| BuildError::missing_field("is_bot"))?,
            is_spam: self
                .is_spam
                .ok_or_else(|| BuildError::missing_field("is_spam"))?,
            is_cloud_provider: self
                .is_cloud_provider
                .ok_or_else(|| BuildError::missing_field("is_cloud_provider"))?,
            cloud_provider_name: self.cloud_provider_name,
        })
    }
}
