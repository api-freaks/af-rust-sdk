pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatValidateResponse {
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_vat_number: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub requested_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub validation: VatValidateResponseValidation,
    #[serde(default)]
    pub company: VatValidateResponseCompany,
}

impl VatValidateResponse {
    pub fn builder() -> VatValidateResponseBuilder {
        <VatValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatValidateResponseBuilder {
    country_code: Option<String>,
    vat_number: Option<String>,
    requester_country_code: Option<String>,
    requester_vat_number: Option<String>,
    requested_at: Option<DateTime<FixedOffset>>,
    validation: Option<VatValidateResponseValidation>,
    company: Option<VatValidateResponseCompany>,
}

impl VatValidateResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn vat_number(mut self, value: impl Into<String>) -> Self {
        self.vat_number = Some(value.into());
        self
    }

    pub fn requester_country_code(mut self, value: impl Into<String>) -> Self {
        self.requester_country_code = Some(value.into());
        self
    }

    pub fn requester_vat_number(mut self, value: impl Into<String>) -> Self {
        self.requester_vat_number = Some(value.into());
        self
    }

    pub fn requested_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.requested_at = Some(value);
        self
    }

    pub fn validation(mut self, value: VatValidateResponseValidation) -> Self {
        self.validation = Some(value);
        self
    }

    pub fn company(mut self, value: VatValidateResponseCompany) -> Self {
        self.company = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatValidateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](VatValidateResponseBuilder::country_code)
    /// - [`vat_number`](VatValidateResponseBuilder::vat_number)
    /// - [`requested_at`](VatValidateResponseBuilder::requested_at)
    /// - [`validation`](VatValidateResponseBuilder::validation)
    /// - [`company`](VatValidateResponseBuilder::company)
    pub fn build(self) -> Result<VatValidateResponse, BuildError> {
        Ok(VatValidateResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            vat_number: self
                .vat_number
                .ok_or_else(|| BuildError::missing_field("vat_number"))?,
            requester_country_code: self.requester_country_code,
            requester_vat_number: self.requester_vat_number,
            requested_at: self
                .requested_at
                .ok_or_else(|| BuildError::missing_field("requested_at"))?,
            validation: self
                .validation
                .ok_or_else(|| BuildError::missing_field("validation"))?,
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
        })
    }
}
