pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeDistanceResponse {
    /// Number of distance results returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<i64>,
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
    result_count: Option<i64>,
    results: Option<Vec<ZipcodeDistanceResponseResultsItem>>,
}

impl ZipcodeDistanceResponseBuilder {
    pub fn result_count(mut self, value: i64) -> Self {
        self.result_count = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<ZipcodeDistanceResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeDistanceResponse`].
    pub fn build(self) -> Result<ZipcodeDistanceResponse, BuildError> {
        Ok(ZipcodeDistanceResponse {
            result_count: self.result_count,
            results: self.results,
        })
    }
}
