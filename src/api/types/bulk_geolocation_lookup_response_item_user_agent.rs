pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemUserAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<BulkGeolocationLookupResponseItemUserAgentDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<BulkGeolocationLookupResponseItemUserAgentEngine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<BulkGeolocationLookupResponseItemUserAgentOperatingSystem>,
}

impl BulkGeolocationLookupResponseItemUserAgent {
    pub fn builder() -> BulkGeolocationLookupResponseItemUserAgentBuilder {
        <BulkGeolocationLookupResponseItemUserAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemUserAgentBuilder {
    user_agent_string: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
    device: Option<BulkGeolocationLookupResponseItemUserAgentDevice>,
    engine: Option<BulkGeolocationLookupResponseItemUserAgentEngine>,
    operating_system: Option<BulkGeolocationLookupResponseItemUserAgentOperatingSystem>,
}

impl BulkGeolocationLookupResponseItemUserAgentBuilder {
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

    pub fn device(mut self, value: BulkGeolocationLookupResponseItemUserAgentDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn engine(mut self, value: BulkGeolocationLookupResponseItemUserAgentEngine) -> Self {
        self.engine = Some(value);
        self
    }

    pub fn operating_system(
        mut self,
        value: BulkGeolocationLookupResponseItemUserAgentOperatingSystem,
    ) -> Self {
        self.operating_system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemUserAgent`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemUserAgent, BuildError> {
        Ok(BulkGeolocationLookupResponseItemUserAgent {
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
