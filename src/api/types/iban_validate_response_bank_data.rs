pub use crate::prelude::*;

/// Object contains Bank and BIC details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IbanValidateResponseBankData {
    /// SWIFT/BIC code extracted from IBAN for some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// Bank name extracted from IBAN for some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    /// Bank code extracted from IBAN for some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Branch code extracted from IBAN for some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// Country name extracted from IBAN.
    #[serde(default)]
    pub country: String,
    /// Alpha-2 standard country code extracted from IBAN.
    #[serde(default)]
    pub country_iso2: String,
    /// City / branch name for respective bank extracted from IBAN for some countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Bank branch address extracted from IBAN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Account number extracted from IBAN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Indicates whether given IBAN country supports Single Euro Payments Area (SEPA)
    #[serde(default)]
    pub sepa: bool,
}

impl IbanValidateResponseBankData {
    pub fn builder() -> IbanValidateResponseBankDataBuilder {
        <IbanValidateResponseBankDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IbanValidateResponseBankDataBuilder {
    bic: Option<String>,
    bank: Option<String>,
    bank_code: Option<String>,
    branch_code: Option<String>,
    country: Option<String>,
    country_iso2: Option<String>,
    city: Option<String>,
    address: Option<String>,
    account: Option<String>,
    sepa: Option<bool>,
}

impl IbanValidateResponseBankDataBuilder {
    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn bank(mut self, value: impl Into<String>) -> Self {
        self.bank = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn branch_code(mut self, value: impl Into<String>) -> Self {
        self.branch_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn country_iso2(mut self, value: impl Into<String>) -> Self {
        self.country_iso2 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn sepa(mut self, value: bool) -> Self {
        self.sepa = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IbanValidateResponseBankData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](IbanValidateResponseBankDataBuilder::country)
    /// - [`country_iso2`](IbanValidateResponseBankDataBuilder::country_iso2)
    /// - [`sepa`](IbanValidateResponseBankDataBuilder::sepa)
    pub fn build(self) -> Result<IbanValidateResponseBankData, BuildError> {
        Ok(IbanValidateResponseBankData {
            bic: self.bic,
            bank: self.bank,
            bank_code: self.bank_code,
            branch_code: self.branch_code,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            country_iso2: self
                .country_iso2
                .ok_or_else(|| BuildError::missing_field("country_iso2"))?,
            city: self.city,
            address: self.address,
            account: self.account,
            sepa: self.sepa.ok_or_else(|| BuildError::missing_field("sepa"))?,
        })
    }
}
