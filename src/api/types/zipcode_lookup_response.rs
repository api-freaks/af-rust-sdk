pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ZipcodeLookupResponseResultsItem>>,
}

impl ZipcodeLookupResponse {
    pub fn builder() -> ZipcodeLookupResponseBuilder {
        <ZipcodeLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeLookupResponseBuilder {
    results: Option<Vec<ZipcodeLookupResponseResultsItem>>,
}

impl ZipcodeLookupResponseBuilder {
    pub fn results(mut self, value: Vec<ZipcodeLookupResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeLookupResponse`].
    pub fn build(self) -> Result<ZipcodeLookupResponse, BuildError> {
        Ok(ZipcodeLookupResponse {
            results: self.results,
        })
    }
}
