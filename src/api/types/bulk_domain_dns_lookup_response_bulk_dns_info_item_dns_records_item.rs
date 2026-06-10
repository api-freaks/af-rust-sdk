pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem {
    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress,
    ),

    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne,
    ),

    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority,
    ),

    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName,
    ),

    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin,
    ),

    BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings(
        BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings,
    ),
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem {
    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_address(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress(_)
        )
    }

    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne(_)
        )
    }

    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_priority(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority(_)
        )
    }

    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_single_name(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName(_)
        )
    }

    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_admin(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin(_)
        )
    }

    pub fn is_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_strings(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings(_)
        )
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_address(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_address(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_one(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_one(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_priority(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_priority(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_single_name(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_single_name(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_admin(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_admin(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_strings(
        &self,
    ) -> Option<&BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_bulk_domain_dns_lookup_response_bulk_dns_info_item_dns_records_item_strings(
        self,
    ) -> Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings> {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriority(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemSingleName(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAdmin(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
