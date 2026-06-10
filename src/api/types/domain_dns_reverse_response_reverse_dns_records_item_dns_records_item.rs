pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem {
    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress,
    ),

    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne,
    ),

    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority,
    ),

    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName,
    ),

    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin,
    ),

    DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings(
        DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings,
    ),
}

impl DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem {
    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_address(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress(_)
        )
    }

    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne(_)
        )
    }

    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_priority(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority(_)
        )
    }

    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_single_name(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName(_)
        )
    }

    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_admin(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin(_)
        )
    }

    pub fn is_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_strings(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings(_)
        )
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_address(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_address(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_one(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_one(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_priority(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_priority(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_single_name(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_single_name(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_admin(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_admin(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_strings(
        &self,
    ) -> Option<&DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_reverse_response_reverse_dns_records_item_dns_records_item_strings(
        self,
    ) -> Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriority(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemSingleName(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAdmin(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemStrings(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
