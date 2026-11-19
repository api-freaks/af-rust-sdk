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
    /// Bot detection confidence from 0 to 100. 0 unless is_bot is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bot_confidence_score: Option<f64>,
    /// Last observed bot activity date in YYYY-MM-DD, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_last_seen: Option<String>,
    /// Organization running the bot, when known (mostly declared bots).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_operator_name: Option<String>,
    /// Kind of bot activity, when known. Declared bots (is_known_good_bot true): search_engine, ai_crawler, ai_assistant, seo_crawler, site_monitor, link_preview, feed_fetcher, archiver, ad_verification, scanner. Undeclared bots (is_known_good_bot false): scanner, scraper, exploit, credential_stuffing, brute_force, worm. New values can be added over time, so treat it as a plain string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    /// Corporate gateway provider name, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_gateway_provider_name: Option<String>,
    /// Kind of corporate gateway, when known. Known values: secure_web_gateway, browser_isolation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_gateway_type: Option<String>,
    /// true if the IP is an outbound address of a corporate security gateway (SWG, SSE, or SASE).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_corporate_gateway: Option<bool>,
    /// true if the bot is a declared bot with a verifiable operator (e.g. a search engine crawler or uptime monitor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_known_good_bot: Option<bool>,
    /// Number of days in the last 30 days on which the IP was seen acting as a proxy. `0` when no proxy activity was observed.
    #[serde(rename = "proxy_days_seen_30d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_days_seen30d: Option<i64>,
    /// Number of days in the last 7 days on which the IP was seen acting as a proxy. `0` when no proxy activity was observed.
    #[serde(rename = "proxy_days_seen_7d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_days_seen7d: Option<i64>,
    /// Number of distinct proxy providers that offered the IP in the last 30 days. `0` when no proxy activity was observed.
    #[serde(rename = "proxy_provider_count_30d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_provider_count30d: Option<i64>,
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
    bot_confidence_score: Option<f64>,
    bot_last_seen: Option<String>,
    bot_operator_name: Option<String>,
    bot_type: Option<String>,
    corporate_gateway_provider_name: Option<String>,
    corporate_gateway_type: Option<String>,
    is_corporate_gateway: Option<bool>,
    is_known_good_bot: Option<bool>,
    proxy_days_seen30d: Option<i64>,
    proxy_days_seen7d: Option<i64>,
    proxy_provider_count30d: Option<i64>,
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

    pub fn bot_confidence_score(mut self, value: f64) -> Self {
        self.bot_confidence_score = Some(value);
        self
    }

    pub fn bot_last_seen(mut self, value: impl Into<String>) -> Self {
        self.bot_last_seen = Some(value.into());
        self
    }

    pub fn bot_operator_name(mut self, value: impl Into<String>) -> Self {
        self.bot_operator_name = Some(value.into());
        self
    }

    pub fn bot_type(mut self, value: impl Into<String>) -> Self {
        self.bot_type = Some(value.into());
        self
    }

    pub fn corporate_gateway_provider_name(mut self, value: impl Into<String>) -> Self {
        self.corporate_gateway_provider_name = Some(value.into());
        self
    }

    pub fn corporate_gateway_type(mut self, value: impl Into<String>) -> Self {
        self.corporate_gateway_type = Some(value.into());
        self
    }

    pub fn is_corporate_gateway(mut self, value: bool) -> Self {
        self.is_corporate_gateway = Some(value);
        self
    }

    pub fn is_known_good_bot(mut self, value: bool) -> Self {
        self.is_known_good_bot = Some(value);
        self
    }

    pub fn proxy_days_seen30d(mut self, value: i64) -> Self {
        self.proxy_days_seen30d = Some(value);
        self
    }

    pub fn proxy_days_seen7d(mut self, value: i64) -> Self {
        self.proxy_days_seen7d = Some(value);
        self
    }

    pub fn proxy_provider_count30d(mut self, value: i64) -> Self {
        self.proxy_provider_count30d = Some(value);
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
            bot_confidence_score: self.bot_confidence_score,
            bot_last_seen: self.bot_last_seen,
            bot_operator_name: self.bot_operator_name,
            bot_type: self.bot_type,
            corporate_gateway_provider_name: self.corporate_gateway_provider_name,
            corporate_gateway_type: self.corporate_gateway_type,
            is_corporate_gateway: self.is_corporate_gateway,
            is_known_good_bot: self.is_known_good_bot,
            proxy_days_seen30d: self.proxy_days_seen30d,
            proxy_days_seen7d: self.proxy_days_seen7d,
            proxy_provider_count30d: self.proxy_provider_count30d,
        })
    }
}
