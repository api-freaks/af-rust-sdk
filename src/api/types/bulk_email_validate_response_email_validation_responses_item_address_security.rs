pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub threat_score: f64,
    #[serde(default)]
    pub is_tor: bool,
    #[serde(default)]
    pub is_proxy: bool,
    #[serde(default)]
    pub proxy_type: String,
    #[serde(default)]
    pub proxy_provider: String,
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(default)]
    pub is_known_attacker: bool,
    #[serde(default)]
    pub is_spam: bool,
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default)]
    pub is_cloud_provider: bool,
    #[serde(default)]
    pub cloud_provider: String,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity {
    pub fn builder() -> BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder
    {
        <BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder {
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

impl BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder {
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

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`threat_score`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::threat_score)
    /// - [`is_tor`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_tor)
    /// - [`is_proxy`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_proxy)
    /// - [`proxy_type`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::proxy_type)
    /// - [`proxy_provider`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::proxy_provider)
    /// - [`is_anonymous`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_anonymous)
    /// - [`is_known_attacker`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_known_attacker)
    /// - [`is_spam`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_spam)
    /// - [`is_bot`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_bot)
    /// - [`is_cloud_provider`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::is_cloud_provider)
    /// - [`cloud_provider`](BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurityBuilder::cloud_provider)
    pub fn build(
        self,
    ) -> Result<BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity, BuildError>
    {
        Ok(
            BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity {
                threat_score: self
                    .threat_score
                    .ok_or_else(|| BuildError::missing_field("threat_score"))?,
                is_tor: self
                    .is_tor
                    .ok_or_else(|| BuildError::missing_field("is_tor"))?,
                is_proxy: self
                    .is_proxy
                    .ok_or_else(|| BuildError::missing_field("is_proxy"))?,
                proxy_type: self
                    .proxy_type
                    .ok_or_else(|| BuildError::missing_field("proxy_type"))?,
                proxy_provider: self
                    .proxy_provider
                    .ok_or_else(|| BuildError::missing_field("proxy_provider"))?,
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
