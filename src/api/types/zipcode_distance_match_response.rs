pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceMatchResponse {
    /// Number of matching ZIP/postal code pairs returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ZipcodeDistanceMatchResponseResultsItem>>,
}

impl ZipcodeDistanceMatchResponse {
    pub fn builder() -> ZipcodeDistanceMatchResponseBuilder {
        <ZipcodeDistanceMatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceMatchResponseBuilder {
    results_count: Option<String>,
    results: Option<Vec<ZipcodeDistanceMatchResponseResultsItem>>,
}

impl ZipcodeDistanceMatchResponseBuilder {
    pub fn results_count(mut self, value: impl Into<String>) -> Self {
        self.results_count = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ZipcodeDistanceMatchResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceMatchResponse`].
    pub fn build(self) -> Result<ZipcodeDistanceMatchResponse, BuildError> {
        Ok(ZipcodeDistanceMatchResponse {
            results_count: self.results_count,
            results: self.results,
        })
    }
}
