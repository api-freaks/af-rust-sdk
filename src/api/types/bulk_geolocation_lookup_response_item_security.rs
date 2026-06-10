pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupResponseItemSecurity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threat_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_known_attacker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloud_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
}

impl BulkGeolocationLookupResponseItemSecurity {
    pub fn builder() -> BulkGeolocationLookupResponseItemSecurityBuilder {
        <BulkGeolocationLookupResponseItemSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemSecurityBuilder {
    threat_score: Option<f64>,
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

impl BulkGeolocationLookupResponseItemSecurityBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemSecurity`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemSecurity, BuildError> {
        Ok(BulkGeolocationLookupResponseItemSecurity {
            threat_score: self.threat_score,
            is_tor: self.is_tor,
            is_proxy: self.is_proxy,
            proxy_type: self.proxy_type,
            proxy_provider: self.proxy_provider,
            is_anonymous: self.is_anonymous,
            is_known_attacker: self.is_known_attacker,
            is_spam: self.is_spam,
            is_bot: self.is_bot,
            is_cloud_provider: self.is_cloud_provider,
            cloud_provider: self.cloud_provider,
        })
    }
}
