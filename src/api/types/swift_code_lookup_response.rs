pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SwiftCodeLookupResponse {
    #[serde(rename = "swiftCode")]
    #[serde(default)]
    pub swift_code: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "bankAddress")]
    #[serde(default)]
    pub bank_address: String,
    #[serde(rename = "bankCode")]
    #[serde(default)]
    pub bank_code: String,
    #[serde(rename = "bankName")]
    #[serde(default)]
    pub bank_name: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub country: String,
    /// Name of the branch or office. Null when not resolvable.
    #[serde(rename = "branchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Type of institution: "FIIN" (financial institution) or "NFIN" (non-financial institution).
    #[serde(rename = "institType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instit_type: Option<String>,
    /// Postal/ZIP code of the bank branch. Null when not resolvable.
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl SwiftCodeLookupResponse {
    pub fn builder() -> SwiftCodeLookupResponseBuilder {
        <SwiftCodeLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SwiftCodeLookupResponseBuilder {
    swift_code: Option<String>,
    country_code: Option<String>,
    bank_address: Option<String>,
    bank_code: Option<String>,
    bank_name: Option<String>,
    city: Option<String>,
    country: Option<String>,
    branch_name: Option<String>,
    instit_type: Option<String>,
    postal_code: Option<String>,
}

impl SwiftCodeLookupResponseBuilder {
    pub fn swift_code(mut self, value: impl Into<String>) -> Self {
        self.swift_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn bank_address(mut self, value: impl Into<String>) -> Self {
        self.bank_address = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn branch_name(mut self, value: impl Into<String>) -> Self {
        self.branch_name = Some(value.into());
        self
    }

    pub fn instit_type(mut self, value: impl Into<String>) -> Self {
        self.instit_type = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SwiftCodeLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`swift_code`](SwiftCodeLookupResponseBuilder::swift_code)
    /// - [`country_code`](SwiftCodeLookupResponseBuilder::country_code)
    /// - [`bank_address`](SwiftCodeLookupResponseBuilder::bank_address)
    /// - [`bank_code`](SwiftCodeLookupResponseBuilder::bank_code)
    /// - [`bank_name`](SwiftCodeLookupResponseBuilder::bank_name)
    /// - [`city`](SwiftCodeLookupResponseBuilder::city)
    /// - [`country`](SwiftCodeLookupResponseBuilder::country)
    pub fn build(self) -> Result<SwiftCodeLookupResponse, BuildError> {
        Ok(SwiftCodeLookupResponse {
            swift_code: self
                .swift_code
                .ok_or_else(|| BuildError::missing_field("swift_code"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            bank_address: self
                .bank_address
                .ok_or_else(|| BuildError::missing_field("bank_address"))?,
            bank_code: self
                .bank_code
                .ok_or_else(|| BuildError::missing_field("bank_code"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            branch_name: self.branch_name,
            instit_type: self.instit_type,
            postal_code: self.postal_code,
        })
    }
}
