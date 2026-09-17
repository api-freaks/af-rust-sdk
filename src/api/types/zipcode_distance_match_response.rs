pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceMatchResponse {
    /// Number of matching ZIP/postal code pairs returned. Zero when no pairs fall within the threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<i64>,
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
    result_count: Option<i64>,
    results: Option<Vec<ZipcodeDistanceMatchResponseResultsItem>>,
}

impl ZipcodeDistanceMatchResponseBuilder {
    pub fn result_count(mut self, value: i64) -> Self {
        self.result_count = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<ZipcodeDistanceMatchResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceMatchResponse`].
    pub fn build(self) -> Result<ZipcodeDistanceMatchResponse, BuildError> {
        Ok(ZipcodeDistanceMatchResponse {
            result_count: self.result_count,
            results: self.results,
        })
    }
}
