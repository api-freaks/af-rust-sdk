pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DomainWhoisReverseResponseWhoisDomainsHistoricalItem {
    DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact(
        DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact,
    ),

    DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname(
        DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname,
    ),
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItem {
    pub fn is_domain_whois_reverse_response_whois_domains_historical_item_administrative_contact(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact(_)
        )
    }

    pub fn is_domain_whois_reverse_response_whois_domains_historical_item_companyname(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname(_)
        )
    }

    pub fn as_domain_whois_reverse_response_whois_domains_historical_item_administrative_contact(
        &self,
    ) -> Option<&DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact> {
        match self {
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_whois_reverse_response_whois_domains_historical_item_administrative_contact(
        self,
    ) -> Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact> {
        match self {
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_whois_reverse_response_whois_domains_historical_item_companyname(
        &self,
    ) -> Option<&DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname> {
        match self {
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_whois_reverse_response_whois_domains_historical_item_companyname(
        self,
    ) -> Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname> {
        match self {
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for DomainWhoisReverseResponseWhoisDomainsHistoricalItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
