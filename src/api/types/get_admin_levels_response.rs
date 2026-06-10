pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminLevelsResponse {
    #[serde(default)]
    pub admin_levels: Vec<String>,
}

impl GetAdminLevelsResponse {
    pub fn builder() -> GetAdminLevelsResponseBuilder {
        <GetAdminLevelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminLevelsResponseBuilder {
    admin_levels: Option<Vec<String>>,
}

impl GetAdminLevelsResponseBuilder {
    pub fn admin_levels(mut self, value: Vec<String>) -> Self {
        self.admin_levels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAdminLevelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin_levels`](GetAdminLevelsResponseBuilder::admin_levels)
    pub fn build(self) -> Result<GetAdminLevelsResponse, BuildError> {
        Ok(GetAdminLevelsResponse {
            admin_levels: self
                .admin_levels
                .ok_or_else(|| BuildError::missing_field("admin_levels"))?,
        })
    }
}
