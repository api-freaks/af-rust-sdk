pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem {
    BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact(
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact,
    ),

    BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError(
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError,
    ),
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem {
    pub fn is_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_abuse_contact(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact(_)
        )
    }

    pub fn is_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_error(&self) -> bool {
        matches!(
            self,
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError(_)
        )
    }

    pub fn as_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_abuse_contact(
        &self,
    ) -> Option<&BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact> {
        match self {
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_abuse_contact(
        self,
    ) -> Option<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact> {
        match self {
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_error(
        &self,
    ) -> Option<&BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError> {
        match self {
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_domain_whois_lookup_v2response_bulk_whois_response_item_error(
        self,
    ) -> Option<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError> {
        match self {
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContact(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
