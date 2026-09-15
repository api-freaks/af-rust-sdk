pub use crate::prelude::*;

/// Individual trust / risk indicators for the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseTrustSignalsIndicators {
    /// Indicates whether the domain was recently registered. null when WHOIS data is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_newly_registered: Option<bool>,
    /// Indicates whether the domain uses a free TLD extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_free_extension: Option<bool>,
    /// Indicates whether the domain uses a free SSL certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_free_ssl: Option<bool>,
    /// Indicates whether WHOIS privacy protection is enabled. null when WHOIS data is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_privacy_whois: Option<bool>,
    /// Age of the SSL certificate in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_age_days: Option<i64>,
    /// Indicates whether a DMARC record exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_dmarc: Option<bool>,
    /// Indicates whether an SPF record exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spf: Option<bool>,
    /// Indicates whether the domain redirects to an external site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirects_externally: Option<bool>,
    /// Indicates whether obfuscated JavaScript was detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript_obfuscated: Option<bool>,
    /// Age of the domain in days. null when WHOIS data is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_age_days: Option<i64>,
    /// Domain registrar name. null when WHOIS data is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
}

impl DomainReputationResponseTrustSignalsIndicators {
    pub fn builder() -> DomainReputationResponseTrustSignalsIndicatorsBuilder {
        <DomainReputationResponseTrustSignalsIndicatorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseTrustSignalsIndicatorsBuilder {
    is_newly_registered: Option<bool>,
    uses_free_extension: Option<bool>,
    uses_free_ssl: Option<bool>,
    has_privacy_whois: Option<bool>,
    ssl_age_days: Option<i64>,
    has_dmarc: Option<bool>,
    has_spf: Option<bool>,
    redirects_externally: Option<bool>,
    javascript_obfuscated: Option<bool>,
    domain_age_days: Option<i64>,
    registrar: Option<String>,
}

impl DomainReputationResponseTrustSignalsIndicatorsBuilder {
    pub fn is_newly_registered(mut self, value: bool) -> Self {
        self.is_newly_registered = Some(value);
        self
    }

    pub fn uses_free_extension(mut self, value: bool) -> Self {
        self.uses_free_extension = Some(value);
        self
    }

    pub fn uses_free_ssl(mut self, value: bool) -> Self {
        self.uses_free_ssl = Some(value);
        self
    }

    pub fn has_privacy_whois(mut self, value: bool) -> Self {
        self.has_privacy_whois = Some(value);
        self
    }

    pub fn ssl_age_days(mut self, value: i64) -> Self {
        self.ssl_age_days = Some(value);
        self
    }

    pub fn has_dmarc(mut self, value: bool) -> Self {
        self.has_dmarc = Some(value);
        self
    }

    pub fn has_spf(mut self, value: bool) -> Self {
        self.has_spf = Some(value);
        self
    }

    pub fn redirects_externally(mut self, value: bool) -> Self {
        self.redirects_externally = Some(value);
        self
    }

    pub fn javascript_obfuscated(mut self, value: bool) -> Self {
        self.javascript_obfuscated = Some(value);
        self
    }

    pub fn domain_age_days(mut self, value: i64) -> Self {
        self.domain_age_days = Some(value);
        self
    }

    pub fn registrar(mut self, value: impl Into<String>) -> Self {
        self.registrar = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseTrustSignalsIndicators`].
    pub fn build(self) -> Result<DomainReputationResponseTrustSignalsIndicators, BuildError> {
        Ok(DomainReputationResponseTrustSignalsIndicators {
            is_newly_registered: self.is_newly_registered,
            uses_free_extension: self.uses_free_extension,
            uses_free_ssl: self.uses_free_ssl,
            has_privacy_whois: self.has_privacy_whois,
            ssl_age_days: self.ssl_age_days,
            has_dmarc: self.has_dmarc,
            has_spf: self.has_spf,
            redirects_externally: self.redirects_externally,
            javascript_obfuscated: self.javascript_obfuscated,
            domain_age_days: self.domain_age_days,
            registrar: self.registrar,
        })
    }
}
