pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminUnitsResponse {
    #[serde(default)]
    pub admin_units: Vec<GetAdminUnitsResponseAdminUnitsItem>,
}

impl GetAdminUnitsResponse {
    pub fn builder() -> GetAdminUnitsResponseBuilder {
        <GetAdminUnitsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminUnitsResponseBuilder {
    admin_units: Option<Vec<GetAdminUnitsResponseAdminUnitsItem>>,
}

impl GetAdminUnitsResponseBuilder {
    pub fn admin_units(mut self, value: Vec<GetAdminUnitsResponseAdminUnitsItem>) -> Self {
        self.admin_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAdminUnitsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin_units`](GetAdminUnitsResponseBuilder::admin_units)
    pub fn build(self) -> Result<GetAdminUnitsResponse, BuildError> {
        Ok(GetAdminUnitsResponse {
            admin_units: self
                .admin_units
                .ok_or_else(|| BuildError::missing_field("admin_units"))?,
        })
    }
}
