pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkZipcodeLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BulkZipcodeLookupResponseResultsItem>>,
}

impl BulkZipcodeLookupResponse {
    pub fn builder() -> BulkZipcodeLookupResponseBuilder {
        <BulkZipcodeLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkZipcodeLookupResponseBuilder {
    results: Option<Vec<BulkZipcodeLookupResponseResultsItem>>,
}

impl BulkZipcodeLookupResponseBuilder {
    pub fn results(mut self, value: Vec<BulkZipcodeLookupResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkZipcodeLookupResponse`].
    pub fn build(self) -> Result<BulkZipcodeLookupResponse, BuildError> {
        Ok(BulkZipcodeLookupResponse {
            results: self.results,
        })
    }
}
