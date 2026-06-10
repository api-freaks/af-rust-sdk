pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAgentLookupResponse {
    /// User-agent string that is parsed for browser, device and operating system details.
    #[serde(default)]
    pub user_agent_string: String,
    /// Agent name or the actual browser name that was used.
    #[serde(default)]
    pub name: String,
    /// Agent type or the browser type that was used. Possible values:
    /// 1. Browser
    /// 2. Browser Webview
    /// 3. Mobile App
    /// 4. Robot Mobile
    /// 5. Cloud Application
    /// 6. Email Client
    /// 7. Voice
    /// 8. Special
    /// 9. Testclient
    /// 10. Hacker
    /// 11. Unknown
    #[serde(default)]
    pub r#type: String,
    /// Agent version or the browser version
    #[serde(default)]
    pub version: String,
    /// Agent version major or the browser version major
    #[serde(default)]
    pub version_major: String,
    #[serde(default)]
    pub device: UserAgentLookupResponseDevice,
    #[serde(default)]
    pub engine: UserAgentLookupResponseEngine,
    #[serde(default)]
    pub operating_system: UserAgentLookupResponseOperatingSystem,
}

impl UserAgentLookupResponse {
    pub fn builder() -> UserAgentLookupResponseBuilder {
        <UserAgentLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAgentLookupResponseBuilder {
    user_agent_string: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    device: Option<UserAgentLookupResponseDevice>,
    engine: Option<UserAgentLookupResponseEngine>,
    operating_system: Option<UserAgentLookupResponseOperatingSystem>,
}

impl UserAgentLookupResponseBuilder {
    pub fn user_agent_string(mut self, value: impl Into<String>) -> Self {
        self.user_agent_string = Some(value.into());
        self
    }

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

    pub fn device(mut self, value: UserAgentLookupResponseDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn engine(mut self, value: UserAgentLookupResponseEngine) -> Self {
        self.engine = Some(value);
        self
    }

    pub fn operating_system(mut self, value: UserAgentLookupResponseOperatingSystem) -> Self {
        self.operating_system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserAgentLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_agent_string`](UserAgentLookupResponseBuilder::user_agent_string)
    /// - [`name`](UserAgentLookupResponseBuilder::name)
    /// - [`r#type`](UserAgentLookupResponseBuilder::r#type)
    /// - [`version`](UserAgentLookupResponseBuilder::version)
    /// - [`version_major`](UserAgentLookupResponseBuilder::version_major)
    /// - [`device`](UserAgentLookupResponseBuilder::device)
    /// - [`engine`](UserAgentLookupResponseBuilder::engine)
    /// - [`operating_system`](UserAgentLookupResponseBuilder::operating_system)
    pub fn build(self) -> Result<UserAgentLookupResponse, BuildError> {
        Ok(UserAgentLookupResponse {
            user_agent_string: self
                .user_agent_string
                .ok_or_else(|| BuildError::missing_field("user_agent_string"))?,
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
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            engine: self
                .engine
                .ok_or_else(|| BuildError::missing_field("engine"))?,
            operating_system: self
                .operating_system
                .ok_or_else(|| BuildError::missing_field("operating_system"))?,
        })
    }
}
