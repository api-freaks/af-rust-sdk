pub use crate::prelude::*;

/// Threat and anonymity profile for the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity {
    /// Quantitative risk assessment score. Higher values indicate elevated security risk.
    #[serde(default)]
    pub threat_score: i64,
    /// Detection flag for IP addresses routed through the Tor anonymity network.
    #[serde(default)]
    pub is_tor: bool,
    /// Detection flag identifying IP addresses operating as proxy servers.
    #[serde(default)]
    pub is_proxy: bool,
    /// Classification of detected proxy infrastructure type (e.g. VPN, HTTP, SOCKS). Included only when is_proxy is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    /// Identified service provider operating the proxy infrastructure. Included only when is_proxy is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_provider: Option<String>,
    /// Anonymization detection flag for IP addresses using identity obfuscation techniques.
    #[serde(default)]
    pub is_anonymous: bool,
    /// Threat intelligence flag for IP addresses documented as confirmed malicious sources.
    #[serde(default)]
    pub is_known_attacker: bool,
    /// Reputation flag for IP addresses associated with spam transmission.
    #[serde(default)]
    pub is_spam: bool,
    /// Automated traffic detection flag for IP addresses from bot networks.
    #[serde(default)]
    pub is_bot: bool,
    /// Infrastructure classification flag for IP addresses from cloud computing providers.
    #[serde(default)]
    pub is_cloud_provider: bool,
    /// Identified cloud infrastructure or hosting service provider name.
    #[serde(default)]
    pub cloud_provider: String,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity {
    pub fn builder(
    ) -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder {
    threat_score: Option<i64>,
    is_tor: Option<bool>,
    is_proxy: Option<bool>,
    proxy_type: Option<String>,
    proxy_provider: Option<String>,
    is_anonymous: Option<bool>,
    is_known_attacker: Option<bool>,
    is_spam: Option<bool>,
    is_bot: Option<bool>,
    is_cloud_provider: Option<bool>,
    cloud_provider: Option<String>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder {
    pub fn threat_score(mut self, value: i64) -> Self {
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

    pub fn proxy_type(mut self, value: impl Into<String>) -> Self {
        self.proxy_type = Some(value.into());
        self
    }

    pub fn proxy_provider(mut self, value: impl Into<String>) -> Self {
        self.proxy_provider = Some(value.into());
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

    pub fn is_spam(mut self, value: bool) -> Self {
        self.is_spam = Some(value);
        self
    }

    pub fn is_bot(mut self, value: bool) -> Self {
        self.is_bot = Some(value);
        self
    }

    pub fn is_cloud_provider(mut self, value: bool) -> Self {
        self.is_cloud_provider = Some(value);
        self
    }

    pub fn cloud_provider(mut self, value: impl Into<String>) -> Self {
        self.cloud_provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`threat_score`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::threat_score)
    /// - [`is_tor`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_tor)
    /// - [`is_proxy`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_proxy)
    /// - [`is_anonymous`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_anonymous)
    /// - [`is_known_attacker`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_known_attacker)
    /// - [`is_spam`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_spam)
    /// - [`is_bot`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_bot)
    /// - [`is_cloud_provider`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::is_cloud_provider)
    /// - [`cloud_provider`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurityBuilder::cloud_provider)
    pub fn build(
        self,
    ) -> Result<
        BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity,
        BuildError,
    > {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity {
                threat_score: self
                    .threat_score
                    .ok_or_else(|| BuildError::missing_field("threat_score"))?,
                is_tor: self
                    .is_tor
                    .ok_or_else(|| BuildError::missing_field("is_tor"))?,
                is_proxy: self
                    .is_proxy
                    .ok_or_else(|| BuildError::missing_field("is_proxy"))?,
                proxy_type: self.proxy_type,
                proxy_provider: self.proxy_provider,
                is_anonymous: self
                    .is_anonymous
                    .ok_or_else(|| BuildError::missing_field("is_anonymous"))?,
                is_known_attacker: self
                    .is_known_attacker
                    .ok_or_else(|| BuildError::missing_field("is_known_attacker"))?,
                is_spam: self
                    .is_spam
                    .ok_or_else(|| BuildError::missing_field("is_spam"))?,
                is_bot: self
                    .is_bot
                    .ok_or_else(|| BuildError::missing_field("is_bot"))?,
                is_cloud_provider: self
                    .is_cloud_provider
                    .ok_or_else(|| BuildError::missing_field("is_cloud_provider"))?,
                cloud_provider: self
                    .cloud_provider
                    .ok_or_else(|| BuildError::missing_field("cloud_provider"))?,
            },
        )
    }
}
