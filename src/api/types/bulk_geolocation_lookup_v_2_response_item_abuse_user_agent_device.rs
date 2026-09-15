pub use crate::prelude::*;

/// Device details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice {
    /// Detected device label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Device category (Desktop, Mobile, Tablet, Bot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Device vendor/brand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// CPU / architecture string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseUserAgentDeviceBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseUserAgentDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentDeviceBuilder {
    name: Option<String>,
    r#type: Option<String>,
    brand: Option<String>,
    cpu: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentDeviceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn cpu(mut self, value: impl Into<String>) -> Self {
        self.cpu = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice`].
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice {
            name: self.name,
            r#type: self.r#type,
            brand: self.brand,
            cpu: self.cpu,
        })
    }
}
