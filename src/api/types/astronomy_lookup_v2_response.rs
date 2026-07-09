pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupV2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AstronomyLookupV2ResponseLocation>,
    #[serde(default)]
    pub astronomy: AstronomyLookupV2ResponseAstronomy,
}

impl AstronomyLookupV2Response {
    pub fn builder() -> AstronomyLookupV2ResponseBuilder {
        <AstronomyLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupV2ResponseBuilder {
    ip: Option<String>,
    location: Option<AstronomyLookupV2ResponseLocation>,
    astronomy: Option<AstronomyLookupV2ResponseAstronomy>,
}

impl AstronomyLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn location(mut self, value: AstronomyLookupV2ResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn astronomy(mut self, value: AstronomyLookupV2ResponseAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`astronomy`](AstronomyLookupV2ResponseBuilder::astronomy)
    pub fn build(self) -> Result<AstronomyLookupV2Response, BuildError> {
        Ok(AstronomyLookupV2Response {
            ip: self.ip,
            location: self.location,
            astronomy: self
                .astronomy
                .ok_or_else(|| BuildError::missing_field("astronomy"))?,
        })
    }
}
