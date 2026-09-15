pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default)]
    pub location: AstronomyLookupResponseLocation,
    #[serde(default)]
    pub astronomy: AstronomyLookupResponseAstronomy,
}

impl AstronomyLookupResponse {
    pub fn builder() -> AstronomyLookupResponseBuilder {
        <AstronomyLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupResponseBuilder {
    ip: Option<String>,
    location: Option<AstronomyLookupResponseLocation>,
    astronomy: Option<AstronomyLookupResponseAstronomy>,
}

impl AstronomyLookupResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn location(mut self, value: AstronomyLookupResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn astronomy(mut self, value: AstronomyLookupResponseAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](AstronomyLookupResponseBuilder::location)
    /// - [`astronomy`](AstronomyLookupResponseBuilder::astronomy)
    pub fn build(self) -> Result<AstronomyLookupResponse, BuildError> {
        Ok(AstronomyLookupResponse {
            ip: self.ip,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            astronomy: self
                .astronomy
                .ok_or_else(|| BuildError::missing_field("astronomy"))?,
        })
    }
}
