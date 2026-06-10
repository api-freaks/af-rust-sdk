pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCitiesResponse {
    #[serde(default)]
    pub cities: Vec<GetCitiesResponseCitiesItem>,
}

impl GetCitiesResponse {
    pub fn builder() -> GetCitiesResponseBuilder {
        <GetCitiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCitiesResponseBuilder {
    cities: Option<Vec<GetCitiesResponseCitiesItem>>,
}

impl GetCitiesResponseBuilder {
    pub fn cities(mut self, value: Vec<GetCitiesResponseCitiesItem>) -> Self {
        self.cities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCitiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cities`](GetCitiesResponseBuilder::cities)
    pub fn build(self) -> Result<GetCitiesResponse, BuildError> {
        Ok(GetCitiesResponse {
            cities: self
                .cities
                .ok_or_else(|| BuildError::missing_field("cities"))?,
        })
    }
}
