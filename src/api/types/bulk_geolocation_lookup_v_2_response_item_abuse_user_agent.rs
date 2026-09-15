pub use crate::prelude::*;

/// Parsed User-Agent details from the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgent {
    /// Raw User-Agent string used for parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent_string: Option<String>,
    /// Detected user agent product name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User agent category (e.g., Browser, Mobile App, Bot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Full product version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Major version extracted from version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
    /// Device details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice>,
    /// Rendering engine details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine>,
    /// Operating system details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgent {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseUserAgentBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseUserAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentBuilder {
    user_agent_string: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    device: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice>,
    engine: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine>,
    operating_system: Option<BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentBuilder {
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

    pub fn device(
        mut self,
        value: BulkGeolocationLookupV2ResponseItemAbuseUserAgentDevice,
    ) -> Self {
        self.device = Some(value);
        self
    }

    pub fn engine(
        mut self,
        value: BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine,
    ) -> Self {
        self.engine = Some(value);
        self
    }

    pub fn operating_system(
        mut self,
        value: BulkGeolocationLookupV2ResponseItemAbuseUserAgentOperatingSystem,
    ) -> Self {
        self.operating_system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseUserAgent`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuseUserAgent, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseUserAgent {
            user_agent_string: self.user_agent_string,
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
            device: self.device,
            engine: self.engine,
            operating_system: self.operating_system,
        })
    }
}
