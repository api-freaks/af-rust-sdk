pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCountriesResponse {
    #[serde(default)]
    pub countries: Vec<GetCountriesResponseCountriesItem>,
}

impl GetCountriesResponse {
    pub fn builder() -> GetCountriesResponseBuilder {
        <GetCountriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCountriesResponseBuilder {
    countries: Option<Vec<GetCountriesResponseCountriesItem>>,
}

impl GetCountriesResponseBuilder {
    pub fn countries(mut self, value: Vec<GetCountriesResponseCountriesItem>) -> Self {
        self.countries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCountriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`countries`](GetCountriesResponseBuilder::countries)
    pub fn build(self) -> Result<GetCountriesResponse, BuildError> {
        Ok(GetCountriesResponse {
            countries: self
                .countries
                .ok_or_else(|| BuildError::missing_field("countries"))?,
        })
    }
}
