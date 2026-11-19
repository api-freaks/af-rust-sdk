pub use crate::prelude::*;

/// Countries and organizations that have flags available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSupportedFlagsResponse {
    /// Countries with a flag, each with its name and ISO 3166-1 alpha-2/alpha-3 codes.
    #[serde(default)]
    pub supported_countries: Vec<GetSupportedFlagsResponseSupportedCountriesItem>,
    /// Identifiers of supported organizations, usable as the flag lookup key.
    #[serde(default)]
    pub supported_organizations: Vec<String>,
}

impl GetSupportedFlagsResponse {
    pub fn builder() -> GetSupportedFlagsResponseBuilder {
        <GetSupportedFlagsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSupportedFlagsResponseBuilder {
    supported_countries: Option<Vec<GetSupportedFlagsResponseSupportedCountriesItem>>,
    supported_organizations: Option<Vec<String>>,
}

impl GetSupportedFlagsResponseBuilder {
    pub fn supported_countries(
        mut self,
        value: Vec<GetSupportedFlagsResponseSupportedCountriesItem>,
    ) -> Self {
        self.supported_countries = Some(value);
        self
    }

    pub fn supported_organizations(mut self, value: Vec<String>) -> Self {
        self.supported_organizations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSupportedFlagsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supported_countries`](GetSupportedFlagsResponseBuilder::supported_countries)
    /// - [`supported_organizations`](GetSupportedFlagsResponseBuilder::supported_organizations)
    pub fn build(self) -> Result<GetSupportedFlagsResponse, BuildError> {
        Ok(GetSupportedFlagsResponse {
            supported_countries: self
                .supported_countries
                .ok_or_else(|| BuildError::missing_field("supported_countries"))?,
            supported_organizations: self
                .supported_organizations
                .ok_or_else(|| BuildError::missing_field("supported_organizations"))?,
        })
    }
}
