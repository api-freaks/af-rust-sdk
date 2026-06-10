pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAgentLookupResponseEngine {
    /// Layout engine name or the underlying core that converts the HTML into visual
    #[serde(default)]
    pub name: String,
    /// Layout engine type. Possible values:
    /// 1. Browser
    /// 2. Mobile App
    /// 3. Hacker
    /// 4. Robot
    /// 5. Unknown
    #[serde(default)]
    pub r#type: String,
    /// Layout engine version
    #[serde(default)]
    pub version: String,
    /// Layout engine version major
    #[serde(default)]
    pub version_major: String,
}

impl UserAgentLookupResponseEngine {
    pub fn builder() -> UserAgentLookupResponseEngineBuilder {
        <UserAgentLookupResponseEngineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAgentLookupResponseEngineBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
}

impl UserAgentLookupResponseEngineBuilder {
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

    /// Consumes the builder and constructs a [`UserAgentLookupResponseEngine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UserAgentLookupResponseEngineBuilder::name)
    /// - [`r#type`](UserAgentLookupResponseEngineBuilder::r#type)
    /// - [`version`](UserAgentLookupResponseEngineBuilder::version)
    /// - [`version_major`](UserAgentLookupResponseEngineBuilder::version_major)
    pub fn build(self) -> Result<UserAgentLookupResponseEngine, BuildError> {
        Ok(UserAgentLookupResponseEngine {
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
        })
    }
}
