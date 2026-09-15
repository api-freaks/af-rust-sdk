pub use crate::prelude::*;

/// Device details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseUserAgentDevice {
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

impl GeolocationLookupV2ResponseUserAgentDevice {
    pub fn builder() -> GeolocationLookupV2ResponseUserAgentDeviceBuilder {
        <GeolocationLookupV2ResponseUserAgentDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseUserAgentDeviceBuilder {
    name: Option<String>,
    r#type: Option<String>,
    brand: Option<String>,
    cpu: Option<String>,
}

impl GeolocationLookupV2ResponseUserAgentDeviceBuilder {
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

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseUserAgentDevice`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseUserAgentDevice, BuildError> {
        Ok(GeolocationLookupV2ResponseUserAgentDevice {
            name: self.name,
            r#type: self.r#type,
            brand: self.brand,
            cpu: self.cpu,
        })
    }
}
