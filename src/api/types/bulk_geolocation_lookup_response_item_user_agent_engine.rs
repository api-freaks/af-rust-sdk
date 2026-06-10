pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemUserAgentEngine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
}

impl BulkGeolocationLookupResponseItemUserAgentEngine {
    pub fn builder() -> BulkGeolocationLookupResponseItemUserAgentEngineBuilder {
        <BulkGeolocationLookupResponseItemUserAgentEngineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemUserAgentEngineBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
}

impl BulkGeolocationLookupResponseItemUserAgentEngineBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemUserAgentEngine`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemUserAgentEngine, BuildError> {
        Ok(BulkGeolocationLookupResponseItemUserAgentEngine {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
        })
    }
}
