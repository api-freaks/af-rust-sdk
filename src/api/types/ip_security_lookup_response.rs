pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IpSecurityLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<IpSecurityLookupResponseSecurity>,
}

impl IpSecurityLookupResponse {
    pub fn builder() -> IpSecurityLookupResponseBuilder {
        <IpSecurityLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpSecurityLookupResponseBuilder {
    ip: Option<String>,
    security: Option<IpSecurityLookupResponseSecurity>,
}

impl IpSecurityLookupResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn security(mut self, value: IpSecurityLookupResponseSecurity) -> Self {
        self.security = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IpSecurityLookupResponse`].
    pub fn build(self) -> Result<IpSecurityLookupResponse, BuildError> {
        Ok(IpSecurityLookupResponse {
            ip: self.ip,
            security: self.security,
        })
    }
}
