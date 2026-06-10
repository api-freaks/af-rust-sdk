pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSubregionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subregions: Option<Vec<String>>,
}

impl GetSubregionsResponse {
    pub fn builder() -> GetSubregionsResponseBuilder {
        <GetSubregionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubregionsResponseBuilder {
    subregions: Option<Vec<String>>,
}

impl GetSubregionsResponseBuilder {
    pub fn subregions(mut self, value: Vec<String>) -> Self {
        self.subregions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSubregionsResponse`].
    pub fn build(self) -> Result<GetSubregionsResponse, BuildError> {
        Ok(GetSubregionsResponse {
            subregions: self.subregions,
        })
    }
}
