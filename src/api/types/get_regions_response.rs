pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRegionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

impl GetRegionsResponse {
    pub fn builder() -> GetRegionsResponseBuilder {
        <GetRegionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRegionsResponseBuilder {
    regions: Option<Vec<String>>,
}

impl GetRegionsResponseBuilder {
    pub fn regions(mut self, value: Vec<String>) -> Self {
        self.regions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRegionsResponse`].
    pub fn build(self) -> Result<GetRegionsResponse, BuildError> {
        Ok(GetRegionsResponse {
            regions: self.regions,
        })
    }
}
