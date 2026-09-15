pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BulkGeolocationLookupV2ResponseItem {
    BulkGeolocationLookupV2ResponseItemAbuse(BulkGeolocationLookupV2ResponseItemAbuse),

    BulkGeolocationLookupV2ResponseItemMessage(BulkGeolocationLookupV2ResponseItemMessage),
}

impl BulkGeolocationLookupV2ResponseItem {
    pub fn is_bulk_geolocation_lookup_v2response_item_abuse(&self) -> bool {
        matches!(self, Self::BulkGeolocationLookupV2ResponseItemAbuse(_))
    }

    pub fn is_bulk_geolocation_lookup_v2response_item_message(&self) -> bool {
        matches!(self, Self::BulkGeolocationLookupV2ResponseItemMessage(_))
    }

    pub fn as_bulk_geolocation_lookup_v2response_item_abuse(
        &self,
    ) -> Option<&BulkGeolocationLookupV2ResponseItemAbuse> {
        match self {
            Self::BulkGeolocationLookupV2ResponseItemAbuse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_geolocation_lookup_v2response_item_abuse(
        self,
    ) -> Option<BulkGeolocationLookupV2ResponseItemAbuse> {
        match self {
            Self::BulkGeolocationLookupV2ResponseItemAbuse(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bulk_geolocation_lookup_v2response_item_message(
        &self,
    ) -> Option<&BulkGeolocationLookupV2ResponseItemMessage> {
        match self {
            Self::BulkGeolocationLookupV2ResponseItemMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_geolocation_lookup_v2response_item_message(
        self,
    ) -> Option<BulkGeolocationLookupV2ResponseItemMessage> {
        match self {
            Self::BulkGeolocationLookupV2ResponseItemMessage(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BulkGeolocationLookupV2ResponseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BulkGeolocationLookupV2ResponseItemAbuse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkGeolocationLookupV2ResponseItemMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
