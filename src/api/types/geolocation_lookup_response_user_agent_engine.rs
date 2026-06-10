pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseUserAgentEngine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
}

impl GeolocationLookupResponseUserAgentEngine {
    pub fn builder() -> GeolocationLookupResponseUserAgentEngineBuilder {
        <GeolocationLookupResponseUserAgentEngineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseUserAgentEngineBuilder {
    name: Option<String>,
    r#type: Option<String>,
    version: Option<String>,
    version_major: Option<String>,
}

impl GeolocationLookupResponseUserAgentEngineBuilder {
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

    /// Consumes the builder and constructs a [`GeolocationLookupResponseUserAgentEngine`].
    pub fn build(self) -> Result<GeolocationLookupResponseUserAgentEngine, BuildError> {
        Ok(GeolocationLookupResponseUserAgentEngine {
            name: self.name,
            r#type: self.r#type,
            version: self.version,
            version_major: self.version_major,
        })
    }
}
