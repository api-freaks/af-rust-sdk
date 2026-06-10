pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkUserAgentLookupResponseItem {
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
    pub device: BulkUserAgentLookupResponseItemDevice,
    #[serde(default)]
    pub engine: BulkUserAgentLookupResponseItemEngine,
    #[serde(default)]
    pub operating_system: BulkUserAgentLookupResponseItemOperatingSystem,
    /// Error message for this user-agent string, present only if parsing of this entry failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BulkUserAgentLookupResponseItem {
    pub fn builder() -> BulkUserAgentLookupResponseItemBuilder {
        <BulkUserAgentLookupResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkUserAgentLookupResponseItemBuilder {
    user_agent_string: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    device: Option<BulkUserAgentLookupResponseItemDevice>,
    engine: Option<BulkUserAgentLookupResponseItemEngine>,
    operating_system: Option<BulkUserAgentLookupResponseItemOperatingSystem>,
    message: Option<String>,
}

impl BulkUserAgentLookupResponseItemBuilder {
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

    pub fn device(mut self, value: BulkUserAgentLookupResponseItemDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn engine(mut self, value: BulkUserAgentLookupResponseItemEngine) -> Self {
        self.engine = Some(value);
        self
    }

    pub fn operating_system(
        mut self,
        value: BulkUserAgentLookupResponseItemOperatingSystem,
    ) -> Self {
        self.operating_system = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkUserAgentLookupResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_agent_string`](BulkUserAgentLookupResponseItemBuilder::user_agent_string)
    /// - [`name`](BulkUserAgentLookupResponseItemBuilder::name)
    /// - [`r#type`](BulkUserAgentLookupResponseItemBuilder::r#type)
    /// - [`version`](BulkUserAgentLookupResponseItemBuilder::version)
    /// - [`version_major`](BulkUserAgentLookupResponseItemBuilder::version_major)
    /// - [`device`](BulkUserAgentLookupResponseItemBuilder::device)
    /// - [`engine`](BulkUserAgentLookupResponseItemBuilder::engine)
    /// - [`operating_system`](BulkUserAgentLookupResponseItemBuilder::operating_system)
    pub fn build(self) -> Result<BulkUserAgentLookupResponseItem, BuildError> {
        Ok(BulkUserAgentLookupResponseItem {
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
            message: self.message,
        })
    }
}
