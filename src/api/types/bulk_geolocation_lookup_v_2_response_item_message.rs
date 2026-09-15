pub use crate::prelude::*;

/// Per-item error, returned in place of a location result when an individual IP is invalid, bogon/reserved, or not found in the database.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BulkGeolocationLookupV2ResponseItemMessage {
    /// Human-readable reason this IP could not be resolved.
    pub message: String,
}

impl BulkGeolocationLookupV2ResponseItemMessage {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemMessageBuilder {
        <BulkGeolocationLookupV2ResponseItemMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemMessageBuilder {
    message: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemMessageBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](BulkGeolocationLookupV2ResponseItemMessageBuilder::message)
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemMessage, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemMessage {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
