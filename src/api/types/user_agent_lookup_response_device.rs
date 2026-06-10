pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAgentLookupResponseDevice {
    /// Device name or the hardware that was used
    #[serde(default)]
    pub name: String,
    /// Device type or the hardware type. Possible values:
    /// 1. Desktop
    /// 2. Mobile
    /// 3. Tablet
    /// 4. Phone
    /// 5. Watch
    /// 6. Virtual Reality
    /// 7. eReader
    /// 8. Set-top box
    /// 9. TV
    /// 10. Game Console
    /// 11. Handheld Game Console
    /// 12. Voice
    /// 13. Robot
    /// 14. Robot Mobile
    /// 15. Robot Imitator
    /// 16. Hacker
    /// 17. Anonymized
    /// 18. Unknown
    #[serde(default)]
    pub r#type: String,
    /// Device brand or the hardware brand name
    #[serde(default)]
    pub brand: String,
    /// Device's CPU model or machine CPU
    #[serde(default)]
    pub cpu: String,
}

impl UserAgentLookupResponseDevice {
    pub fn builder() -> UserAgentLookupResponseDeviceBuilder {
        <UserAgentLookupResponseDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAgentLookupResponseDeviceBuilder {
    name: Option<String>,
    r#type: Option<String>,
    brand: Option<String>,
    cpu: Option<String>,
}

impl UserAgentLookupResponseDeviceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn cpu(mut self, value: impl Into<String>) -> Self {
        self.cpu = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserAgentLookupResponseDevice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UserAgentLookupResponseDeviceBuilder::name)
    /// - [`r#type`](UserAgentLookupResponseDeviceBuilder::r#type)
    /// - [`brand`](UserAgentLookupResponseDeviceBuilder::brand)
    /// - [`cpu`](UserAgentLookupResponseDeviceBuilder::cpu)
    pub fn build(self) -> Result<UserAgentLookupResponseDevice, BuildError> {
        Ok(UserAgentLookupResponseDevice {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            brand: self
                .brand
                .ok_or_else(|| BuildError::missing_field("brand"))?,
            cpu: self.cpu.ok_or_else(|| BuildError::missing_field("cpu"))?,
        })
    }
}
