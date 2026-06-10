pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem {
    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress,
    ),

    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne,
    ),

    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority,
    ),

    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName,
    ),

    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin,
    ),

    DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings(
        DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings,
    ),
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem {
    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_address(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress(_)
        )
    }

    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne(_)
        )
    }

    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_priority(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority(_)
        )
    }

    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_single_name(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName(_)
        )
    }

    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_admin(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin(_)
        )
    }

    pub fn is_domain_dns_history_response_historical_dns_records_item_dns_records_item_strings(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings(_)
        )
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_address(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_address(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_one(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_one(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_priority(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_priority(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_single_name(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_single_name(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_admin(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_admin(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_domain_dns_history_response_historical_dns_records_item_dns_records_item_strings(
        &self,
    ) -> Option<&DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_dns_history_response_historical_dns_records_item_dns_records_item_strings(
        self,
    ) -> Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings> {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAddress(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
            Self::DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemStrings(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}
