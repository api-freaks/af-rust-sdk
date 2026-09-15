pub use crate::prelude::*;

/// Operating system details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem {
    /// Operating system name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OS category (Desktop, Mobile, Server).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// OS version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Major OS version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
    /// OS build identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystemBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystemBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    build: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystemBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem`].
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem, BuildError> {
        Ok(
            BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem {
                name: self.name,
                r#type: self.r#type,
                version: self.version,
                version_major: self.version_major,
                build: self.build,
            },
        )
    }
}
