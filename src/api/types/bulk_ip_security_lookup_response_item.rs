pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkIpSecurityLookupResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkIpSecurityLookupResponseItemSecurity>,
}

impl BulkIpSecurityLookupResponseItem {
    pub fn builder() -> BulkIpSecurityLookupResponseItemBuilder {
        <BulkIpSecurityLookupResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkIpSecurityLookupResponseItemBuilder {
    ip: Option<String>,
    security: Option<BulkIpSecurityLookupResponseItemSecurity>,
}

impl BulkIpSecurityLookupResponseItemBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn security(mut self, value: BulkIpSecurityLookupResponseItemSecurity) -> Self {
        self.security = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkIpSecurityLookupResponseItem`].
    pub fn build(self) -> Result<BulkIpSecurityLookupResponseItem, BuildError> {
        Ok(BulkIpSecurityLookupResponseItem {
            ip: self.ip,
            security: self.security,
        })
    }
}
