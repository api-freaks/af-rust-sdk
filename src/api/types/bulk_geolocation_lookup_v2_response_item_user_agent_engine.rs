pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemUserAgentEngine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemUserAgentEngine {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemUserAgentEngineBuilder {
        <BulkGeolocationLookupV2ResponseItemUserAgentEngineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemUserAgentEngineBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemUserAgentEngineBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemUserAgentEngine`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemUserAgentEngine, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemUserAgentEngine {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
        })
    }
}
