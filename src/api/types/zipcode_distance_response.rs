pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceResponse {
    /// Number of distance results returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ZipcodeDistanceResponseResultsItem>>,
}

impl ZipcodeDistanceResponse {
    pub fn builder() -> ZipcodeDistanceResponseBuilder {
        <ZipcodeDistanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeDistanceResponseBuilder {
    results_count: Option<String>,
    results: Option<Vec<ZipcodeDistanceResponseResultsItem>>,
}

impl ZipcodeDistanceResponseBuilder {
    pub fn results_count(mut self, value: impl Into<String>) -> Self {
        self.results_count = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ZipcodeDistanceResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceResponse`].
    pub fn build(self) -> Result<ZipcodeDistanceResponse, BuildError> {
        Ok(ZipcodeDistanceResponse {
            results_count: self.results_count,
            results: self.results,
        })
    }
}
