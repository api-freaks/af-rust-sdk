pub use crate::prelude::*;

/// Rendering engine details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine {
    /// Rendering engine name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Engine category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Full engine version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Major engine version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngineBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngineBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngineBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine`].
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseUserAgentEngine {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
        })
    }
}
