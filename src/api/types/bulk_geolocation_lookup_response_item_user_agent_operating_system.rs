pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemUserAgentOperatingSystem {
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

impl BulkGeolocationLookupResponseItemUserAgentOperatingSystem {
    pub fn builder() -> BulkGeolocationLookupResponseItemUserAgentOperatingSystemBuilder {
        <BulkGeolocationLookupResponseItemUserAgentOperatingSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemUserAgentOperatingSystemBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    build: Option<String>,
}

impl BulkGeolocationLookupResponseItemUserAgentOperatingSystemBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemUserAgentOperatingSystem`].
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupResponseItemUserAgentOperatingSystem, BuildError> {
        Ok(BulkGeolocationLookupResponseItemUserAgentOperatingSystem {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
            build: self.build,
        })
    }
}
