pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IbanValidateResponse {
    /// Determines whether IBAN passes all validation checks
    #[serde(default)]
    pub valid: bool,
    /// The IBAN number provided by the user.
    #[serde(default)]
    pub iban: String,
    /// Object contains IBAN validation details.
    #[serde(default)]
    pub validation: IbanValidateResponseValidation,
    /// Object contains Bank and BIC details.
    #[serde(default)]
    pub bank_data: IbanValidateResponseBankData,
}

impl IbanValidateResponse {
    pub fn builder() -> IbanValidateResponseBuilder {
        <IbanValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IbanValidateResponseBuilder {
    valid: Option<bool>,
    iban: Option<String>,
    validation: Option<IbanValidateResponseValidation>,
    bank_data: Option<IbanValidateResponseBankData>,
}

impl IbanValidateResponseBuilder {
    pub fn valid(mut self, value: bool) -> Self {
        self.valid = Some(value);
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn validation(mut self, value: IbanValidateResponseValidation) -> Self {
        self.validation = Some(value);
        self
    }

    pub fn bank_data(mut self, value: IbanValidateResponseBankData) -> Self {
        self.bank_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IbanValidateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`valid`](IbanValidateResponseBuilder::valid)
    /// - [`iban`](IbanValidateResponseBuilder::iban)
    /// - [`validation`](IbanValidateResponseBuilder::validation)
    /// - [`bank_data`](IbanValidateResponseBuilder::bank_data)
    pub fn build(self) -> Result<IbanValidateResponse, BuildError> {
        Ok(IbanValidateResponse {
            valid: self
                .valid
                .ok_or_else(|| BuildError::missing_field("valid"))?,
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
            validation: self
                .validation
                .ok_or_else(|| BuildError::missing_field("validation"))?,
            bank_data: self
                .bank_data
                .ok_or_else(|| BuildError::missing_field("bank_data"))?,
        })
    }
}
