pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DomainDnsLookupResponseDnsRecordsItem {
    DomainDnsLookupResponseDnsRecordsItemAddress(DomainDnsLookupResponseDnsRecordsItemAddress),

    DomainDnsLookupResponseDnsRecordsItemOne(DomainDnsLookupResponseDnsRecordsItemOne),

    DomainDnsLookupResponseDnsRecordsItemPriority(DomainDnsLookupResponseDnsRecordsItemPriority),

    DomainDnsLookupResponseDnsRecordsItemSingleName(
        DomainDnsLookupResponseDnsRecordsItemSingleName,
    ),

    DomainDnsLookupResponseDnsRecordsItemAdmin(DomainDnsLookupResponseDnsRecordsItemAdmin),

    DomainDnsLookupResponseDnsRecordsItemStrings(DomainDnsLookupResponseDnsRecordsItemStrings),
}

impl DomainDnsLookupResponseDnsRecordsItem {
    pub fn is_domain_dns_lookup_response_dns_records_item_address(&self) -> bool {
        matches!(self, Self::DomainDnsLookupResponseDnsRecordsItemAddress(_))
    }

    pub fn is_domain_dns_lookup_response_dns_records_item_one(&self) -> bool {
        matches!(self, Self::DomainDnsLookupResponseDnsRecordsItemOne(_))
    }

    pub fn is_domain_dns_lookup_response_dns_records_item_priority(&self) -> bool {
        matches!(self, Self::DomainDnsLookupResponseDnsRecordsItemPriority(_))
    }

    pub fn is_domain_dns_lookup_response_dns_records_item_single_name(&self) -> bool {
        matches!(
            self,
            Self::DomainDnsLookupResponseDnsRecordsItemSingleName(_)
        )
    }

    pub fn is_domain_dns_lookup_response_dns_records_item_admin(&self) -> bool {
        matches!(self, Self::DomainDnsLookupResponseDnsRecordsItemAdmin(_))
    }

    pub fn is_domain_dns_lookup_response_dns_records_item_strings(&self) -> bool {
        matches!(self, Self::DomainDnsLookupResponseDnsRecordsItemStrings(_))
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_address(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemAddress(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_address(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemAddress(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_one(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemOne> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_one(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemOne> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_priority(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemPriority(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_priority(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemPriority(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_single_name(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemSingleName(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_single_name(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemSingleName(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_admin(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemAdmin(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_admin(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemAdmin(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_lookup_response_dns_records_item_strings(
        &self,
    ) -> Option<&DomainDnsLookupResponseDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemStrings(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_lookup_response_dns_records_item_strings(
        self,
    ) -> Option<DomainDnsLookupResponseDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemStrings(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for DomainDnsLookupResponseDnsRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainDnsLookupResponseDnsRecordsItemAddress(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsLookupResponseDnsRecordsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsLookupResponseDnsRecordsItemPriority(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsLookupResponseDnsRecordsItemSingleName(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsLookupResponseDnsRecordsItemAdmin(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsLookupResponseDnsRecordsItemStrings(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
