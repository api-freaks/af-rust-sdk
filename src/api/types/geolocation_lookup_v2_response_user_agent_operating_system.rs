pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseUserAgentOperatingSystem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

impl GeolocationLookupV2ResponseUserAgentOperatingSystem {
    pub fn builder() -> GeolocationLookupV2ResponseUserAgentOperatingSystemBuilder {
        <GeolocationLookupV2ResponseUserAgentOperatingSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseUserAgentOperatingSystemBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    build: Option<String>,
}

impl GeolocationLookupV2ResponseUserAgentOperatingSystemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn version_major(mut self, value: impl Into<String>) -> Self {
        self.version_major = Some(value.into());
        self
    }

    pub fn os_build(mut self, value: impl Into<String>) -> Self {
        self.build = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseUserAgentOperatingSystem`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseUserAgentOperatingSystem, BuildError> {
        Ok(GeolocationLookupV2ResponseUserAgentOperatingSystem {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
            build: self.build,
        })
    }
}
