pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkUserAgentLookupResponseItemOperatingSystem {
    /// Operating system name or the base software name that runs on the hardware.
    #[serde(default)]
    pub name: String,
    /// Operating system type. Possible values:
    /// 1. Desktop
    /// 2. Mobile
    /// 3. Game Console
    /// 4. Embedded
    /// 5. Cloud
    /// 6. Hacker
    /// 7. Anonymized
    /// 8. Unknown
    #[serde(default)]
    pub r#type: String,
    /// Operating system version
    #[serde(default)]
    pub version: String,
    /// Operating system version major
    #[serde(default)]
    pub version_major: String,
    /// Operating system build
    #[serde(default)]
    pub build: String,
}

impl BulkUserAgentLookupResponseItemOperatingSystem {
    pub fn builder() -> BulkUserAgentLookupResponseItemOperatingSystemBuilder {
        <BulkUserAgentLookupResponseItemOperatingSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkUserAgentLookupResponseItemOperatingSystemBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    build: Option<String>,
}

impl BulkUserAgentLookupResponseItemOperatingSystemBuilder {
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

    /// Consumes the builder and constructs a [`BulkUserAgentLookupResponseItemOperatingSystem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BulkUserAgentLookupResponseItemOperatingSystemBuilder::name)
    /// - [`r#type`](BulkUserAgentLookupResponseItemOperatingSystemBuilder::r#type)
    /// - [`version`](BulkUserAgentLookupResponseItemOperatingSystemBuilder::version)
    /// - [`version_major`](BulkUserAgentLookupResponseItemOperatingSystemBuilder::version_major)
    /// - [`build`](BulkUserAgentLookupResponseItemOperatingSystemBuilder::build)
    pub fn build(self) -> Result<BulkUserAgentLookupResponseItemOperatingSystem, BuildError> {
        Ok(BulkUserAgentLookupResponseItemOperatingSystem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            version_major: self
                .version_major
                .ok_or_else(|| BuildError::missing_field("version_major"))?,
            build: self
                .build
                .ok_or_else(|| BuildError::missing_field("build"))?,
        })
    }
}
